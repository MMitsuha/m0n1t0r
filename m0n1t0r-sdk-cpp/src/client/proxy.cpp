#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
std::string Client::proxySocks5() {
  auto res =
      cpr::Post(cpr::Url(fmt::format("{}/proxy/socks5/noauth", base_url)));
  auto json = getBodyJson(res);
  return json;
}

std::string Client::proxySocks5(const std::string &name,
                                const std::string &password) {
  auto res = cpr::Post(cpr::Url(fmt::format("{}/proxy/socks5/pass", base_url)),
                       cpr::Payload{{"name", name}, {"password", password}});
  auto json = getBodyJson(res);
  return json;
}
} // namespace m0n1t0r
