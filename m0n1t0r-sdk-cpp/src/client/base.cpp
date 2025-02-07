#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Client::Client(const std::string &_base_url, const std::string &_addr)
    : addr(_addr),
      base_url(fmt::format("{}/client/{}", normalizeUrl(_base_url), addr)) {}

Client::Detail Client::Detail::fromJson(nlohmann::json json) {
  return Detail{
      json["addr"],
      SystemInfo::fromJson(json["system_info"]),
      json["target_platform"],
      json["version"],
      json["build_time"],
      json["commit_hash"],
  };
}

Client::Notification Client::Notification::fromJson(nlohmann::json json) {
  return Notification{
      json["addr"],
      json["event"],
  };
}

Client::Detail Client::getDetail() {
  auto res = cpr::Get(cpr::Url(base_url));
  return Detail::fromJson(getBodyJson(res));
}

std::thread Client::notifyClose(std::function<void()> close) {
  return std::thread([=]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_close = [=](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_close_handler(on_close);

    ws_client::connection_ptr con =
        c.get_connection(fmt::format("ws://{}/notify", base_url), ec);

    if (ec) {
      auto message =
          fmt::format("Could not create connection because: {}", ec.message());
      spdlog::error(message);
      throw std::runtime_error(message);
    }
    c.connect(con);
    c.run();
  });
}

std::vector<std::shared_ptr<Client>> Client::all(const std::string &base_url) {
  auto ret = std::vector<std::shared_ptr<Client>>();
  auto res =
      cpr::Get(cpr::Url(fmt::format("{}/client", normalizeUrl(base_url))));
  auto json = getBodyJson(res);

  for (auto &detail : json) {
    ret.emplace_back(std::make_shared<Client>(base_url, detail["addr"]));
  }
  return ret;
}

std::vector<std::shared_ptr<Client>>
Client::all(const std::shared_ptr<Server> server) {
  return all(server->getBaseUrl());
}

std::thread Client::notify(const std::shared_ptr<Server> server,
                           std::function<bool(const Notification &)> callback) {
  return notify(server->getBaseUrl(), callback);
}

std::thread Client::notify(const std::string &base_url,
                           std::function<bool(const Notification &)> callback) {
  return std::thread([=]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_message = [=, &c](websocketpp::connection_hdl h,
                              ws_client::message_ptr msg) {
      auto json = json::parse(msg->get_payload());
      auto handle = c.get_con_from_hdl(h);
      auto cont = callback(Notification::fromJson(json));

      if (cont == false) {
        handle->close(websocketpp::close::status::normal, "Bye");
      }
    };

    c.init_asio();
    c.set_message_handler(on_message);

    ws_client::connection_ptr con = c.get_connection(
        fmt::format("ws://{}/notify", normalizeUrl(base_url)), ec);

    if (ec) {
      auto message =
          fmt::format("Could not create connection because: {}", ec.message());
      spdlog::error(message);
      throw std::runtime_error(message);
    }
    c.connect(con);
    c.run();
  });
}
} // namespace m0n1t0r
