#pragma once
#include <Windows.h>
#include <fmt/format.h>
#include <string>
#include <string_view>

class AppError : public std::exception {
public:
  AppError(std::string_view message, uint32_t status = GetLastError())
      : m_message(message), m_status(status),
        m_full_message(fmt::format("{}, code: {:X}", m_message, m_status)) {}

  inline const char *what() const override {
    if (m_status == ERROR_SUCCESS) {
      return m_message.c_str();
    } else {
      return m_full_message.c_str();
    }
  }

private:
  std::string m_message;
  std::string m_full_message;
  uint32_t m_status;
};
