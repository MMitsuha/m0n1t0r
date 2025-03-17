#pragma once
#include "cxx.h"
#include <Windows.h>
#include <string>

auto utf8_to_wstring(rust::String &string) -> std::wstring;
auto acp_to_utf8(rust::Vec<uint8_t> const &string) -> rust::String;
auto acp() -> uint32_t;
