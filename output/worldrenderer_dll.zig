// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-11-07 09:16:29.927261200 UTC

pub const dota2_dumper = struct {
    pub const schemas = struct {
        // Module: worldrenderer.dll
        // Class count: 14
        // Enum count: 0
        pub const worldrenderer_dll = struct {
            // Parent: None
            // Field count: 3
            pub const CEntityInstance = struct {
                pub const m_iszPrivateVScripts: usize = 0x8; // CUtlSymbolLarge
                pub const m_pEntity: usize = 0x10; // CEntityIdentity*
                pub const m_CScriptComponent: usize = 0x28; // CScriptComponent*
            };
            // Parent: None
            // Field count: 0
            pub const CEntityComponent = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CScriptComponent = struct {
                pub const m_scriptClassName: usize = 0x30; // CUtlSymbolLarge
            };
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CEntityIdentity = struct {
                pub const m_nameStringTableIndex: usize = 0x14; // int32
                pub const m_name: usize = 0x18; // CUtlSymbolLarge
                pub const m_designerName: usize = 0x20; // CUtlSymbolLarge
                pub const m_flags: usize = 0x30; // uint32
                pub const m_worldGroupId: usize = 0x38; // WorldGroupId_t
                pub const m_fDataObjectTypes: usize = 0x3C; // uint32
                pub const m_PathIndex: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const m_pAttributes: usize = 0x48; // CEntityAttributeTable*
                pub const m_pPrev: usize = 0x50; // CEntityIdentity*
                pub const m_pNext: usize = 0x58; // CEntityIdentity*
                pub const m_pPrevByClass: usize = 0x60; // CEntityIdentity*
                pub const m_pNextByClass: usize = 0x68; // CEntityIdentity*
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const NodeData_t = struct {
                pub const m_vOrigin: usize = 0x0; // Vector
                pub const m_vMinBounds: usize = 0xC; // Vector
                pub const m_vMaxBounds: usize = 0x18; // Vector
                pub const m_worldNodePrefix: usize = 0x28; // CUtlString
            };
            // Parent: None
            // Field count: 0
            pub const VMapResourceData_t = struct {
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const WorldBuilderParams_t = struct {
                pub const m_flMinDrawVolumeSize: usize = 0x0; // float32
                pub const m_bBuildBakedLighting: usize = 0x4; // bool
                pub const m_bAggregateInstanceStreams: usize = 0x5; // bool
                pub const m_bakedLightingInfo: usize = 0x8; // BakedLightingInfo_t
                pub const m_nCompileTimestamp: usize = 0x50; // uint64
                pub const m_nCompileFingerprint: usize = 0x58; // uint64
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PermEntityLumpData_t = struct {
                pub const m_name: usize = 0x8; // CUtlString
                pub const m_childLumps: usize = 0x10; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
                pub const m_entityKeyValues: usize = 0x28; // CUtlLeanVector<EntityKeyValueData_t>
            };
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const EntityIOConnectionData_t = struct {
                pub const m_outputName: usize = 0x0; // CUtlString
                pub const m_targetType: usize = 0x8; // uint32
                pub const m_targetName: usize = 0x10; // CUtlString
                pub const m_inputName: usize = 0x18; // CUtlString
                pub const m_overrideParam: usize = 0x20; // CUtlString
                pub const m_flDelay: usize = 0x28; // float32
                pub const m_nTimesToFire: usize = 0x2C; // int32
                pub const m_paramMap: usize = 0x30; // KeyValues3
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const VoxelVisBlockOffset_t = struct {
                pub const m_nOffset: usize = 0x0; // uint32
                pub const m_nElementCount: usize = 0x4; // uint32
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub const InfoForResourceTypeVMapResourceData_t = struct {
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const World_t = struct {
                pub const m_builderParams: usize = 0x0; // WorldBuilderParams_t
                pub const m_worldNodes: usize = 0x60; // CUtlVector<NodeData_t>
                pub const m_worldLightingInfo: usize = 0x78; // BakedLightingInfo_t
                pub const m_entityLumps: usize = 0xC0; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const EntityKeyValueData_t = struct {
                pub const m_connections: usize = 0x8; // CUtlVector<EntityIOConnectionData_t>
                pub const m_keyValuesData: usize = 0x20; // CUtlBinaryBlock
            };
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CVoxelVisibility = struct {
                pub const m_nBaseClusterCount: usize = 0x40; // uint32
                pub const m_nPVSBytesPerCluster: usize = 0x44; // uint32
                pub const m_vMinBounds: usize = 0x48; // Vector
                pub const m_vMaxBounds: usize = 0x54; // Vector
                pub const m_flGridSize: usize = 0x60; // float32
                pub const m_nSkyVisibilityCluster: usize = 0x64; // uint32
                pub const m_nSunVisibilityCluster: usize = 0x68; // uint32
                pub const m_NodeBlock: usize = 0x6C; // VoxelVisBlockOffset_t
                pub const m_RegionBlock: usize = 0x74; // VoxelVisBlockOffset_t
                pub const m_EnclosedClusterListBlock: usize = 0x7C; // VoxelVisBlockOffset_t
                pub const m_EnclosedClustersBlock: usize = 0x84; // VoxelVisBlockOffset_t
                pub const m_MasksBlock: usize = 0x8C; // VoxelVisBlockOffset_t
                pub const m_nVisBlocks: usize = 0x94; // VoxelVisBlockOffset_t
            };
        };
    };
};
