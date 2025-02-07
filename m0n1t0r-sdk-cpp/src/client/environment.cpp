#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
std::unordered_map<std::string, std::string> Client::listEnvironments() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/environment", base_url)));
  auto map = getBodyJson(res);
  return map.get<std::unordered_map<std::string, std::string>>();
}
} // namespace m0n1t0r
