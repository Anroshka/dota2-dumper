// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-11-07 09:16:29.927261200 UTC

#pragma once

#include <cstddef>
#include <cstdint>

namespace dota2_dumper {
    namespace schemas {
        // Module: worldrenderer.dll
        // Class count: 14
        // Enum count: 0
        namespace worldrenderer_dll {
            // Parent: None
            // Field count: 3
            namespace CEntityInstance {
                constexpr std::ptrdiff_t m_iszPrivateVScripts = 0x8; // CUtlSymbolLarge
                constexpr std::ptrdiff_t m_pEntity = 0x10; // CEntityIdentity*
                constexpr std::ptrdiff_t m_CScriptComponent = 0x28; // CScriptComponent*
            }
            // Parent: None
            // Field count: 0
            namespace CEntityComponent {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CScriptComponent {
                constexpr std::ptrdiff_t m_scriptClassName = 0x30; // CUtlSymbolLarge
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CEntityIdentity {
                constexpr std::ptrdiff_t m_nameStringTableIndex = 0x14; // int32
                constexpr std::ptrdiff_t m_name = 0x18; // CUtlSymbolLarge
                constexpr std::ptrdiff_t m_designerName = 0x20; // CUtlSymbolLarge
                constexpr std::ptrdiff_t m_flags = 0x30; // uint32
                constexpr std::ptrdiff_t m_worldGroupId = 0x38; // WorldGroupId_t
                constexpr std::ptrdiff_t m_fDataObjectTypes = 0x3C; // uint32
                constexpr std::ptrdiff_t m_PathIndex = 0x40; // ChangeAccessorFieldPathIndex_t
                constexpr std::ptrdiff_t m_pAttributes = 0x48; // CEntityAttributeTable*
                constexpr std::ptrdiff_t m_pPrev = 0x50; // CEntityIdentity*
                constexpr std::ptrdiff_t m_pNext = 0x58; // CEntityIdentity*
                constexpr std::ptrdiff_t m_pPrevByClass = 0x60; // CEntityIdentity*
                constexpr std::ptrdiff_t m_pNextByClass = 0x68; // CEntityIdentity*
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace NodeData_t {
                constexpr std::ptrdiff_t m_vOrigin = 0x0; // Vector
                constexpr std::ptrdiff_t m_vMinBounds = 0xC; // Vector
                constexpr std::ptrdiff_t m_vMaxBounds = 0x18; // Vector
                constexpr std::ptrdiff_t m_worldNodePrefix = 0x28; // CUtlString
            }
            // Parent: None
            // Field count: 0
            namespace VMapResourceData_t {
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace WorldBuilderParams_t {
                constexpr std::ptrdiff_t m_flMinDrawVolumeSize = 0x0; // float32
                constexpr std::ptrdiff_t m_bBuildBakedLighting = 0x4; // bool
                constexpr std::ptrdiff_t m_bAggregateInstanceStreams = 0x5; // bool
                constexpr std::ptrdiff_t m_bakedLightingInfo = 0x8; // BakedLightingInfo_t
                constexpr std::ptrdiff_t m_nCompileTimestamp = 0x50; // uint64
                constexpr std::ptrdiff_t m_nCompileFingerprint = 0x58; // uint64
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PermEntityLumpData_t {
                constexpr std::ptrdiff_t m_name = 0x8; // CUtlString
                constexpr std::ptrdiff_t m_childLumps = 0x10; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
                constexpr std::ptrdiff_t m_entityKeyValues = 0x28; // CUtlLeanVector<EntityKeyValueData_t>
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace EntityIOConnectionData_t {
                constexpr std::ptrdiff_t m_outputName = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_targetType = 0x8; // uint32
                constexpr std::ptrdiff_t m_targetName = 0x10; // CUtlString
                constexpr std::ptrdiff_t m_inputName = 0x18; // CUtlString
                constexpr std::ptrdiff_t m_overrideParam = 0x20; // CUtlString
                constexpr std::ptrdiff_t m_flDelay = 0x28; // float32
                constexpr std::ptrdiff_t m_nTimesToFire = 0x2C; // int32
                constexpr std::ptrdiff_t m_paramMap = 0x30; // KeyValues3
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace VoxelVisBlockOffset_t {
                constexpr std::ptrdiff_t m_nOffset = 0x0; // uint32
                constexpr std::ptrdiff_t m_nElementCount = 0x4; // uint32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            namespace InfoForResourceTypeVMapResourceData_t {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace World_t {
                constexpr std::ptrdiff_t m_builderParams = 0x0; // WorldBuilderParams_t
                constexpr std::ptrdiff_t m_worldNodes = 0x60; // CUtlVector<NodeData_t>
                constexpr std::ptrdiff_t m_worldLightingInfo = 0x78; // BakedLightingInfo_t
                constexpr std::ptrdiff_t m_entityLumps = 0xC0; // CUtlVector<CStrongHandleCopyable<InfoForResourceTypeCEntityLump>>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace EntityKeyValueData_t {
                constexpr std::ptrdiff_t m_connections = 0x8; // CUtlVector<EntityIOConnectionData_t>
                constexpr std::ptrdiff_t m_keyValuesData = 0x20; // CUtlBinaryBlock
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CVoxelVisibility {
                constexpr std::ptrdiff_t m_nBaseClusterCount = 0x40; // uint32
                constexpr std::ptrdiff_t m_nPVSBytesPerCluster = 0x44; // uint32
                constexpr std::ptrdiff_t m_vMinBounds = 0x48; // Vector
                constexpr std::ptrdiff_t m_vMaxBounds = 0x54; // Vector
                constexpr std::ptrdiff_t m_flGridSize = 0x60; // float32
                constexpr std::ptrdiff_t m_nSkyVisibilityCluster = 0x64; // uint32
                constexpr std::ptrdiff_t m_nSunVisibilityCluster = 0x68; // uint32
                constexpr std::ptrdiff_t m_NodeBlock = 0x6C; // VoxelVisBlockOffset_t
                constexpr std::ptrdiff_t m_RegionBlock = 0x74; // VoxelVisBlockOffset_t
                constexpr std::ptrdiff_t m_EnclosedClusterListBlock = 0x7C; // VoxelVisBlockOffset_t
                constexpr std::ptrdiff_t m_EnclosedClustersBlock = 0x84; // VoxelVisBlockOffset_t
                constexpr std::ptrdiff_t m_MasksBlock = 0x8C; // VoxelVisBlockOffset_t
                constexpr std::ptrdiff_t m_nVisBlocks = 0x94; // VoxelVisBlockOffset_t
            }
        }
    }
}
