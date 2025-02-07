#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Client::SystemInfo Client::SystemInfo::fromJson(nlohmann::json json) {
  return SystemInfo{
      json["boot_time"], json["cpu_arch"],       json["distribution_id"],
      json["host_name"], json["kernel_version"], json["long_os_version"],
      json["name"],      json["uptime"],
  };
}

Client::SystemInfo Client::getSystemInfo() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/info/system", base_url)));
  auto json = getBodyJson(res);
  return SystemInfo::fromJson(json);
}
} // namespace m0n1t0r
