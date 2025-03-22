#pragma once
#include "cxx.h"
#include <Windows.h>

struct Output;
struct InteractiveContext;

Output execute(rust::String command, rust::Vec<rust::String> args);
std::tuple<void * /*remote_shellcode*/, void * /*remote_parameter*/>
write_process_memory(HANDLE process, rust::Vec<rust::u8> shellcode,
                     rust::Vec<rust::u8> parameter);
void inject_shellcode_by_id_rtc(rust::u32 pid, rust::Vec<rust::u8> shellcode,
                                rust::u32 ep_offset,
                                rust::Vec<rust::u8> parameter);
void inject_shellcode_by_id_apc(rust::u32 pid, rust::Vec<rust::u8> shellcode,
                                rust::u32 ep_offset,
                                rust::Vec<rust::u8> parameter);
rust::u32 id_by_name(rust::String name);
