#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Server::Server(const std::string &_base_url)
    : base_url(fmt::format("{}/server", normalizeUrl(_base_url))) {}

Server::Detail Server::getDetail() {
  auto res = cpr::Get(cpr::Url(base_url));
  return Detail::fromJson(getBodyJson(res));
}

Server::Detail Server::Detail::fromJson(nlohmann::json json) {
  return Detail{
      json["version"],
      json["build_time"],
      json["commit_hash"],
  };
}
} // namespace m0n1t0r
