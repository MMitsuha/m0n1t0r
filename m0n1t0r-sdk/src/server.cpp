#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <fmt/format.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Server::Server(const std::string &_base_url) : base_url(_base_url) {}

Server::Notification Server::Notification::fromJson(nlohmann::json json) {
  return Notification{
      json["addr"],
      json["event"],
  };
}

std::vector<std::shared_ptr<Client>> Server::allClient() {
  auto ret = std::vector<std::shared_ptr<Client>>();
  auto res =
      cpr::Get(cpr::Url(fmt::format("{}/client", normalizeUrl(base_url))));
  auto json = getBodyJson(res);

  for (auto &detail : json) {
    ret.emplace_back(std::make_shared<Client>(base_url, detail["addr"]));
  }
  return ret;
}

std::thread
Server::notifyConnect(std::function<bool(const Notification &)> callback) {
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
        fmt::format("ws://{}/client/notify", normalizeUrl(base_url)), ec);

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

std::thread Server::notifyClose(std::function<void()> close) {
  return std::thread([=]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_close = [=, &c](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_close_handler(on_close);

    ws_client::connection_ptr con = c.get_connection(
        fmt::format("ws://{}/client/notify", normalizeUrl(base_url)), ec);

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

std::shared_ptr<Client> Server::client(const std::string &addr) {
  return std::make_shared<Client>(base_url, addr);
}
} // namespace m0n1t0r
