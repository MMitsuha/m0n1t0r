#pragma once
#include "cxx.h"
#include <Windows.h>
#include <string>

std::wstring utf8_to_wstring(rust::String string);
rust::String acp_to_utf8(rust::Vec<uint8_t> string);
uint32_t acp();
