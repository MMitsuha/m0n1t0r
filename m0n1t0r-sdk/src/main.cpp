#include "common.h"
#include <cpr/cpr.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
std::string normalizeUrl(std::string url) {
  auto prefix = url.find("://");
  if (prefix != std::string::npos) {
    url.erase(0, prefix + 3);
  }
  auto suffix = url.find_last_of("/");
  if (suffix != std::string::npos) {
    url.erase(url.find_last_of("/"));
  }
  return url;
}

nlohmann::json getBodyJson(const cpr::Response &res) {
  if (res.error) {
    auto message = fmt::format("Failed send request: {}", res.error.message);
    spdlog::error(message);
    throw std::runtime_error(message);
  }

  auto json = json::parse(res.text);

  if (json["code"] != 0) {
    auto message = fmt::format("Failed to get JSON: {}",
                               static_cast<std::string>(json["body"]));
    spdlog::error(message);
    throw std::runtime_error(message);
  }

  return json["body"];
}
} // namespace m0n1t0r
