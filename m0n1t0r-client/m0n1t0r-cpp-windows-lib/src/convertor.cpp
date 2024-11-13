#include "convertor.h"

auto to_wstring(rust::String &string) -> std::optional<std::wstring> {
  auto ptr = string.c_str();
  auto wide_len = MultiByteToWideChar(CP_UTF8, MB_COMPOSITE, ptr, string.size(),
                                      nullptr, 0);
  std::wstring wide_string(wide_len, 0);
  MultiByteToWideChar(CP_UTF8, MB_COMPOSITE, ptr, string.size(),
                      wide_string.data(), wide_len);
  return wide_string;
}
