#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
void Client::terminate() {
  auto res = cpr::Post(cpr::Url(fmt::format("{}/terminate", base_url)));
  auto json = getBodyJson(res);
}
} // namespace m0n1t0r
