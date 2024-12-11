#pragma once
#include "cxx.h"
#include <optional>

struct Output;
struct InteractiveContext;

Output execute(rust::String command, rust::Vec<rust::String> args);
bool inject_shellcode_by_id(rust::u32 pid, rust::Vec<rust::u8> shellcode,
                            rust::u32 ep_offset, rust::Vec<rust::u8> parameter);
rust::u32 get_id_by_name(rust::String name);
