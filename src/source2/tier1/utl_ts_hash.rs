use std::collections::HashSet;

use memflow::prelude::v1::*;

use super::UtlMemoryPool;

#[repr(C)]
pub struct UtlTsHashPoolEntry<D> {
    pub next: Pointer64<UtlTsHashPoolEntry<D>>,
    pad_0: [u8; 0x8],
    pub data: Pointer64<D>,
    pad_1: [u8; 0x18],
}

unsafe impl<D: 'static> Pod for UtlTsHashPoolEntry<D> {}

#[repr(C)]
pub struct UtlTsHashFixedData<D, K> {
    pub ui_key: K,                                 // 0x0000
    pub next: Pointer64<UtlTsHashFixedData<D, K>>, // 0x0008
    pub data: Pointer64<D>,                        // 0x0010
}

unsafe impl<D: 'static, K: Pod> Pod for UtlTsHashFixedData<D, K> {}

#[repr(C)]
pub struct UtlTsHashBucket<D, K> {
    pub add_lock: usize,                                        // 0x0000
    pub first: Pointer64<UtlTsHashFixedData<D, K>>,             // 0x0008
    pub first_uncommitted: Pointer64<UtlTsHashFixedData<D, K>>, // 0x0010
}

#[repr(C)]
pub struct UtlTsHash<D, const C: usize = 256, K = u64> {
    pub entry_mem: UtlMemoryPool,            // 0x0000
    pub buckets: [UtlTsHashBucket<D, K>; C], // 0x0060
    pub needs_commit: u8,                    // 0x1860
    pad_0: [u8; 0x3],                        // 0x1861
    pub contention_check: i32,               // 0x1864
    pad_1: [u8; 0x8],                        // 0x1868
}

impl<D: Pod, const C: usize, K: Pod> UtlTsHash<D, C, K> {
    pub fn registered(&self, mem: &mut impl MemoryView) -> Result<Vec<Pointer64<D>>> {
        let mut elements = Vec::new();
        let mut seen_nodes = HashSet::new();
        let mut seen_data = HashSet::new();
        // Include both bucket chains, including committed registrations.
        for bucket in &self.buckets {
            for mut node_ptr in [bucket.first, bucket.first_uncommitted] {
                while !node_ptr.is_null() && seen_nodes.insert(node_ptr.address().to_umem()) {
                    let node = mem.read_ptr(node_ptr).data()?;
                    if !node.data.is_null() && seen_data.insert(node.data.address().to_umem()) {
                        elements.push(node.data);
                    }
                    node_ptr = node.next;
                }
            }
        }
        Ok(elements)
    }

    pub fn pool_candidates(&self, mem: &mut impl MemoryView) -> Result<Vec<Pointer64<D>>> {
        let mut elements = Vec::new();
        let mut seen_data = HashSet::new();
        // Free slots may still reference bindings promoted to the global registry.
        // The caller checks membership in that registry before using these pointers.
        let limit = usize::try_from(self.entry_mem.peak_allocated).unwrap_or(0);
        let mut seen_pool = HashSet::new();
        let mut entry_ptr = Pointer64::<UtlTsHashPoolEntry<D>>::from(
            self.entry_mem.free_blocks.head.next.address(),
        );
        while !entry_ptr.is_null()
            && seen_pool.len() < limit
            && seen_pool.insert(entry_ptr.address().to_umem())
        {
            let entry = mem.read_ptr(entry_ptr).data()?;
            if !entry.data.is_null() && seen_data.insert(entry.data.address().to_umem()) {
                elements.push(entry.data);
            }
            entry_ptr = entry.next;
        }
        Ok(elements)
    }
}

unsafe impl<D: 'static, const C: usize, K: 'static> Pod for UtlTsHash<D, C, K> {}

#[cfg(test)]
mod tests {
    use super::*;
    use memflow::connector::FileIoMemory;
    use std::io::Cursor;

    #[test]
    fn enumerates_both_chains_and_pool_candidates_without_cycles() {
        let mut bytes = vec![0u8; 0x400];
        for (at, value) in [
            (0x108, 0x100u64),
            (0x110, 0x1000),
            (0x148, 0x100),
            (0x150, 0x2000),
            (0x190, 0xBAD),
        ] {
            bytes[at..at + 8].copy_from_slice(&value.to_le_bytes());
        }
        let mut mem = FileIoMemory::with_size(Cursor::new(bytes), 0x400)
            .unwrap()
            .into_phys_view();
        // All fields are plain memory representations, including pointer addresses.
        let mut hash: UtlTsHash<u64, 1> = unsafe { std::mem::zeroed() };
        hash.buckets[0].first = Pointer64::from(Address::from(0x100u64));
        hash.buckets[0].first_uncommitted = Pointer64::from(Address::from(0x140u64));
        hash.entry_mem.free_blocks.head.next = Pointer64::from(Address::from(0x180u64));
        hash.entry_mem.blocks_allocated = 1;
        hash.entry_mem.peak_allocated = 1;
        let mut result: Vec<_> = hash
            .registered(&mut mem)
            .unwrap()
            .into_iter()
            .chain(hash.pool_candidates(&mut mem).unwrap())
            .map(|ptr| ptr.address().to_umem())
            .collect();
        result.sort_unstable();
        assert_eq!(result, [0xBAD, 0x1000, 0x2000]);
    }
}
