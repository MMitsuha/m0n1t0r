#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
void Client::download(const std::string &url, const std::string &path) {
  auto res = cpr::Post(cpr::Url(fmt::format("{}/network/download", base_url)),
                       cpr::Payload{{"url", url}, {"path", path}});
  auto json = getBodyJson(res);
}
} // namespace m0n1t0r
