#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
void Client::update(const std::string &url, const std::string &temp) {
  auto res = cpr::Post(cpr::Url(fmt::format("{}/update/byurl", base_url)),
                       cpr::Payload{{"url", url}, {"temp", temp}});
  auto json = getBodyJson(res);
}

void Client::update(const std::vector<uint8_t> &file, const std::string &temp) {
  auto res = cpr::Post(
      cpr::Url(fmt::format("{}/update/byfile", base_url)),
      cpr::Multipart{cpr::Part{"file", cpr::Buffer(file.begin(), file.end(),
                                                   "m0n1t0r-client.exe")},
                     cpr::Part{"temp", temp}});
  auto json = getBodyJson(res);
}
} // namespace m0n1t0r
