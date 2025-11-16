#pragma once
#include <Windows.h>
#include <string>

extern std::string
    *g_key; // Global variable holding the key for the XOR encrypted payload.
extern volatile LONG g_voidgate_lock;
