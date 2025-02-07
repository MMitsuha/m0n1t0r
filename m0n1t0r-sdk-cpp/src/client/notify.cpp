#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
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
} // namespace m0n1t0r
