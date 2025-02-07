#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Client::CommandOutput Client::CommandOutput::fromJson(nlohmann::json json) {
  return CommandOutput{
      json["stderr"],
      json["stdout"],
      json["success"],
  };
}

Client::CommandOutput
Client::executeCommandBlocked(const std::string &command) {
  auto res =
      cpr::Post(cpr::Url(fmt::format("{}/process/execute", base_url)),
                cpr::Payload{{"command", command}, {"option", "blocked"}});
  auto json = getBodyJson(res);
  return CommandOutput::fromJson(json);
}

void Client::executeCommandDetached(const std::string &command) {
  auto res =
      cpr::Post(cpr::Url(fmt::format("{}/process/execute", base_url)),
                cpr::Payload{{"command", command}, {"option", "detached"}});
  auto json = getBodyJson(res);
}

Client::Process Client::Process::fromJson(nlohmann::json json) {
  return Process{
      json["cmd"],
      json["exe"].is_null() ? "[unknown]" : json["exe"],
      json["name"],
      json["pid"],
  };
}

std::vector<Client::Process> Client::listProcesses() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/process", base_url)));
  auto array = getBodyJson(res);
  auto ret = std::vector<Process>();

  for (auto &process : array) {
    ret.emplace_back(Process::fromJson(process));
  }
  return ret;
}

std::thread Client::executeCommandInteractive(
    const std::string &proc, std::function<bool(const std::string &)> callback,
    std::function<void()> close, msd::channel<std::string> &input) {
  return std::thread([=, &input]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_message = [=, &c](websocketpp::connection_hdl h,
                              ws_client::message_ptr msg) {
      if (callback(msg->get_payload()) == false) {
        auto handle = c.get_con_from_hdl(h);
        handle->close(websocketpp::close::status::normal, "Bye");
      }
    };
    auto on_close = [=](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_message_handler(on_message);
    c.set_close_handler(on_close);

    ws_client::connection_ptr con =
        c.get_connection(fmt::format("ws://{}/process/interactive?command={}",
                                     base_url, cpr::util::urlEncode(proc)),
                         ec);

    if (ec) {
      auto message =
          fmt::format("Could not create connection because: {}", ec.message());
      spdlog::error(message);
      throw std::runtime_error(message);
    }
    c.connect(con);

    std::thread([=, &input]() {
      for (const auto &command : input) {
        auto ec = con->send(command);

        if (ec) {
          auto message =
              fmt::format("Could not send message because: {}", ec.message());
          spdlog::error(message);
          input << command;
          return;
        }
      }
    }).detach();

    c.run();
  });
}

std::vector<Client::Process> killProcessInternal(const std::string &base_url,
                                                 const std::string &param) {
  auto res =
      cpr::Delete(cpr::Url(fmt::format("{}/process/{}", base_url, param)));
  auto array = getBodyJson(res);
  auto ret = std::vector<Client::Process>();

  for (auto &process : array) {
    ret.emplace_back(Client::Process::fromJson(process));
  }
  return ret;
}

std::vector<Client::Process> Client::killProcess(const std::string &name) {
  return killProcessInternal(base_url, fmt::format("name/{}", name));
}

std::vector<Client::Process> Client::killProcess(const uint32_t pid) {
  return killProcessInternal(base_url, fmt::format("pid/{}", pid));
}
} // namespace m0n1t0r
