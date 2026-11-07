// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-11-07 09:16:29.927261200 UTC

pub const dota2_dumper = struct {
    pub const schemas = struct {
        // Module: pulse_system.dll
        // Class count: 132
        // Enum count: 15
        pub const pulse_system_dll = struct {
            // Alignment: 4
            // Member count: 2
            pub const PulseBestOutflowRules_t = enum(u32) {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseTestEnumFlags_t = enum(u32) {
                NONE = 0x0,
                FIRST = 0x1,
                SECOND = 0x2,
                THIRD = 0x4
            };
            // Alignment: 4
            // Member count: 3
            pub const PulseTestEnumShape_t = enum(u32) {
                CIRCLE = 0x64,
                SQUARE = 0xC8,
                TRIANGLE = 0x12C
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseCursorCancelPriority_t = enum(u32) {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const PulseTestEnumFlagsAlt_t = enum(u32) {
                NONE = 0x0,
                FIRST = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const PulseMethodCallMode_t = enum(u32) {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const PulseTestEnumColor_t = enum(u32) {
                BLACK = 0x0,
                WHITE = 0x1,
                RED = 0x2,
                GREEN = 0x3,
                BLUE = 0x4
            };
            // Alignment: 4
            // Member count: 2
            pub const PulseCursorWakePriority_t = enum(u32) {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
            };
            // Alignment: 4
            // Member count: 7
            pub const PulseVariableKeysSource_t = enum(u32) {
                PRIVATE = 0x0,
                CPP = 0x1,
                VMAP = 0x2,
                VMDL = 0x3,
                XML = 0x4,
                VDATA = 0x5,
                COUNT = 0x6
            };
            // Alignment: 4
            // Member count: 1
            pub const PulseDurationStringFormat_t = enum(u32) {
                MM_SS_LEADING_ZERO = 0x0
            };
            // Alignment: 4
            // Member count: 9
            pub const EPulseGraphExecutionHistoryFlag = enum(u32) {
                NO_FLAGS = 0x0,
                CURSOR_ADD_TAG = 0x1,
                CURSOR_REMOVE_TAG = 0x2,
                CURSOR_RETIRED = 0x4,
                CURSOR_CREATE_CHILD = 0x8,
                REQUIREMENT_PASS = 0x10,
                REQUIREMENT_FAIL = 0x20,
                CALL_TO_PULSE = 0x40,
                RETURN = 0x80
            };
            // Alignment: 4
            // Member count: 32
            pub const PulseValueType_t = enum(u32) {
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
                PVAL_COUNT = 0x1E
            };
            // Alignment: 4
            // Member count: 6
            pub const PulseApiFeature_t = enum(u32) {
                AF_NONE = 0x0,
                AF_ENTITIES = 0x1,
                AF_PANORAMA = 0x2,
                AF_PARTICLES = 0x8,
                AF_FAKE_ENTITIES = 0x10,
                AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
            };
            // Alignment: 2
            // Member count: 128
            pub const PulseInstructionCode_t = enum(u16) {
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
                SET_VAR = 0xF,
                GET_VAR = 0x10,
                GET_VAR_DETACH = 0x11,
                DETACH_REGISTER = 0x12,
                SET_VAR_ARRAY_ELEMENT_1D = 0x13,
                SET_VAR_OBSERVABLE = 0x14,
                GET_CONST = 0x15,
                GET_ARRAY_ELEMENT = 0x16,
                GET_DOMAIN_VALUE = 0x17,
                COPY = 0x18,
                NOT = 0x19,
                NEGATE = 0x1A,
                ADD = 0x1B,
                SUB = 0x1C,
                MUL = 0x1D,
                DIV = 0x1E,
                MOD = 0x1F,
                LT = 0x20,
                LTE = 0x21,
                EQ = 0x22,
                NE = 0x23,
                AND = 0x24,
                OR = 0x25,
                SCALE = 0x26,
                SCALE_INV = 0x27,
                ELEMENT_ACCESS = 0x28,
                CONVERT_VALUE = 0x29,
                REINTERPRET_INSTANCE = 0x2A,
                GET_BLACKBOARD_REFERENCE = 0x2B,
                SET_BLACKBOARD_REFERENCE = 0x2C,
                GET_TEMPVAR = 0x2D,
                SET_TEMPVAR = 0x2E,
                SET_TEMPVAR_OBSERVABLE = 0x2F,
                LAST_SERIALIZED_CODE = 0x30,
                NEGATE_INT = 0x31,
                NEGATE_FLOAT = 0x32,
                NEGATE_VEC2 = 0x33,
                NEGATE_VEC3 = 0x34,
                NEGATE_VEC4 = 0x35,
                ADD_INT = 0x36,
                ADD_FLOAT = 0x37,
                ADD_STRING = 0x38,
                ADD_VEC2 = 0x39,
                ADD_VEC3 = 0x3A,
                ADD_VEC3WS_VEC3 = 0x3B,
                ADD_VEC3_VEC3WS = 0x3C,
                ADD_VEC4 = 0x3D,
                ADD_GAMETIME_FLOAT = 0x3E,
                ADD_FLOAT_GAMETIME = 0x3F,
                SUB_INT = 0x40,
                SUB_FLOAT = 0x41,
                SUB_VEC2 = 0x42,
                SUB_VEC3 = 0x43,
                SUB_VEC3WS_VEC3 = 0x44,
                SUB_VEC3WS_VEC3WS = 0x45,
                SUB_VEC4 = 0x46,
                SUB_GAMETIME_FLOAT = 0x47,
                SUB_GAMETIME = 0x48,
                MUL_INT = 0x49,
                MUL_FLOAT = 0x4A,
                DIV_FLOAT = 0x4B,
                MOD_INT = 0x4C,
                MOD_FLOAT = 0x4D,
                LT_INT = 0x4E,
                LT_FLOAT = 0x4F,
                LT_GAMETIME = 0x50,
                LTE_INT = 0x51,
                LTE_FLOAT = 0x52,
                LTE_GAMETIME = 0x53,
                EQ_BOOL = 0x54,
                EQ_INT = 0x55,
                EQ_FLOAT = 0x56,
                EQ_VEC2 = 0x57,
                EQ_VEC3 = 0x58,
                EQ_VEC3WS = 0x59,
                EQ_VEC4 = 0x5A,
                EQ_STRING = 0x5B,
                EQ_ENTITY_NAME = 0x5C,
                EQ_ENUM = 0x5D,
                EQ_EHANDLE = 0x5E,
                EQ_PANEL_HANDLE = 0x5F,
                EQ_LEAF = 0x60,
                EQ_COLOR_RGB = 0x61,
                EQ_ARRAY = 0x62,
                EQ_GAMETIME = 0x63,
                NE_BOOL = 0x64,
                NE_INT = 0x65,
                NE_FLOAT = 0x66,
                NE_VEC2 = 0x67,
                NE_VEC3 = 0x68,
                NE_VEC3WS = 0x69,
                NE_VEC4 = 0x6A,
                NE_STRING = 0x6B,
                NE_ENTITY_NAME = 0x6C,
                NE_ENUM = 0x6D,
                NE_EHANDLE = 0x6E,
                NE_PANEL_HANDLE = 0x6F,
                NE_LEAF = 0x70,
                NE_COLOR_RGB = 0x71,
                NE_ARRAY = 0x72,
                NE_GAMETIME = 0x73,
                SCALE_VEC3 = 0x74,
                SCALE_VEC2 = 0x75,
                SCALE_VEC4 = 0x76,
                SCALE_INV_VEC3 = 0x77,
                SCALE_INV_VEC2 = 0x78,
                SCALE_INV_VEC4 = 0x79,
                ELEMENT_ACCESS_VEC2 = 0x7A,
                ELEMENT_ACCESS_VEC3 = 0x7B,
                ELEMENT_ACCESS_VEC3WS = 0x7C,
                ELEMENT_ACCESS_VEC4 = 0x7D,
                ELEMENT_ACCESS_COLOR_RGB = 0x7E,
                GET_CONST_INLINE_STORAGE = 0x7F
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseDomainValueType_t = enum(u32) {
                INVALID = 0xFFFFFFFF,
                ENTITY_NAME = 0x0,
                PANEL_ID = 0x1,
                COUNT = 0x2
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Step_TestDomainDestroyFakeEntity = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub const CPulseCell_WaitForCursorsWithTag = struct {
                pub const m_bTagSelfWhenComplete: usize = 0x128; // bool
                pub const m_nDesiredKillPriority: usize = 0x12C; // PulseCursorCancelPriority_t
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_RaceCursors = struct {
                pub const m_Outflows: usize = 0xD8; // CUtlVector<CPulse_OutflowConnection>
                pub const m_OnFinished: usize = 0xF0; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_NoInflow = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain_FakeEntityOwner = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Base = struct {
                pub const m_nEditorNodeID: usize = 0x8; // PulseDocNodeID_t
            };
            // Parent: None
            // Field count: 0
            pub const CPulse_ResumePoint = struct {
            };
            // Parent: None
            // Field count: 2
            pub const CTestDomainDerived_Cursor = struct {
                pub const m_nCursorValueA: usize = 0xD8; // int32
                pub const m_nCursorValueB: usize = 0xDC; // int32
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_PickBestOutflowSelector = struct {
                pub const m_nCheckType: usize = 0x48; // PulseBestOutflowRules_t
                pub const m_OutflowList: usize = 0x50; // PulseSelectorOutflowList_t
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorHeaderIcon
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_WaitForObservable = struct {
                pub const m_Condition: usize = 0xD8; // CPulseObservableExpression<bool>
                pub const m_OnTrue: usize = 0x168; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 4
            pub const CPulse_OutflowConnection = struct {
                pub const m_SourceOutflowName: usize = 0x0; // PulseSymbol_t
                pub const m_nDestChunk: usize = 0x10; // PulseRuntimeChunkIndex_t
                pub const m_nInstruction: usize = 0x14; // int32
                pub const m_OutflowRegisterMap: usize = 0x18; // PulseRegisterMap_t
            };
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseGraphDef = struct {
                pub const m_DomainIdentifier: usize = 0x8; // PulseSymbol_t
                pub const m_DomainSubType: usize = 0x18; // CPulseType
                pub const m_ParentMapName: usize = 0x30; // PulseSymbol_t
                pub const m_ParentXmlName: usize = 0x40; // PulseSymbol_t
                pub const m_Chunks: usize = 0x50; // CUtlVector<CPulse_Chunk*>
                pub const m_Cells: usize = 0x68; // CUtlVector<CPulseCell_Base*>
                pub const m_Vars: usize = 0x80; // CUtlVector<CPulse_Variable>
                pub const m_TempVarBanks: usize = 0x98; // CUtlVector<CPulse_TempVarBankDefinition*>
                pub const m_PublicOutputs: usize = 0xB0; // CUtlVector<CPulse_PublicOutput>
                pub const m_InvokeBindings: usize = 0xC8; // CUtlVector<CPulse_InvokeBinding*>
                pub const m_CallInfos: usize = 0xE0; // CUtlVector<CPulse_CallInfo*>
                pub const m_Constants: usize = 0xF8; // CUtlVector<CPulse_Constant>
                pub const m_DomainValues: usize = 0x110; // CUtlVector<CPulse_DomainValue>
                pub const m_BlackboardReferences: usize = 0x128; // CUtlVector<CPulse_BlackboardReference>
                pub const m_OutputConnections: usize = 0x140; // CUtlVector<CPulse_OutputConnection*>
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_TestYieldForever = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_TestWaitWithCursorState__InstanceState_t = struct {
                pub const m_nDummy: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_FireCursors = struct {
                pub const m_Outflows: usize = 0xD8; // CUtlVector<CPulse_OutflowConnection>
                pub const m_bWaitForChildOutflows: usize = 0xF0; // bool
                pub const m_OnFinished: usize = 0xF8; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Timeline__TimelineEvent_t = struct {
                pub const m_flTimeFromPrevious: usize = 0x0; // float32
                pub const m_EventOutflow: usize = 0x8; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_IntervalTimer__CursorState_t = struct {
                pub const m_StartTime: usize = 0x0; // GameTime_t
                pub const m_EndTime: usize = 0x4; // GameTime_t
                pub const m_flWaitInterval: usize = 0x8; // float32
                pub const m_flWaitIntervalHigh: usize = 0xC; // float32
                pub const m_bCompleteOnNextWake: usize = 0x10; // bool
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_BaseRequirement = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorHeaderIcon
            pub const CPulseCell_BaseState = struct {
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const OutflowWithRequirements_t = struct {
                pub const m_Connection: usize = 0x0; // CPulse_OutflowConnection
                pub const m_DestinationFlowNodeID: usize = 0x48; // PulseDocNodeID_t
                pub const m_RequirementNodeIDs: usize = 0x50; // CUtlVector<PulseDocNodeID_t>
                pub const m_nCursorStateBlockIndex: usize = 0x68; // CUtlVector<int32>
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_IsRequirementValid = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Value_Gradient = struct {
                pub const m_Gradient: usize = 0x48; // CColorGradient
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_TestWaitWithAutoTracepoints = struct {
                pub const m_TracePrefix: usize = 0xD8; // CUtlString
                pub const m_WakeResume: usize = 0xE0; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseNodeDynamicOutflows_t__DynamicOutflow_t = struct {
                pub const m_OutflowID: usize = 0x0; // CGlobalSymbol
                pub const m_Connection: usize = 0x8; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_MultiOutflow_WithParams = struct {
                pub const m_Out1: usize = 0x48; // SignatureOutflow_Continue
                pub const m_Out2: usize = 0x90; // SignatureOutflow_Continue
            };
            // Parent: None
            // Field count: 0
            pub const CBasePulseGraphInstance = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_GraphHook = struct {
                pub const m_HookName: usize = 0x80; // PulseSymbol_t
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Resume = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t = struct {
                pub const nTestStep: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 4
            pub const CPulseTurtleGraphicsCursor = struct {
                pub const m_Color: usize = 0xD8; // Color
                pub const m_vPos: usize = 0xDC; // Vector2D
                pub const m_flHeadingDeg: usize = 0xE4; // float32
                pub const m_bPenUp: usize = 0xE8; // bool
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_TestWaitWithCursorState__CursorState_t = struct {
                pub const flWaitValue: usize = 0x0; // float32
                pub const bFail: usize = 0x4; // bool
                pub const m_hSelfCursor: usize = 0x8; // HYieldedCursor
                pub const m_hSelfCellInstanceUntyped: usize = 0x14; // HPulseCellBase
                pub const m_hSelfCellInstance: usize = 0x1C; // HPulseCell<CPulseCell_TestWaitWithCursorState>
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_BaseEntrypoint = struct {
                pub const m_EntryChunk: usize = 0x48; // PulseRuntimeChunkIndex_t
                pub const m_RegisterMap: usize = 0x50; // PulseRegisterMap_t
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_MultiInflow_NoDefault = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_WaitForCursorsWithTagBase = struct {
                pub const m_nCursorsAllowedToWait: usize = 0xD8; // int32
                pub const m_WaitComplete: usize = 0xE0; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_InvokeBinding = struct {
                pub const m_RegisterMap: usize = 0x0; // PulseRegisterMap_t
                pub const m_FuncName: usize = 0x30; // PulseSymbol_t
                pub const m_nCellIndex: usize = 0x40; // PulseRuntimeCellIndex_t
                pub const m_nSrcChunk: usize = 0x44; // PulseRuntimeChunkIndex_t
                pub const m_nSrcInstruction: usize = 0x48; // int32
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_IntervalTimer = struct {
                pub const m_Completed: usize = 0xD8; // CPulse_ResumePoint
                pub const m_OnInterval: usize = 0x120; // SignatureOutflow_Continue
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_BaseLerp = struct {
                pub const m_WakeResume: usize = 0xD8; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_Value_TestValue50 = struct {
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_MultiOutflow_WithParams_Yielding = struct {
                pub const m_Out1: usize = 0xD8; // SignatureOutflow_Continue
                pub const m_AsyncChild1: usize = 0x120; // SignatureOutflow_Continue
                pub const m_AsyncChild2: usize = 0x168; // SignatureOutflow_Continue
                pub const m_YieldResume1: usize = 0x1B0; // SignatureOutflow_Resume
                pub const m_YieldResume2: usize = 0x1F8; // SignatureOutflow_Resume
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Value_Curve = struct {
                pub const m_Curve: usize = 0x48; // CPiecewiseCurve
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_EventHandler = struct {
                pub const m_EventName: usize = 0x80; // PulseSymbol_t
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_BaseFlow = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Step_TestDomainTracepoint = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Outflow_CycleShuffled__InstanceState_t = struct {
                pub const m_Shuffle: usize = 0x0; // CUtlVectorFixedGrowable<uint8,8>
                pub const m_nNextShuffle: usize = 0x20; // int32
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_BaseLerp__CursorState_t = struct {
                pub const m_StartTime: usize = 0x0; // GameTime_t
                pub const m_EndTime: usize = 0x4; // GameTime_t
            };
            // Parent: None
            // Field count: 1
            pub const CPulseGraphInstance_TestDomain_Derived = struct {
                pub const m_nInstanceValueX: usize = 0xD8; // int32
            };
            // Parent: None
            // Field count: 9
            pub const CPulseGraphInstance_TestDomain = struct {
                pub const m_bIsRunningUnitTests: usize = 0xA8; // bool
                pub const m_bExplicitTimeStepping: usize = 0xA9; // bool
                pub const m_bExpectingToDestroyWithYieldedCursors: usize = 0xAA; // bool
                pub const m_bQuietTracepoints: usize = 0xAB; // bool
                pub const m_bExpectingCursorTerminatedDueToMaxInstructions: usize = 0xAC; // bool
                pub const m_nCursorsTerminatedDueToMaxInstructions: usize = 0xB0; // int32
                pub const m_nNextValidateIndex: usize = 0xB4; // int32
                pub const m_Tracepoints: usize = 0xB8; // CUtlVector<CUtlString>
                pub const m_bTestYesOrNoPath: usize = 0xD0; // bool
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Continue = struct {
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Timeline = struct {
                pub const m_TimelineEvents: usize = 0xD8; // CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
                pub const m_bWaitForChildOutflows: usize = 0xF0; // bool
                pub const m_OnFinished: usize = 0xF8; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_EntOutputHandler = struct {
                pub const m_SourceEntity: usize = 0x80; // PulseSymbol_t
                pub const m_SourceOutput: usize = 0x90; // PulseSymbol_t
                pub const m_ExpectedParamType: usize = 0xA0; // CPulseType
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_Outflow_TestExplicitYesNo = struct {
                pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
                pub const m_No: usize = 0x90; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_Outflow_TestRandomYesNo = struct {
                pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
                pub const m_No: usize = 0x90; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Outflow_CycleOrdered__InstanceState_t = struct {
                pub const m_nNextIndex: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_LimitCount__InstanceState_t = struct {
                pub const m_nCurrentCount: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Test_MultiInflow_WithDefault = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Step_DebugLog = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCustomFGDMetadata
            pub const CPulseCell_BaseYieldingInflow = struct {
                pub const m_BaseFlow_OnAfterCancel: usize = 0x48; // CPulse_ResumePoint
                pub const m_BaseFlow_WhileActive: usize = 0x90; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseNodeDynamicOutflows_t = struct {
                pub const m_Outflows: usize = 0x0; // CUtlVector<PulseNodeDynamicOutflows_t::DynamicOutflow_t>
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_IsRequirementValid__Criteria_t = struct {
                pub const m_bIsValid: usize = 0x0; // bool
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_ObservableVariableListener = struct {
                pub const m_nBlackboardReference: usize = 0x80; // PulseRuntimeBlackboardReferenceIndex_t
                pub const m_bSelfReference: usize = 0x82; // bool
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_ObservableSwitchState = struct {
                pub const m_SwitchValue: usize = 0xD8; // CPulseObservableExpression<CPulseVariant>
                pub const m_Cases: usize = 0x168; // CPulseObservableSwitchCases
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Outflow_CycleOrdered = struct {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseSelectorOutflowList_t = struct {
                pub const m_Outflows: usize = 0x0; // CUtlVector<OutflowWithRequirements_t>
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TurtleGraphics = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Val_TestDomainGetEntityName = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_Inflow_Wait = struct {
                pub const m_WakeResume: usize = 0xD8; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_TestWaitWithCursorState = struct {
                pub const m_WakeResume: usize = 0xD8; // CPulse_ResumePoint
                pub const m_WakeFail: usize = 0x120; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Outflow_CycleShuffled = struct {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_Method = struct {
                pub const m_MethodName: usize = 0x80; // PulseSymbol_t
                pub const m_Description: usize = 0x90; // CUtlString
                pub const m_bIsPublic: usize = 0x98; // bool
                pub const m_Args: usize = 0xA0; // CUtlLeanVector<CPulseRuntimeMethodArg>
                pub const m_ReturnValues: usize = 0xB0; // CUtlLeanVector<CPulseRuntimeMethodArg>
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_BaseValue = struct {
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_BooleanSwitchState = struct {
                pub const m_Condition: usize = 0xD8; // CPulseObservableExpression<bool>
                pub const m_WhenTrue: usize = 0x168; // CPulse_OutflowConnection
                pub const m_WhenFalse: usize = 0x1B0; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Inflow_Yield = struct {
                pub const m_UnyieldResume: usize = 0xD8; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_Unknown = struct {
                pub const m_UnknownKeys: usize = 0x48; // KeyValues3
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Outflow_CycleRandom = struct {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Step_PublicOutput = struct {
                pub const m_OutputIndex: usize = 0x48; // PulseRuntimeOutputIndex_t
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Val_TestDomainFindEntityByName = struct {
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_BlackboardReference = struct {
                pub const m_hBlackboardResource: usize = 0x0; // CStrongHandle<InfoForResourceTypeIPulseGraphDef>
                pub const m_BlackboardResource: usize = 0x8; // PulseSymbol_t
                pub const m_nNodeID: usize = 0x18; // PulseDocNodeID_t
                pub const m_NodeName: usize = 0x20; // CGlobalSymbol
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_ReturnValues = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub const CPulseCell_Value_RandomInt = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_TestEnums = struct {
                pub const m_nReferenceColor: usize = 0x48; // PulseTestEnumColor_t
                pub const m_nReferenceFlags: usize = 0x4C; // PulseTestEnumFlags_t
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPulseEditorHeaderText
            pub const CPulseCell_Step_TestDomainEntFire = struct {
                pub const m_Input: usize = 0x48; // CUtlString
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            // MPulseEditorCanvasItemSpecKV3
            pub const CPulseCell_ExampleSelector = struct {
                pub const m_OutflowList: usize = 0x48; // PulseSelectorOutflowList_t
            };
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_CallInfo = struct {
                pub const m_PortName: usize = 0x0; // PulseSymbol_t
                pub const m_nEditorNodeID: usize = 0x10; // PulseDocNodeID_t
                pub const m_RegisterMap: usize = 0x18; // PulseRegisterMap_t
                pub const m_CallMethodID: usize = 0x48; // PulseDocNodeID_t
                pub const m_nSrcChunk: usize = 0x4C; // PulseRuntimeChunkIndex_t
                pub const m_nSrcInstruction: usize = 0x50; // int32
                pub const m_nBreakDestChunk: usize = 0x54; // PulseRuntimeChunkIndex_t
                pub const m_nBreakDestInstruction: usize = 0x58; // int32
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_InlineNodeSkipSelector = struct {
                pub const m_nFlowNodeID: usize = 0x48; // PulseDocNodeID_t
                pub const m_bAnd: usize = 0x4C; // bool
                pub const m_PassOutflow: usize = 0x50; // PulseSelectorOutflowList_t
                pub const m_FailOutflow: usize = 0x68; // CPulse_OutflowConnection
            };
            // Parent: None
            // Field count: 3
            pub const CPulseCell_ExampleCriteria__Criteria_t = struct {
                pub const m_flFloatValue1: usize = 0x0; // float32
                pub const m_flFloatValue2: usize = 0x4; // float32
                pub const m_bMyBool: usize = 0x8; // bool
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_ExampleCriteria = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            pub const CPulseCell_LimitCount = struct {
                pub const m_nLimitCount: usize = 0x48; // int32
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_Step_CallExternalMethod = struct {
                pub const m_MethodName: usize = 0xD8; // PulseSymbol_t
                pub const m_nBlackboardIndex: usize = 0xE8; // PulseRuntimeBlackboardReferenceIndex_t
                pub const m_ExpectedArgs: usize = 0xF0; // CUtlLeanVector<CPulseRuntimeMethodArg>
                pub const m_nAsyncCallMode: usize = 0x100; // PulseMethodCallMode_t
                pub const m_OnFinished: usize = 0x108; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_LimitCount__Criteria_t = struct {
                pub const m_bLimitCountPasses: usize = 0x0; // bool
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub const CPulseCell_Step_TestDomainCreateFakeEntity = struct {
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseCell_TestYieldWithObservables = struct {
                pub const m_flWatchForFloatValue: usize = 0xD8; // float32
                pub const m_LiveFloatValue: usize = 0xE0; // CPulseObservableExpression<float32>
                pub const m_WatchForStringValue: usize = 0x170; // CUtlString
                pub const m_LiveStringValue: usize = 0x178; // CPulseObservableExpression<CUtlString>
                pub const m_WakeResume: usize = 0x208; // CPulse_ResumePoint
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub const CPulseCell_CursorQueue = struct {
                pub const m_nCursorsAllowedToRunParallel: usize = 0x128; // int32
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub const CPulseCell_Value_RandomFloat = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseExecCursor = struct {
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeVarIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeEntrypointIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeDomainValueIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_Chunk = struct {
                pub const m_Instructions: usize = 0x0; // CUtlLeanVector<PGDInstruction_t>
                pub const m_Registers: usize = 0x10; // CUtlLeanVector<CPulse_RegisterInfo>
                pub const m_InstructionDebugInfos: usize = 0x20; // CUtlLeanVector<CPulse_InstructionDebug>
                pub const m_nTempVarBank: usize = 0x30; // PulseRuntimeTempVarBankIndex_t
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseBreakpointLocation = struct {
                pub const m_NodeID: usize = 0x0; // PulseDocNodeID_t
                pub const m_SequencePoint: usize = 0x8; // PulseSymbol_t
                pub const m_PortName: usize = 0x18; // PulseSymbol_t
                pub const m_bDeferBreak: usize = 0x28; // bool
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeInvokeIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeRegisterIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 1
            pub const PulseCursorID_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseGraphExecutionHistoryCursorDesc_t = struct {
                pub const vecAncestorCursorIDs: usize = 0x0; // CUtlVector<PulseCursorID_t>
                pub const nSpawnNodeID: usize = 0x18; // PulseDocNodeID_t
                pub const nRetiredAtNodeID: usize = 0x1C; // PulseDocNodeID_t
                pub const flLastReferenced: usize = 0x20; // float32
                pub const nLastValidEntryIdx: usize = 0x24; // int32
                pub const bWasAnObservableComputation: usize = 0x28; // bool
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeTempVarIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 1
            pub const TestComponent_t = struct {
                pub const m_ComponentData: usize = 0x8; // CUtlString
            };
            // Parent: None
            // Field count: 3
            pub const PulseRegisterMap_t = struct {
                pub const m_Inparams: usize = 0x0; // KeyValues3
                pub const m_InparamsWhichCanBeMoved: usize = 0x10; // CKV3MemberNameSet
                pub const m_Outparams: usize = 0x20; // KeyValues3
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_TempVarBankDefinition = struct {
                pub const m_TempVars: usize = 0x0; // CUtlVector<CPulse_TempVarInfo>
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_PublicOutput = struct {
                pub const m_Name: usize = 0x0; // PulseSymbol_t
                pub const m_Description: usize = 0x10; // CUtlString
                pub const m_Args: usize = 0x18; // CUtlLeanVector<CPulseRuntimeMethodArg>
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_OutputConnection = struct {
                pub const m_SourceOutput: usize = 0x0; // PulseSymbol_t
                pub const m_TargetEntity: usize = 0x10; // PulseSymbol_t
                pub const m_TargetInput: usize = 0x20; // PulseSymbol_t
                pub const m_Param: usize = 0x30; // PulseSymbol_t
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseGraphExecutionHistoryNodeDesc_t = struct {
                pub const strCellDesc: usize = 0x0; // CBufferString
                pub const strBindingName: usize = 0x10; // PulseSymbol_t
            };
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PulseGraphExecutionHistoryEntry_t = struct {
                pub const nCursorID: usize = 0x0; // PulseCursorID_t
                pub const nEditorID: usize = 0x4; // PulseDocNodeID_t
                pub const seqPoint: usize = 0x8; // PulseSymbol_t
                pub const flExecTime: usize = 0x18; // float32
                pub const unFlags: usize = 0x1C; // uint32
                pub const tagName: usize = 0x20; // PulseSymbol_t
                pub const childID: usize = 0x30; // PulseCursorID_t
            };
            // Parent: None
            // Field count: 1
            pub const PulseCursorYieldToken_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseGraphExecutionHistory = struct {
                pub const m_nInstanceID: usize = 0x0; // PulseGraphInstanceID_t
                pub const m_strFileName: usize = 0x8; // CUtlString
                pub const m_vecHistory: usize = 0x10; // CUtlVector<PulseGraphExecutionHistoryEntry_t*>
                pub const m_mapCellDesc: usize = 0x28; // CUtlOrderedMap<PulseDocNodeID_t,PulseGraphExecutionHistoryNodeDesc_t*>
                pub const m_mapCursorDesc: usize = 0x50; // CUtlOrderedMap<PulseCursorID_t,PulseGraphExecutionHistoryCursorDesc_t*>
            };
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const PGDInstruction_t = struct {
                pub const m_nCode: usize = 0x0; // PulseInstructionCode_t
                pub const m_nVar: usize = 0x4; // PulseRuntimeVarIndex_t
                pub const m_nReg0: usize = 0x8; // PulseRuntimeRegisterIndex_t
                pub const m_nReg1: usize = 0xA; // PulseRuntimeRegisterIndex_t
                pub const m_nReg2: usize = 0xC; // PulseRuntimeRegisterIndex_t
                pub const m_nInvokeBindingIndex: usize = 0x10; // PulseRuntimeInvokeIndex_t
                pub const m_nChunk: usize = 0x14; // PulseRuntimeChunkIndex_t
                pub const m_nDestInstruction: usize = 0x18; // int32
                pub const m_nCallInfoIndex: usize = 0x1C; // PulseRuntimeCallInfoIndex_t
                pub const m_nConstIdx: usize = 0x20; // PulseRuntimeConstantIndex_t
                pub const m_nDomainValueIdx: usize = 0x22; // PulseRuntimeDomainValueIndex_t
                pub const m_nBlackboardReferenceIdx: usize = 0x24; // PulseRuntimeBlackboardReferenceIndex_t
                pub const m_nTempVarIdx: usize = 0x26; // PulseRuntimeTempVarIndex_t
            };
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_Variable = struct {
                pub const m_Name: usize = 0x0; // PulseSymbol_t
                pub const m_Description: usize = 0x10; // CUtlString
                pub const m_Type: usize = 0x18; // CPulseType
                pub const m_DefaultValue: usize = 0x30; // KeyValues3
                pub const m_nKeysSource: usize = 0x44; // PulseVariableKeysSource_t
                pub const m_bIsPublicBlackboardVariable: usize = 0x48; // bool
                pub const m_bIsObservable: usize = 0x49; // bool
                pub const m_nEditorNodeID: usize = 0x4C; // PulseDocNodeID_t
                pub const m_Metadata: usize = 0x50; // KeyValues3
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeOutputIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_DomainValue = struct {
                pub const m_nType: usize = 0x0; // PulseDomainValueType_t
                pub const m_Value: usize = 0x8; // CGlobalSymbolCaseSensitive
                pub const m_RequiredRuntimeType: usize = 0x10; // CPulseType
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeBlackboardReferenceIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_TempVarInfo = struct {
                pub const m_Name: usize = 0x0; // PulseSymbol_t
                pub const m_Type: usize = 0x10; // CPulseType
                pub const m_nEditorNodeID: usize = 0x28; // PulseDocNodeID_t
                pub const m_bIsObservable: usize = 0x2C; // bool
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeConstantIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 1
            pub const PulseDocNodeID_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_RegisterInfo = struct {
                pub const m_nReg: usize = 0x0; // PulseRuntimeRegisterIndex_t
                pub const m_Type: usize = 0x8; // CPulseType
                pub const m_OriginName: usize = 0x20; // CKV3MemberNameWithStorage
                pub const m_nWrittenByInstruction: usize = 0x58; // int32
                pub const m_nLastReadByInstruction: usize = 0x5C; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeCellIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeTempVarBankIndex_t = struct {
                pub const m_Value: usize = 0x0; // int16
            };
            // Parent: None
            // Field count: 1
            pub const PulseGraphInstanceID_t = struct {
                pub const m_Value: usize = 0x0; // uint32
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_Constant = struct {
                pub const m_Type: usize = 0x0; // CPulseType
                pub const m_Value: usize = 0x18; // KeyValues3
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeCallInfoIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeChunkIndex_t = struct {
                pub const m_Value: usize = 0x0; // int32
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulseRuntimeMethodArg = struct {
                pub const m_Name: usize = 0x0; // CKV3MemberNameWithStorage
                pub const m_Description: usize = 0x38; // CUtlString
                pub const m_Type: usize = 0x40; // CPulseType
            };
            // Parent: None
            // Field count: 1
            pub const PulseRuntimeStateOffset_t = struct {
                pub const m_Value: usize = 0x0; // uint16
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CPulse_InstructionDebug = struct {
                pub const m_nFlowNodeID: usize = 0x0; // PulseDocNodeID_t
                pub const m_nValueNodeID: usize = 0x4; // PulseDocNodeID_t
                pub const m_SequencePointName: usize = 0x8; // PulseSymbol_t
            };
        };
    };
};
