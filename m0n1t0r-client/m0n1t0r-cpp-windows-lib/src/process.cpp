#include "process.h"
#include "convertor.h"
#include "process.rs.h"
#include <Windows.h>

std::optional<std::tuple<HANDLE, HANDLE, HANDLE, HANDLE>> create_pipes() {
  SECURITY_ATTRIBUTES sa{};
  HANDLE stdout_rx, stdout_tx;
  HANDLE stderr_rx, stderr_tx;
  sa.nLength = sizeof(SECURITY_ATTRIBUTES);
  sa.bInheritHandle = true;
  sa.lpSecurityDescriptor = nullptr;

  if (CreatePipe(&stdout_rx, &stdout_tx, &sa, 0) == 0) {
    return {};
  }

  if (CreatePipe(&stderr_rx, &stderr_tx, &sa, 0) == 0) {
    CloseHandle(stdout_rx);
    CloseHandle(stdout_tx);
    return {};
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
  auto [stdout_rx, stdout_tx, stderr_rx, stderr_tx] = create_pipes().value();
  std::wstring command_line;
  STARTUPINFOW si{};
  PROCESS_INFORMATION pi{};
  Output output{};

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
    // error
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
