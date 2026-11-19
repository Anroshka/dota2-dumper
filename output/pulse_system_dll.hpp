// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-11-19 23:56:39.632743400 UTC

#pragma once

#include <cstddef>
#include <cstdint>

namespace dota2_dumper {
    namespace schemas {
        // Module: pulse_system.dll
        // Class count: 135
        // Enum count: 15
        namespace pulse_system_dll {
            // Alignment: 4
            // Member count: 2
            enum class PulseBestOutflowRules_t : uint32_t {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseTestEnumFlags_t : uint32_t {
                NONE = 0x0,
                FIRST = 0x1,
                SECOND = 0x2,
                THIRD = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class PulseTestEnumShape_t : uint32_t {
                CIRCLE = 0x64,
                SQUARE = 0xC8,
                TRIANGLE = 0x12C
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseCursorCancelPriority_t : uint32_t {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseTestEnumFlagsAlt_t : uint32_t {
                NONE = 0x0,
                FIRST = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseMethodCallMode_t : uint32_t {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 2
            // Member count: 157
            enum class PulseInstructionCode_t : uint16_t {
                INVALID = 0x0,
                IMMEDIATE_HALT = 0x1,
                RETURN_VOID = 0x2,
                RETURN_VALUE = 0x3,
                LOOP_BREAK = 0x4,
                NOP = 0x5,
                JUMP = 0x6,
                JUMP_COND = 0x7,
                CHUNK_LEAP = 0x8,
                CHUNK_LEAP_COND = 0x9,
                PULSE_CALL_SYNC = 0xA,
                PULSE_CALL_ASYNC_FIRE = 0xB,
                CREATE_CHILD_CURSOR_OUTFLOW = 0xC,
                CELL_INVOKE = 0xD,
                LIBRARY_INVOKE = 0xE,
                BROADCAST_INVOKE = 0xF,
                SET_VAR = 0x10,
                GET_VAR = 0x11,
                GET_VAR_DETACH = 0x12,
                DETACH_REGISTER = 0x13,
                SET_VAR_ARRAY_ELEMENT_1D = 0x14,
                SET_VAR_OBSERVABLE = 0x15,
                GET_CONST = 0x16,
                GET_ARRAY_ELEMENT = 0x17,
                GET_DOMAIN_VALUE = 0x18,
                COPY = 0x19,
                NOT = 0x1A,
                NEGATE = 0x1B,
                ADD = 0x1C,
                SUB = 0x1D,
                MUL = 0x1E,
                DIV = 0x1F,
                MOD = 0x20,
                LT = 0x21,
                LTE = 0x22,
                EQ = 0x23,
                NE = 0x24,
                AND = 0x25,
                OR = 0x26,
                SCALE = 0x27,
                SCALE_INV = 0x28,
                ELEMENT_ACCESS = 0x29,
                CONVERT_VALUE = 0x2A,
                REINTERPRET_INSTANCE = 0x2B,
                GET_BLACKBOARD_REFERENCE = 0x2C,
                SET_BLACKBOARD_REFERENCE = 0x2D,
                GET_TEMPVAR = 0x2E,
                SET_TEMPVAR = 0x2F,
                SET_TEMPVAR_OBSERVABLE = 0x30,
                BIND_LAMBDA = 0x31,
                LAST_SERIALIZED_CODE = 0x32,
                NEGATE_INT = 0x33,
                NEGATE_FLOAT = 0x34,
                NEGATE_VEC2 = 0x35,
                NEGATE_VEC3 = 0x36,
                NEGATE_VEC4 = 0x37,
                ADD_INT = 0x38,
                ADD_FLOAT = 0x39,
                ADD_STRING = 0x3A,
                ADD_VEC2 = 0x3B,
                ADD_VEC3 = 0x3C,
                ADD_VEC3WS_VEC3 = 0x3D,
                ADD_VEC3_VEC3WS = 0x3E,
                ADD_VEC4 = 0x3F,
                ADD_GAMETIME_FLOAT = 0x40,
                ADD_FLOAT_GAMETIME = 0x41,
                SUB_INT = 0x42,
                SUB_FLOAT = 0x43,
                SUB_VEC2 = 0x44,
                SUB_VEC3 = 0x45,
                SUB_VEC3WS_VEC3 = 0x46,
                SUB_VEC3WS_VEC3WS = 0x47,
                SUB_VEC4 = 0x48,
                SUB_GAMETIME_FLOAT = 0x49,
                SUB_GAMETIME = 0x4A,
                MUL_INT = 0x4B,
                MUL_FLOAT = 0x4C,
                DIV_FLOAT = 0x4D,
                MOD_INT = 0x4E,
                MOD_FLOAT = 0x4F,
                LT_INT = 0x50,
                LT_FLOAT = 0x51,
                LT_GAMETIME = 0x52,
                LTE_INT = 0x53,
                LTE_FLOAT = 0x54,
                LTE_GAMETIME = 0x55,
                EQ_BOOL = 0x56,
                NE_BOOL = 0x57,
                EQ_INT = 0x58,
                NE_INT = 0x59,
                EQ_FLOAT = 0x5A,
                NE_FLOAT = 0x5B,
                EQ_STRING = 0x5C,
                NE_STRING = 0x5D,
                EQ_VEC2 = 0x5E,
                NE_VEC2 = 0x5F,
                EQ_VEC3 = 0x60,
                NE_VEC3 = 0x61,
                EQ_QANGLE = 0x62,
                NE_QANGLE = 0x63,
                EQ_VEC3WS = 0x64,
                NE_VEC3WS = 0x65,
                EQ_VEC4 = 0x66,
                NE_VEC4 = 0x67,
                EQ_TRANSFORM = 0x68,
                NE_TRANSFORM = 0x69,
                EQ_TRANSFORMWS = 0x6A,
                NE_TRANSFORMWS = 0x6B,
                EQ_COLOR_RGB = 0x6C,
                NE_COLOR_RGB = 0x6D,
                EQ_GAMETIME = 0x6E,
                NE_GAMETIME = 0x6F,
                EQ_EHANDLE = 0x70,
                NE_EHANDLE = 0x71,
                EQ_RESOURCE = 0x72,
                NE_RESOURCE = 0x73,
                EQ_RESOURCE_NAME = 0x74,
                NE_RESOURCE_NAME = 0x75,
                EQ_SNDEVT_GUID = 0x76,
                NE_SNDEVT_GUID = 0x77,
                EQ_SNDEVT_NAME = 0x78,
                NE_SNDEVT_NAME = 0x79,
                EQ_ENTITY_NAME = 0x7A,
                NE_ENTITY_NAME = 0x7B,
                EQ_LEAF = 0x7C,
                NE_LEAF = 0x7D,
                EQ_MODEL_MATERIAL_GROUP = 0x7E,
                NE_MODEL_MATERIAL_GROUP = 0x7F,
                EQ_VARIANT = 0x80,
                NE_VARIANT = 0x81,
                EQ_ENUM = 0x82,
                NE_ENUM = 0x83,
                EQ_PANEL_HANDLE = 0x84,
                NE_PANEL_HANDLE = 0x85,
                EQ_ARRAY = 0x86,
                NE_ARRAY = 0x87,
                EQ_PARTICLE_EHANDLE = 0x88,
                NE_PARTICLE_EHANDLE = 0x89,
                EQ_ANIM_SEQUENCE = 0x8A,
                NE_ANIM_SEQUENCE = 0x8B,
                EQ_VDATA_CHOICE = 0x8C,
                NE_VDATA_CHOICE = 0x8D,
                SCALE_VEC3 = 0x8E,
                SCALE_VEC2 = 0x8F,
                SCALE_VEC4 = 0x90,
                SCALE_INV_VEC3 = 0x91,
                SCALE_INV_VEC2 = 0x92,
                SCALE_INV_VEC4 = 0x93,
                ELEMENT_ACCESS_VEC2 = 0x94,
                ELEMENT_ACCESS_VEC3 = 0x95,
                ELEMENT_ACCESS_VEC3WS = 0x96,
                ELEMENT_ACCESS_VEC4 = 0x97,
                ELEMENT_ACCESS_COLOR_RGB = 0x98,
                GET_CONST_INLINE_STORAGE = 0x99,
                BROADCAST_INVOKE_DOMAIN = 0x9A,
                BROADCAST_INVOKE_ARRAY_SIMPLE = 0x9B,
                BROADCAST_INVOKE_ARRAY_DOMAIN = 0x9C
            };
            // Alignment: 4
            // Member count: 5
            enum class PulseTestEnumColor_t : uint32_t {
                BLACK = 0x0,
                WHITE = 0x1,
                RED = 0x2,
                GREEN = 0x3,
                BLUE = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseCursorWakePriority_t : uint32_t {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
            };
            // Alignment: 4
            // Member count: 8
            enum class PulseVariableKeysSource_t : uint32_t {
                PRIVATE = 0x0,
                CPP = 0x1,
                VMAP = 0x2,
                VMDL = 0x3,
                XML = 0x4,
                VDATA = 0x5,
                VMAP_AND_VDATA = 0x6,
                COUNT = 0x7
            };
            // Alignment: 4
            // Member count: 1
            enum class PulseDurationStringFormat_t : uint32_t {
                MM_SS_LEADING_ZERO = 0x0
            };
            // Alignment: 4
            // Member count: 12
            enum class EPulseGraphExecutionHistoryFlag : uint32_t {
                NO_FLAGS = 0x0,
                CURSOR_ADD_TAG = 0x1,
                CURSOR_REMOVE_TAG = 0x2,
                CURSOR_RETIRED = 0x4,
                CURSOR_CREATE_CHILD = 0x8,
                REQUIREMENT_PASS = 0x10,
                REQUIREMENT_FAIL = 0x20,
                CALL_TO_PULSE = 0x40,
                INTERNAL_CALL_TO_PULSE = 0x80,
                RETURN = 0x100,
                RECORD_NEXT_SEQ_POINT = 0x200,
                CURSOR_MAX_INSTRUCTIONS = 0x400
            };
            // Alignment: 4
            // Member count: 33
            enum class PulseValueType_t : uint32_t {
                PVAL_VOID = 0xFFFFFFFF,
                PVAL_BOOL = 0x0,
                PVAL_INT = 0x1,
                PVAL_FLOAT = 0x2,
                PVAL_STRING = 0x3,
                PVAL_VEC2 = 0x4,
                PVAL_VEC3 = 0x5,
                PVAL_QANGLE = 0x6,
                PVAL_VEC3_WORLDSPACE = 0x7,
                PVAL_VEC4 = 0x8,
                PVAL_TRANSFORM = 0x9,
                PVAL_TRANSFORM_WORLDSPACE = 0xA,
                PVAL_COLOR_RGB = 0xB,
                PVAL_GAMETIME = 0xC,
                PVAL_EHANDLE = 0xD,
                PVAL_RESOURCE = 0xE,
                PVAL_RESOURCE_NAME = 0xF,
                PVAL_SNDEVT_GUID = 0x10,
                PVAL_SNDEVT_NAME = 0x11,
                PVAL_ENTITY_NAME = 0x12,
                PVAL_LEAF = 0x13,
                PVAL_MODEL_MATERIAL_GROUP = 0x14,
                PVAL_CURSOR_FLOW = 0x15,
                PVAL_VARIANT = 0x16,
                PVAL_UNKNOWN = 0x17,
                PVAL_ENUM = 0x18,
                PVAL_PANORAMA_PANEL_HANDLE = 0x19,
                PVAL_ARRAY = 0x1A,
                PVAL_PARTICLE_EHANDLE = 0x1B,
                PVAL_ANIM_SEQUENCE = 0x1C,
                PVAL_VDATA_CHOICE = 0x1D,
                PVAL_LAMBDA = 0x1E,
                PVAL_COUNT = 0x1F
            };
            // Alignment: 4
            // Member count: 6
            enum class PulseApiFeature_t : uint32_t {
                AF_NONE = 0x0,
                AF_ENTITIES = 0x1,
                AF_PANORAMA = 0x2,
                AF_PARTICLES = 0x8,
                AF_FAKE_ENTITIES = 0x10,
                AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseDomainValueType_t : uint32_t {
                INVALID = 0xFFFFFFFF,
                ENTITY_NAME = 0x0,
                PANEL_ID = 0x1,
                COUNT = 0x2
            };
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_Step_TestDomainDestroyFakeEntity {
            }
            // Parent: CPulseCell_WaitForCursorsWithTagBase
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPulseEditorSearchExpr
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            namespace CPulseCell_WaitForCursorsWithTag {
                constexpr std::ptrdiff_t m_bTagSelfWhenComplete = 0x128; // bool
                constexpr std::ptrdiff_t m_nDesiredKillPriority = 0x12C; // PulseCursorCancelPriority_t
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_RaceCursors {
                constexpr std::ptrdiff_t m_Outflows = 0xD8; // CUtlVector<CPulse_OutflowConnection>
                constexpr std::ptrdiff_t m_OnFinished = 0xF0; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_NoInflow {
            }
            // Parent: CBasePulseGraphInstance
            // Field count: 0
            namespace CPulseGraphInstance_TestDomain_FakeEntityOwner {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Base {
                constexpr std::ptrdiff_t m_nEditorNodeID = 0x8; // PulseDocNodeID_t
            }
            // Parent: CPulse_OutflowConnection
            // Field count: 0
            namespace CPulse_ResumePoint {
            }
            // Parent: CPulseExecCursor
            // Field count: 2
            namespace CTestDomainDerived_Cursor {
                constexpr std::ptrdiff_t m_nCursorValueA = 0xD8; // int32
                constexpr std::ptrdiff_t m_nCursorValueB = 0xDC; // int32
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_PickBestOutflowSelector {
                constexpr std::ptrdiff_t m_nCheckType = 0x48; // PulseBestOutflowRules_t
                constexpr std::ptrdiff_t m_OutflowList = 0x50; // PulseSelectorOutflowList_t
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorHeaderIcon
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_WaitForObservable {
                constexpr std::ptrdiff_t m_OnTrue = 0xD8; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 4
            namespace CPulse_OutflowConnection {
                constexpr std::ptrdiff_t m_SourceOutflowName = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nDestChunk = 0x10; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nInstruction = 0x14; // int32
                constexpr std::ptrdiff_t m_OutflowRegisterMap = 0x18; // PulseRegisterMap_t
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseGraphDef {
                constexpr std::ptrdiff_t m_DomainIdentifier = 0x8; // PulseSymbol_t
                constexpr std::ptrdiff_t m_DomainSubType = 0x18; // CPulseType
                constexpr std::ptrdiff_t m_ParentMapName = 0x30; // PulseSymbol_t
                constexpr std::ptrdiff_t m_ParentXmlName = 0x40; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Chunks = 0x50; // CUtlVector<CPulse_Chunk*>
                constexpr std::ptrdiff_t m_Cells = 0x68; // CUtlVector<CPulseCell_Base*>
                constexpr std::ptrdiff_t m_Vars = 0x80; // CUtlVector<CPulse_Variable>
                constexpr std::ptrdiff_t m_TempVarBanks = 0x98; // CUtlVector<CPulse_TempVarBankDefinition*>
                constexpr std::ptrdiff_t m_PublicOutputs = 0xB0; // CUtlVector<CPulse_PublicOutput>
                constexpr std::ptrdiff_t m_InvokeBindings = 0xC8; // CUtlVector<CPulse_InvokeBinding*>
                constexpr std::ptrdiff_t m_CallInfos = 0xE0; // CUtlVector<CPulse_CallInfo*>
                constexpr std::ptrdiff_t m_Constants = 0xF8; // CUtlVector<CPulse_Constant>
                constexpr std::ptrdiff_t m_DomainValues = 0x110; // CUtlVector<CPulse_DomainValue>
                constexpr std::ptrdiff_t m_BlackboardReferences = 0x128; // CUtlVector<CPulse_BlackboardReference>
                constexpr std::ptrdiff_t m_LambdaDefs = 0x140; // CUtlVector<CPulse_LambdaDef>
                constexpr std::ptrdiff_t m_OutputConnections = 0x158; // CUtlVector<CPulse_OutputConnection*>
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestYieldForever {
            }
            // Parent: CPulseGraphInstance_TestDomain
            // Field count: 0
            namespace CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestWaitWithCursorState__InstanceState_t {
                constexpr std::ptrdiff_t m_nDummy = 0x0; // int32
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_FireCursors {
                constexpr std::ptrdiff_t m_Outflows = 0xD8; // CUtlVector<CPulse_OutflowConnection>
                constexpr std::ptrdiff_t m_bWaitForChildOutflows = 0xF0; // bool
                constexpr std::ptrdiff_t m_OnFinished = 0xF8; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestYieldWithObservables__CursorState_t {
                constexpr std::ptrdiff_t m_LiveFloatValue = 0x0; // HPulseObservableLambda<float32>
                constexpr std::ptrdiff_t m_LiveStringValue = 0xC; // HPulseObservableLambda<CUtlString>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Timeline__TimelineEvent_t {
                constexpr std::ptrdiff_t m_flTimeFromPrevious = 0x0; // float32
                constexpr std::ptrdiff_t m_EventOutflow = 0x8; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_IntervalTimer__CursorState_t {
                constexpr std::ptrdiff_t m_StartTime = 0x0; // GameTime_t
                constexpr std::ptrdiff_t m_EndTime = 0x4; // GameTime_t
                constexpr std::ptrdiff_t m_flWaitInterval = 0x8; // float32
                constexpr std::ptrdiff_t m_flWaitIntervalHigh = 0xC; // float32
                constexpr std::ptrdiff_t m_bCompleteOnNextWake = 0x10; // bool
                constexpr std::ptrdiff_t m_bCompletingVeryCloseToEndOfInterval = 0x11; // bool
            }
            // Parent: CPulseCell_Base
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BaseRequirement {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorHeaderIcon
            namespace CPulseCell_BaseState {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace OutflowWithRequirements_t {
                constexpr std::ptrdiff_t m_Connection = 0x0; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_DestinationFlowNodeID = 0x48; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_RequirementNodeIDs = 0x50; // CUtlVector<PulseDocNodeID_t>
                constexpr std::ptrdiff_t m_nCursorStateBlockIndex = 0x68; // CUtlVector<int32>
            }
            // Parent: CPulseCell_BaseRequirement
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_IsRequirementValid {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_TestWaitWithAutoTracepoints {
                constexpr std::ptrdiff_t m_TracePrefix = 0xD8; // CUtlString
                constexpr std::ptrdiff_t m_WakeResume = 0xE0; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseNodeDynamicOutflows_t__DynamicOutflow_t {
                constexpr std::ptrdiff_t m_OutflowID = 0x0; // CGlobalSymbol
                constexpr std::ptrdiff_t m_Connection = 0x8; // CPulse_OutflowConnection
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_MultiOutflow_WithParams {
                constexpr std::ptrdiff_t m_Out1 = 0x48; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_Out2 = 0x90; // SignatureOutflow_Continue
            }
            // Parent: None
            // Field count: 0
            namespace CBasePulseGraphInstance {
            }
            // Parent: CPulseCell_Inflow_BaseEntrypoint
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_GraphHook {
                constexpr std::ptrdiff_t m_HookName = 0x80; // PulseSymbol_t
            }
            // Parent: CPulse_ResumePoint
            // Field count: 0
            namespace SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t {
                constexpr std::ptrdiff_t nTestStep = 0x0; // int32
            }
            // Parent: CPulseExecCursor
            // Field count: 4
            namespace CPulseTurtleGraphicsCursor {
                constexpr std::ptrdiff_t m_Color = 0xD8; // Color
                constexpr std::ptrdiff_t m_vPos = 0xDC; // Vector2D
                constexpr std::ptrdiff_t m_flHeadingDeg = 0xE4; // float32
                constexpr std::ptrdiff_t m_bPenUp = 0xE8; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestWaitWithCursorState__CursorState_t {
                constexpr std::ptrdiff_t flWaitValue = 0x0; // float32
                constexpr std::ptrdiff_t bFail = 0x4; // bool
                constexpr std::ptrdiff_t m_hSelfCursor = 0x8; // HYieldedCursor
                constexpr std::ptrdiff_t m_hSelfCellInstanceUntyped = 0x14; // HPulseCellBase
                constexpr std::ptrdiff_t m_hSelfCellInstance = 0x1C; // HPulseCell<CPulseCell_TestWaitWithCursorState>
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_BaseEntrypoint {
                constexpr std::ptrdiff_t m_EntryChunk = 0x48; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_RegisterMap = 0x50; // PulseRegisterMap_t
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_MultiInflow_NoDefault {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_WaitForCursorsWithTagBase {
                constexpr std::ptrdiff_t m_nCursorsAllowedToWait = 0xD8; // int32
                constexpr std::ptrdiff_t m_WaitComplete = 0xE0; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_InvokeBinding {
                constexpr std::ptrdiff_t m_RegisterMap = 0x0; // PulseRegisterMap_t
                constexpr std::ptrdiff_t m_FuncName = 0x30; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nCellIndex = 0x40; // PulseRuntimeCellIndex_t
                constexpr std::ptrdiff_t m_nSrcChunk = 0x44; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nSrcInstruction = 0x48; // int32
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_IntervalTimer {
                constexpr std::ptrdiff_t m_Completed = 0xD8; // CPulse_ResumePoint
                constexpr std::ptrdiff_t m_OnInterval = 0x120; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_bFireAtStart = 0x168; // bool
                constexpr std::ptrdiff_t m_bFireAtEnd = 0x169; // bool
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BaseLerp {
                constexpr std::ptrdiff_t m_WakeResume = 0xD8; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_BaseValue
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_Value_TestValue50 {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_MultiOutflow_WithParams_Yielding {
                constexpr std::ptrdiff_t m_Out1 = 0xD8; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_AsyncChild1 = 0x120; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_AsyncChild2 = 0x168; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_YieldResume1 = 0x1B0; // SignatureOutflow_Resume
                constexpr std::ptrdiff_t m_YieldResume2 = 0x1F8; // SignatureOutflow_Resume
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_WaitForObservable__CursorState_t {
                constexpr std::ptrdiff_t m_Condition = 0x0; // HPulseObservableLambda<bool>
            }
            // Parent: CPulseCell_Inflow_BaseEntrypoint
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_EventHandler {
                constexpr std::ptrdiff_t m_EventName = 0x80; // PulseSymbol_t
            }
            // Parent: CPulseCell_Base
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BaseFlow {
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_Step_TestDomainTracepoint {
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PGDInstruction_t {
                constexpr std::ptrdiff_t m_nCode = 0x0; // PulseInstructionCode_t
                constexpr std::ptrdiff_t m_nVar = 0x4; // PulseRuntimeVarIndex_t
                constexpr std::ptrdiff_t m_nReg0 = 0x8; // PulseRuntimeRegisterIndex_t
                constexpr std::ptrdiff_t m_nReg1 = 0xA; // PulseRuntimeRegisterIndex_t
                constexpr std::ptrdiff_t m_nReg2 = 0xC; // PulseRuntimeRegisterIndex_t
                constexpr std::ptrdiff_t m_nLambdaIdx = 0xE; // PulseRuntimeLambdaIndex_t
                constexpr std::ptrdiff_t m_nInvokeBindingIndex = 0x10; // PulseRuntimeInvokeIndex_t
                constexpr std::ptrdiff_t m_nChunk = 0x14; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nDestInstruction = 0x18; // int32
                constexpr std::ptrdiff_t m_nCallInfoIndex = 0x1C; // PulseRuntimeCallInfoIndex_t
                constexpr std::ptrdiff_t m_nConstIdx = 0x20; // PulseRuntimeConstantIndex_t
                constexpr std::ptrdiff_t m_nDomainValueIdx = 0x22; // PulseRuntimeDomainValueIndex_t
                constexpr std::ptrdiff_t m_nBlackboardReferenceIdx = 0x24; // PulseRuntimeBlackboardReferenceIndex_t
                constexpr std::ptrdiff_t m_nTempVarIdx = 0x26; // PulseRuntimeTempVarIndex_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Outflow_CycleShuffled__InstanceState_t {
                constexpr std::ptrdiff_t m_Shuffle = 0x0; // CUtlVectorFixedGrowable<uint8,8>
                constexpr std::ptrdiff_t m_nNextShuffle = 0x20; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BaseLerp__CursorState_t {
                constexpr std::ptrdiff_t m_StartTime = 0x0; // GameTime_t
                constexpr std::ptrdiff_t m_EndTime = 0x4; // GameTime_t
            }
            // Parent: CPulseGraphInstance_TestDomain
            // Field count: 1
            namespace CPulseGraphInstance_TestDomain_Derived {
                constexpr std::ptrdiff_t m_nInstanceValueX = 0xD8; // int32
            }
            // Parent: CBasePulseGraphInstance
            // Field count: 9
            namespace CPulseGraphInstance_TestDomain {
                constexpr std::ptrdiff_t m_bIsRunningUnitTests = 0xA8; // bool
                constexpr std::ptrdiff_t m_bExplicitTimeStepping = 0xA9; // bool
                constexpr std::ptrdiff_t m_bExpectingToDestroyWithYieldedCursors = 0xAA; // bool
                constexpr std::ptrdiff_t m_bQuietTracepoints = 0xAB; // bool
                constexpr std::ptrdiff_t m_bExpectingCursorTerminatedDueToMaxInstructions = 0xAC; // bool
                constexpr std::ptrdiff_t m_nCursorsTerminatedDueToMaxInstructions = 0xB0; // int32
                constexpr std::ptrdiff_t m_nNextValidateIndex = 0xB4; // int32
                constexpr std::ptrdiff_t m_Tracepoints = 0xB8; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_bTestYesOrNoPath = 0xD0; // bool
            }
            // Parent: CPulse_OutflowConnection
            // Field count: 0
            namespace SignatureOutflow_Continue {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Timeline {
                constexpr std::ptrdiff_t m_TimelineEvents = 0xD8; // CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
                constexpr std::ptrdiff_t m_bWaitForChildOutflows = 0xF0; // bool
                constexpr std::ptrdiff_t m_OnFinished = 0xF8; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_Inflow_BaseEntrypoint
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_EntOutputHandler {
                constexpr std::ptrdiff_t m_SourceEntity = 0x80; // PulseSymbol_t
                constexpr std::ptrdiff_t m_SourceOutput = 0x90; // PulseSymbol_t
                constexpr std::ptrdiff_t m_ExpectedParamType = 0xA0; // CPulseType
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_Outflow_TestExplicitYesNo {
                constexpr std::ptrdiff_t m_Yes = 0x48; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_No = 0x90; // CPulse_OutflowConnection
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_Outflow_TestRandomYesNo {
                constexpr std::ptrdiff_t m_Yes = 0x48; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_No = 0x90; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Outflow_CycleOrdered__InstanceState_t {
                constexpr std::ptrdiff_t m_nNextIndex = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_LimitCount__InstanceState_t {
                constexpr std::ptrdiff_t m_nCurrentCount = 0x0; // int32
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Test_MultiInflow_WithDefault {
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Step_DebugLog {
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCustomFGDMetadata
            namespace CPulseCell_BaseYieldingInflow {
                constexpr std::ptrdiff_t m_BaseFlow_OnAfterCancel = 0x48; // CPulse_ResumePoint
                constexpr std::ptrdiff_t m_BaseFlow_WhileActive = 0x90; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseNodeDynamicOutflows_t {
                constexpr std::ptrdiff_t m_Outflows = 0x0; // CUtlVector<PulseNodeDynamicOutflows_t::DynamicOutflow_t>
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_IsRequirementValid__Criteria_t {
                constexpr std::ptrdiff_t m_bIsValid = 0x0; // bool
            }
            // Parent: CPulseCell_Inflow_BaseEntrypoint
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_ObservableVariableListener {
                constexpr std::ptrdiff_t m_nBlackboardReference = 0x80; // PulseRuntimeBlackboardReferenceIndex_t
                constexpr std::ptrdiff_t m_bSelfReference = 0x82; // bool
            }
            // Parent: CPulseCell_BaseState
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_ObservableSwitchState {
                constexpr std::ptrdiff_t m_Cases = 0xD8; // CPulseObservableSwitchCases
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Outflow_CycleOrdered {
                constexpr std::ptrdiff_t m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseSelectorOutflowList_t {
                constexpr std::ptrdiff_t m_Outflows = 0x0; // CUtlVector<OutflowWithRequirements_t>
            }
            // Parent: CBasePulseGraphInstance
            // Field count: 0
            namespace CPulseGraphInstance_TurtleGraphics {
            }
            // Parent: CPulseCell_BaseValue
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_Val_TestDomainGetEntityName {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_Inflow_Wait {
                constexpr std::ptrdiff_t m_WakeResume = 0xD8; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestWaitWithCursorState {
                constexpr std::ptrdiff_t m_WakeResume = 0xD8; // CPulse_ResumePoint
                constexpr std::ptrdiff_t m_WakeFail = 0x120; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Outflow_CycleShuffled {
                constexpr std::ptrdiff_t m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: CPulseCell_Inflow_BaseEntrypoint
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_Method {
                constexpr std::ptrdiff_t m_MethodName = 0x80; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Description = 0x90; // CUtlString
                constexpr std::ptrdiff_t m_bIsPublic = 0x98; // bool
                constexpr std::ptrdiff_t m_Args = 0xA0; // CUtlLeanVector<CPulseRuntimeMethodArg>
                constexpr std::ptrdiff_t m_ReturnValues = 0xB0; // CUtlLeanVector<CPulseRuntimeMethodArg>
            }
            // Parent: CPulseCell_Base
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BaseValue {
            }
            // Parent: CPulseCell_BaseState
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_BooleanSwitchState {
                constexpr std::ptrdiff_t m_WhenTrue = 0xD8; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_WhenFalse = 0x120; // CPulse_OutflowConnection
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Inflow_Yield {
                constexpr std::ptrdiff_t m_UnyieldResume = 0xD8; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_Base
            // Field count: 1
            namespace CPulseCell_Unknown {
                constexpr std::ptrdiff_t m_UnknownKeys = 0x48; // KeyValues3
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Outflow_CycleRandom {
                constexpr std::ptrdiff_t m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Step_PublicOutput {
                constexpr std::ptrdiff_t m_OutputIndex = 0x48; // PulseRuntimeOutputIndex_t
            }
            // Parent: CPulseCell_BaseValue
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_Val_TestDomainFindEntityByName {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_BlackboardReference {
                constexpr std::ptrdiff_t m_hBlackboardResource = 0x0; // CStrongHandle<InfoForResourceTypeIPulseGraphDef>
                constexpr std::ptrdiff_t m_BlackboardResource = 0x8; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nNodeID = 0x18; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_NodeName = 0x20; // CGlobalSymbol
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_ObservableSwitchState__CursorState_t {
                constexpr std::ptrdiff_t m_SwitchValue = 0x0; // HPulseObservableLambda<CPulseVariant>
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_ReturnValues {
            }
            // Parent: CPulseCell_BaseValue
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_TestEnums {
                constexpr std::ptrdiff_t m_nReferenceColor = 0x48; // PulseTestEnumColor_t
                constexpr std::ptrdiff_t m_nReferenceFlags = 0x4C; // PulseTestEnumFlags_t
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPulseEditorHeaderText
            namespace CPulseCell_Step_TestDomainEntFire {
                constexpr std::ptrdiff_t m_Input = 0x48; // CUtlString
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_BooleanSwitchState__CursorState_t {
                constexpr std::ptrdiff_t m_Condition = 0x0; // HPulseObservableLambda<bool>
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            namespace CPulseCell_ExampleSelector {
                constexpr std::ptrdiff_t m_OutflowList = 0x48; // PulseSelectorOutflowList_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_CallInfo {
                constexpr std::ptrdiff_t m_PortName = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nEditorNodeID = 0x10; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_RegisterMap = 0x18; // PulseRegisterMap_t
                constexpr std::ptrdiff_t m_CallMethodID = 0x48; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_nSrcChunk = 0x4C; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nSrcInstruction = 0x50; // int32
                constexpr std::ptrdiff_t m_nBreakDestChunk = 0x54; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nBreakDestInstruction = 0x58; // int32
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseFunctionHiddenInTool
            namespace CPulseCell_InlineNodeSkipSelector {
                constexpr std::ptrdiff_t m_nFlowNodeID = 0x48; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_bAnd = 0x4C; // bool
                constexpr std::ptrdiff_t m_PassOutflow = 0x50; // PulseSelectorOutflowList_t
                constexpr std::ptrdiff_t m_FailOutflow = 0x68; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 3
            namespace CPulseCell_ExampleCriteria__Criteria_t {
                constexpr std::ptrdiff_t m_flFloatValue1 = 0x0; // float32
                constexpr std::ptrdiff_t m_flFloatValue2 = 0x4; // float32
                constexpr std::ptrdiff_t m_bMyBool = 0x8; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_LambdaDef {
                constexpr std::ptrdiff_t m_EvaluateConnection = 0x0; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_ResultType = 0x48; // CPulseType
                constexpr std::ptrdiff_t m_nEditorNodeID = 0x60; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_PortName = 0x68; // PulseSymbol_t
                constexpr std::ptrdiff_t m_DependentObservableVars = 0x78; // CUtlVector<PulseRuntimeVarIndex_t>
                constexpr std::ptrdiff_t m_DependentObservableBlackboardReferences = 0x90; // CUtlVector<PulseRuntimeBlackboardReferenceIndex_t>
                constexpr std::ptrdiff_t m_DependentObservableTempVars = 0xA8; // CUtlVector<PulseDocNodeID_t>
            }
            // Parent: CPulseCell_BaseRequirement
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_ExampleCriteria {
            }
            // Parent: CPulseCell_BaseRequirement
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            namespace CPulseCell_LimitCount {
                constexpr std::ptrdiff_t m_nLimitCount = 0x48; // int32
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_Step_CallExternalMethod {
                constexpr std::ptrdiff_t m_MethodName = 0xD8; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nBlackboardIndex = 0xE8; // PulseRuntimeBlackboardReferenceIndex_t
                constexpr std::ptrdiff_t m_ExpectedArgs = 0xF0; // CUtlLeanVector<CPulseRuntimeMethodArg>
                constexpr std::ptrdiff_t m_nAsyncCallMode = 0x100; // PulseMethodCallMode_t
                constexpr std::ptrdiff_t m_OnFinished = 0x108; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_LimitCount__Criteria_t {
                constexpr std::ptrdiff_t m_bLimitCountPasses = 0x0; // bool
            }
            // Parent: CPulseCell_BaseFlow
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CPulseCell_Step_TestDomainCreateFakeEntity {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestYieldWithObservables {
                constexpr std::ptrdiff_t m_flWatchForFloatValue = 0xD8; // float32
                constexpr std::ptrdiff_t m_WatchForStringValue = 0xE0; // CUtlString
                constexpr std::ptrdiff_t m_WakeResume = 0xE8; // CPulse_ResumePoint
            }
            // Parent: CPulseCell_WaitForCursorsWithTagBase
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            namespace CPulseCell_CursorQueue {
                constexpr std::ptrdiff_t m_nCursorsAllowedToRunParallel = 0x128; // int32
            }
            // Parent: None
            // Field count: 0
            namespace CPulseExecCursor {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeLambdaIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeVarIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeEntrypointIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeDomainValueIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_Chunk {
                constexpr std::ptrdiff_t m_Instructions = 0x0; // CUtlLeanVector<PGDInstruction_t>
                constexpr std::ptrdiff_t m_Registers = 0x10; // CUtlLeanVector<CPulse_RegisterInfo>
                constexpr std::ptrdiff_t m_InstructionDebugInfos = 0x20; // CUtlLeanVector<CPulse_InstructionDebug>
                constexpr std::ptrdiff_t m_nTempVarBank = 0x30; // PulseRuntimeTempVarBankIndex_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseBreakpointLocation {
                constexpr std::ptrdiff_t m_NodeID = 0x0; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_SequencePoint = 0x8; // PulseSymbol_t
                constexpr std::ptrdiff_t m_PortName = 0x18; // PulseSymbol_t
                constexpr std::ptrdiff_t m_bDeferBreak = 0x28; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeInvokeIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeRegisterIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseCursorID_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseGraphExecutionHistoryCursorDesc_t {
                constexpr std::ptrdiff_t nCursorID = 0x0; // PulseCursorID_t
                constexpr std::ptrdiff_t vecAncestorCursorIDs = 0x8; // CUtlVector<PulseCursorID_t>
                constexpr std::ptrdiff_t nSpawnNodeID = 0x20; // PulseDocNodeID_t
                constexpr std::ptrdiff_t nRetiredAtNodeID = 0x24; // PulseDocNodeID_t
                constexpr std::ptrdiff_t externalCallerInfo = 0x28; // PulseExecHistoryCallerInfo_t
                constexpr std::ptrdiff_t m_vecHistory = 0x30; // CUtlVector<PulseGraphExecutionHistoryEntry_t*>
                constexpr std::ptrdiff_t flLastReferenced = 0x48; // float32
                constexpr std::ptrdiff_t bWasAnObservableComputation = 0x4C; // bool
                constexpr std::ptrdiff_t bOverflowedInstructionHistoryLimit = 0x4D; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeTempVarIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            namespace TestComponent_t {
                constexpr std::ptrdiff_t m_ComponentData = 0x8; // CUtlString
            }
            // Parent: None
            // Field count: 3
            namespace PulseRegisterMap_t {
                constexpr std::ptrdiff_t m_Inparams = 0x0; // KeyValues3
                constexpr std::ptrdiff_t m_InparamsWhichCanBeMoved = 0x10; // CKV3MemberNameSet
                constexpr std::ptrdiff_t m_Outparams = 0x20; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_TempVarBankDefinition {
                constexpr std::ptrdiff_t m_TempVars = 0x0; // CUtlVector<CPulse_TempVarInfo>
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_PublicOutput {
                constexpr std::ptrdiff_t m_Name = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Description = 0x10; // CUtlString
                constexpr std::ptrdiff_t m_Args = 0x18; // CUtlLeanVector<CPulseRuntimeMethodArg>
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_OutputConnection {
                constexpr std::ptrdiff_t m_SourceOutput = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_TargetEntity = 0x10; // PulseSymbol_t
                constexpr std::ptrdiff_t m_TargetInput = 0x20; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Param = 0x30; // PulseSymbol_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseGraphExecutionHistoryNodeDesc_t {
                constexpr std::ptrdiff_t strCellDesc = 0x0; // CBufferString
                constexpr std::ptrdiff_t strBindingName = 0x10; // PulseSymbol_t
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseGraphExecutionHistoryEntry_t {
                constexpr std::ptrdiff_t nEditorID = 0x0; // PulseDocNodeID_t
                constexpr std::ptrdiff_t seqPoint = 0x8; // PulseSymbol_t
                constexpr std::ptrdiff_t flExecTime = 0x18; // float32
                constexpr std::ptrdiff_t unFlags = 0x1C; // uint32
                constexpr std::ptrdiff_t tagName = 0x20; // PulseSymbol_t
                constexpr std::ptrdiff_t childID = 0x30; // PulseCursorID_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseCursorYieldToken_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseGraphExecutionHistory {
                constexpr std::ptrdiff_t m_nInstanceID = 0x0; // PulseGraphInstanceID_t
                constexpr std::ptrdiff_t m_strFileName = 0x8; // CUtlString
                constexpr std::ptrdiff_t m_mapCellDesc = 0x10; // CUtlOrderedMap<PulseDocNodeID_t,PulseGraphExecutionHistoryNodeDesc_t*>
                constexpr std::ptrdiff_t m_mapCursorDesc = 0x38; // CUtlOrderedMap<PulseCursorID_t,PulseGraphExecutionHistoryCursorDesc_t*>
                constexpr std::ptrdiff_t m_mapNodeToNewCursorCount = 0x60; // CUtlOrderedMap<PulseDocNodeID_t,CUtlVector<int32>>
                constexpr std::ptrdiff_t m_nCulledCursorCount = 0xA0; // int32
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_Variable {
                constexpr std::ptrdiff_t m_Name = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Description = 0x10; // CUtlString
                constexpr std::ptrdiff_t m_Type = 0x18; // CPulseType
                constexpr std::ptrdiff_t m_DefaultValue = 0x30; // KeyValues3
                constexpr std::ptrdiff_t m_nKeysSource = 0x44; // PulseVariableKeysSource_t
                constexpr std::ptrdiff_t m_bIsPublicBlackboardVariable = 0x48; // bool
                constexpr std::ptrdiff_t m_bIsObservable = 0x49; // bool
                constexpr std::ptrdiff_t m_nEditorNodeID = 0x4C; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_Metadata = 0x50; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeOutputIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_DomainValue {
                constexpr std::ptrdiff_t m_nType = 0x0; // PulseDomainValueType_t
                constexpr std::ptrdiff_t m_Value = 0x8; // CGlobalSymbolCaseSensitive
                constexpr std::ptrdiff_t m_RequiredRuntimeType = 0x10; // CPulseType
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeBlackboardReferenceIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PulseExecHistoryCallerInfo_t {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_TempVarInfo {
                constexpr std::ptrdiff_t m_Name = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_Type = 0x10; // CPulseType
                constexpr std::ptrdiff_t m_nEditorNodeID = 0x28; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_bIsObservable = 0x2C; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeConstantIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseDocNodeID_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_RegisterInfo {
                constexpr std::ptrdiff_t m_Type = 0x0; // CPulseType
                constexpr std::ptrdiff_t m_OriginName = 0x18; // CKV3MemberNameWithStorage
                constexpr std::ptrdiff_t m_nWrittenByInstruction = 0x50; // int32
                constexpr std::ptrdiff_t m_nLastReadByInstruction = 0x54; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeCellIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeTempVarBankIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseGraphInstanceID_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // uint32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_Constant {
                constexpr std::ptrdiff_t m_Type = 0x0; // CPulseType
                constexpr std::ptrdiff_t m_Value = 0x18; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeCallInfoIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeChunkIndex_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseRuntimeMethodArg {
                constexpr std::ptrdiff_t m_Name = 0x0; // CKV3MemberNameWithStorage
                constexpr std::ptrdiff_t m_Description = 0x38; // CUtlString
                constexpr std::ptrdiff_t m_Type = 0x40; // CPulseType
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MIsBoxedIntegerType
            namespace PulseRuntimeStateOffset_t {
                constexpr std::ptrdiff_t m_Value = 0x0; // uint16
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulse_InstructionDebug {
                constexpr std::ptrdiff_t m_nFlowNodeID = 0x0; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_nValueNodeID = 0x4; // PulseDocNodeID_t
                constexpr std::ptrdiff_t m_SequencePointName = 0x8; // PulseSymbol_t
            }
        }
    }
}
