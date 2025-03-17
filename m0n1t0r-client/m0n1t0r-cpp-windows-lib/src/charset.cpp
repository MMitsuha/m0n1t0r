#include "charset.h"
#include "error.h"

auto utf8_to_wstring(rust::String &string) -> std::wstring {
  auto ptr = string.c_str();
  auto wide_len =
      MultiByteToWideChar(CP_UTF8, 0, ptr, string.size(), nullptr, 0);
  if (wide_len <= 0) {
    throw AppError("failed to get wide len");
  }
  std::wstring wide_string(wide_len, 0);
  if (MultiByteToWideChar(CP_UTF8, 0, ptr, string.size(), wide_string.data(),
                          wide_len) <= 0) {
    throw AppError("failed to convert utf8 to wide");
  }
  return wide_string;
}

auto acp_to_wstring(rust::Vec<uint8_t> const &string) -> std::wstring {
  auto ptr = (LPCCH)string.data();
  auto wide_len =
      MultiByteToWideChar(CP_ACP, 0, ptr, string.size(), nullptr, 0);
  if (wide_len <= 0) {
    throw AppError("failed to get wide len");
  }
  std::wstring wide_string(wide_len, 0);
  if (MultiByteToWideChar(CP_ACP, 0, ptr, string.size(), wide_string.data(),
                          wide_len) <= 0) {
    throw AppError("failed to convert acp to wide");
  }
  return wide_string;
}

auto wstring_to_utf8(std::wstring &wstring) -> rust::String {
  auto ptr = (LPCWCH)wstring.data();
  auto utf8_len = WideCharToMultiByte(CP_UTF8, 0, ptr, wstring.size(), nullptr,
                                      0, nullptr, nullptr);
  if (utf8_len <= 0) {
    throw AppError("failed to get utf8 len");
  }
  rust::String utf8_string("", utf8_len);
  if (WideCharToMultiByte(CP_UTF8, 0, ptr, wstring.size(),
                          (LPSTR)utf8_string.data(), utf8_len, nullptr,
                          nullptr) <= 0) {
    throw AppError("failed to convert wide to utf8");
  }
  return utf8_string;
}

auto acp_to_utf8(rust::Vec<uint8_t> const &string) -> rust::String {
  auto wstring = acp_to_wstring(string);
  return wstring_to_utf8(wstring);
}

auto acp() -> uint32_t { return GetACP(); }
