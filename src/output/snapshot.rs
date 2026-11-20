use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use tempfile::{Builder, TempDir};

use super::BANNER;

pub struct SnapshotDirectory {
    target: PathBuf,
    stage: TempDir,
    previous: PathBuf,
    existing: bool,
    _lock: File,
}

impl SnapshotDirectory {
    pub fn new(path: &Path) -> Result<Self> {
        let path = std::path::absolute(path)?;
        let name = path.file_name().context("output needs a directory name")?;
        let parent = path.parent().context("output directory has no parent")?;
        fs::create_dir_all(parent)?;
        let parent = fs::canonicalize(parent)?;
        let target = parent.join(name);

        let mut lock_name = std::ffi::OsString::from(".");
        lock_name.push(name);
        lock_name.push(".dota2-dumper.lock");
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(parent.join(lock_name))?;
        lock.try_lock()
            .with_context(|| format!("cannot lock output {}", target.display()))?;

        let mut previous_name = std::ffi::OsString::from(".");
        previous_name.push(name);
        previous_name.push(".dota2-dumper.previous");
        let previous = parent.join(previous_name);
        let mut existing = checked_snapshot(&target)?;
        if checked_snapshot(&previous)? {
            if existing {
                fs::remove_dir_all(&previous).context("removing the completed snapshot backup")?;
            } else {
                fs::rename(&previous, &target).context("recovering the previous snapshot")?;
                existing = true;
            }
        }

        let stage = Builder::new()
            .prefix(".dota2-dumper-stage-")
            .tempdir_in(parent)?;
        Ok(Self {
            target,
            stage,
            previous,
            existing,
            _lock: lock,
        })
    }

    pub fn path(&self) -> &Path {
        self.stage.path()
    }

    pub fn publish(self) -> Result<()> {
        if !self.existing {
            fs::rename(self.stage.path(), &self.target).context("publishing the new snapshot")?;
            return Ok(());
        }

        fs::rename(&self.target, &self.previous)
            .context("moving the previous snapshot to backup")?;
        if let Err(publish_error) = fs::rename(self.stage.path(), &self.target) {
            if let Err(rollback_error) = fs::rename(&self.previous, &self.target) {
                bail!(
                    "publishing failed ({publish_error}); restoring failed ({rollback_error}); previous snapshot is at {}",
                    self.previous.display()
                );
            }
            return Err(publish_error).context("publishing the new snapshot");
        }
        if let Err(error) = fs::remove_dir_all(&self.previous) {
            log::warn!(
                "could not remove previous snapshot at {}: {error}",
                self.previous.display()
            );
        }
        Ok(())
    }
}

fn checked_snapshot(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            ensure!(
                metadata.file_type().is_dir(),
                "output is not a plain directory: {}",
                path.display()
            );
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                ensure!(
                    entry.file_type()?.is_file() && is_generated_file(&entry.path())?,
                    "output contains an unrelated entry: {}",
                    entry.path().display()
                );
            }
            Ok(true)
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err.into()),
    }
}

fn is_generated_file(path: &Path) -> Result<bool> {
    let extension = path
        .extension()
        .and_then(|part| part.to_str())
        .unwrap_or("");
    if matches!(extension, "cs" | "hpp" | "rs" | "zig") {
        let mut line = String::new();
        BufReader::new(File::open(path)?).read_line(&mut line)?;
        return Ok(line.trim_end() == BANNER);
    }
    if extension != "json" {
        return Ok(false);
    }
    let Some(stem) = path.file_stem().and_then(|part| part.to_str()) else {
        return Ok(false);
    };
    let file = File::open(path)?;
    let Ok(json) = serde_json::from_reader::<_, serde_json::Value>(BufReader::new(file)) else {
        return Ok(false);
    };
    let Some(object) = json.as_object() else {
        return Ok(false);
    };
    let is_offsets = |value: &serde_json::Value| {
        value
            .as_object()
            .is_some_and(|map| map.values().all(|value| value.is_u64()))
    };
    Ok(match stem {
        "info" => {
            object.get("timestamp").is_some_and(|v| v.is_string())
                && object.contains_key("build_number")
        }
        "buttons" => object.len() == 1 && object.get("client.dll").is_some_and(is_offsets),
        "interfaces" | "offsets" => {
            !object.is_empty()
                && object
                    .iter()
                    .all(|(module, values)| module.ends_with(".dll") && is_offsets(values))
        }
        _ => {
            object.len() == 1
                && object.iter().all(|(module, schema)| {
                    super::slugify(module) == stem
                        && schema.get("classes").is_some_and(|value| value.is_object())
                        && schema.get("enums").is_some_and(|value| value.is_object())
                })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_previous_snapshot_when_publication_fails() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let target = directory.path().join("output");
        fs::create_dir(&target)?;
        fs::write(target.join("buttons.rs"), BANNER)?;

        let snapshot = SnapshotDirectory::new(&target)?;
        fs::remove_dir(snapshot.path())?;
        assert!(snapshot.publish().is_err());
        assert!(target.join("buttons.rs").exists());

        let snapshot = SnapshotDirectory::new(&target)?;
        fs::write(
            snapshot.path().join("buttons.json"),
            r#"{"client.dll": {}}"#,
        )?;
        snapshot.publish()?;
        assert!(target.join("buttons.json").exists());
        assert!(!target.join("buttons.rs").exists());
        Ok(())
    }

    #[test]
    fn refuses_to_replace_unrelated_files() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let target = directory.path().join("output");
        fs::create_dir(&target)?;
        fs::write(target.join("notes.txt"), "keep")?;
        assert!(SnapshotDirectory::new(&target).is_err());
        assert_eq!(fs::read_to_string(target.join("notes.txt"))?, "keep");
        Ok(())
    }
}
