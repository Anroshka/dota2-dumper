// Generated using https://github.com/Anroshka/dota2-dumper
// 2026-07-25 01:27:41.234350800 UTC

#pragma once

#include <cstddef>
#include <cstdint>

namespace dota2_dumper {
    namespace offsets {
        // Module: client.dll
        namespace client_dll {
            constexpr std::ptrdiff_t dwEntityList = 0x64A5C50;
            constexpr std::ptrdiff_t dwGameEntitySystem = 0x64A5C50;
            constexpr std::ptrdiff_t dwGameEntitySystem_highestEntityIndex = 0x2090;
            constexpr std::ptrdiff_t dwGameRules = 0x6128870;
            constexpr std::ptrdiff_t dwGlobalVars = 0x59FCFD8;
            constexpr std::ptrdiff_t dwViewMatrix = 0x612E1F0;
            constexpr std::ptrdiff_t dwViewRender = 0x612D8B8;
        }
        // Module: engine2.dll
        namespace engine2_dll {
            constexpr std::ptrdiff_t dwBuildNumber = 0x60DD24;
            constexpr std::ptrdiff_t dwNetworkGameClient = 0x90BBC0;
            constexpr std::ptrdiff_t dwNetworkGameClient_clientTickCount = 0x378;
            constexpr std::ptrdiff_t dwNetworkGameClient_deltaTick = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_isBackgroundMap = 0x2C141F;
            constexpr std::ptrdiff_t dwNetworkGameClient_localPlayer = 0xF8;
            constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x240;
            constexpr std::ptrdiff_t dwNetworkGameClient_serverTickCount = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x230;
            constexpr std::ptrdiff_t dwWindowHeight = 0x90FF8C;
            constexpr std::ptrdiff_t dwWindowWidth = 0x90FF88;
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            constexpr std::ptrdiff_t dwInputSystem = 0x45BA0;
        }
        // Module: soundsystem.dll
        namespace soundsystem_dll {
            constexpr std::ptrdiff_t dwSoundSystem = 0x552AA0;
            constexpr std::ptrdiff_t dwSoundSystem_engineViewData = 0x7C;
        }
    }
}
