#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Server::Proxy Server::Proxy::fromJson(nlohmann::json json) {
  return Server::Proxy{
      json["addr"],
      json["type"],
  };
}

std::vector<Server::Proxy> Server::listProxies() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/proxy", base_url)));
  auto array = getBodyJson(res);
  auto ret = std::vector<Server::Proxy>();

  for (auto &file : array) {
    ret.emplace_back(Proxy::fromJson(file));
  }
  return ret;
}

void Server::deleteProxy(const std::string &addr) {
  auto res = cpr::Delete(cpr::Url(fmt::format("{}/proxy/{}", base_url, addr)));
  auto json = getBodyJson(res);
}
} // namespace m0n1t0r
