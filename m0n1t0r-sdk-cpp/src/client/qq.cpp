#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Client::QQAccount Client::QQAccount::fromJson(nlohmann::json json) {
  return QQAccount{
      json["uin"],
      json["face_index"],
      json["gender"],
      json["nickname"].is_null() ? "[unknown]" : json["nickname"],
      json["client_type"],
      json["uin_flag"],
      json["account"],
  };
}

std::vector<Client::QQAccount> Client::listQQAccounts() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/qq", base_url)));
  auto array = getBodyJson(res);
  auto ret = std::vector<QQAccount>();

  for (auto &account : array) {
    ret.emplace_back(QQAccount::fromJson(account));
  }
  return ret;
}
} // namespace m0n1t0r
