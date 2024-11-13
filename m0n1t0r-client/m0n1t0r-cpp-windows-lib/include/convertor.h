#pragma once
#include "cxx.h"
#include <Windows.h>
#include <optional>
#include <string>

auto to_wstring(rust::String &string) -> std::optional<std::wstring>;
