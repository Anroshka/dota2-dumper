// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-11-07 09:16:29.927261200 UTC

namespace Dota2Dumper.Schemas {
    // Module: worldrenderer.dll
    // Class count: 14
    // Enum count: 0
    public static class WorldrendererDll {
        // Parent: None
        // Field count: 3
        public static class CEntityInstance {
            public const nint m_iszPrivateVScripts = 0x8; // CUtlSymbolLarge
            public const nint m_pEntity = 0x10; // CEntityIdentity*
            public const nint m_CScriptComponent = 0x28; // CScriptComponent*
        }
        // Parent: None
        // Field count: 0
        public static class CEntityComponent {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CScriptComponent {
            public const nint m_scriptClassName = 0x30; // CUtlSymbolLarge
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CEntityIdentity {
            public const nint m_nameStringTableIndex = 0x14; // int32
            public const nint m_name = 0x18; // CUtlSymbolLarge
            public const nint m_designerName = 0x20; // CUtlSymbolLarge
            public const nint m_flags = 0x30; // uint32
            public const nint m_worldGroupId = 0x38; // WorldGroupId_t
            public const nint m_fDataObjectTypes = 0x3C; // uint32
            public const nint m_PathIndex = 0x40; // ChangeAccessorFieldPathIndex_t
            public const nint m_pAttributes = 0x48; // CEntityAttributeTable*
            public const nint m_pPrev = 0x50; // CEntityIdentity*
            public const nint m_pNext = 0x58; // CEntityIdentity*
            public const nint m_pPrevByClass = 0x60; // CEntityIdentity*
            public const nint m_pNextByClass = 0x68; // CEntityIdentity*
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class NodeData_t {
            public const nint m_vOrigin = 0x0; // Vector
            public const nint m_vMinBounds = 0xC; // Vector
            public const nint m_vMaxBounds = 0x18; // Vector
            public const nint m_worldNodePrefix = 0x28; // CUtlString
        }
        // Parent: None
        // Field count: 0
        public static class VMapResourceData_t {
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class WorldBuilderParams_t {
            public const nint m_flMinDrawVolumeSize = 0x0; // float32
            public const nint m_bBuildBakedLighting = 0x4; // bool
            public const nint m_bAggregateInstanceStreams = 0x5; // bool
            public const nint m_bakedLightingInfo = 0x8; // BakedLightingInfo_t
            public const nint m_nCompileTimestamp = 0x50; // uint64
            public const nint m_nCompileFingerprint = 0x58; // uint64
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PermEntityLumpData_t {
            public const nint m_name = 0x8; // CUtlString
            public const nint m_childLumps = 0x10; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
            public const nint m_entityKeyValues = 0x28; // CUtlLeanVector<EntityKeyValueData_t>
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class EntityIOConnectionData_t {
            public const nint m_outputName = 0x0; // CUtlString
            public const nint m_targetType = 0x8; // uint32
            public const nint m_targetName = 0x10; // CUtlString
            public const nint m_inputName = 0x18; // CUtlString
            public const nint m_overrideParam = 0x20; // CUtlString
            public const nint m_flDelay = 0x28; // float32
            public const nint m_nTimesToFire = 0x2C; // int32
            public const nint m_paramMap = 0x30; // KeyValues3
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class VoxelVisBlockOffset_t {
            public const nint m_nOffset = 0x0; // uint32
            public const nint m_nElementCount = 0x4; // uint32
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MResourceTypeForInfoType
        public static class InfoForResourceTypeVMapResourceData_t {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class World_t {
            public const nint m_builderParams = 0x0; // WorldBuilderParams_t
            public const nint m_worldNodes = 0x60; // CUtlVector<NodeData_t>
            public const nint m_worldLightingInfo = 0x78; // BakedLightingInfo_t
            public const nint m_entityLumps = 0xC0; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class EntityKeyValueData_t {
            public const nint m_connections = 0x8; // CUtlVector<EntityIOConnectionData_t>
            public const nint m_keyValuesData = 0x20; // CUtlBinaryBlock
        }
        // Parent: None
        // Field count: 13
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CVoxelVisibility {
            public const nint m_nBaseClusterCount = 0x40; // uint32
            public const nint m_nPVSBytesPerCluster = 0x44; // uint32
            public const nint m_vMinBounds = 0x48; // Vector
            public const nint m_vMaxBounds = 0x54; // Vector
            public const nint m_flGridSize = 0x60; // float32
            public const nint m_nSkyVisibilityCluster = 0x64; // uint32
            public const nint m_nSunVisibilityCluster = 0x68; // uint32
            public const nint m_NodeBlock = 0x6C; // VoxelVisBlockOffset_t
            public const nint m_RegionBlock = 0x74; // VoxelVisBlockOffset_t
            public const nint m_EnclosedClusterListBlock = 0x7C; // VoxelVisBlockOffset_t
            public const nint m_EnclosedClustersBlock = 0x84; // VoxelVisBlockOffset_t
            public const nint m_MasksBlock = 0x8C; // VoxelVisBlockOffset_t
            public const nint m_nVisBlocks = 0x94; // VoxelVisBlockOffset_t
        }
    }
}
