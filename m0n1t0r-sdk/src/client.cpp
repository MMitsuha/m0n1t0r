#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <fmt/format.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
Client::Client(const std::string &_base_url, const std::string &_addr)
    : base_url(fmt::format("{}/client/{}", normalizeUrl(_base_url), addr)),
      addr(_addr) {}

Client::SystemInfo Client::SystemInfo::fromJson(nlohmann::json json) {
  return SystemInfo{
      json["boot_time"], json["cpu_arch"],       json["distribution_id"],
      json["host_name"], json["kernel_version"], json["long_os_version"],
      json["name"],      json["uptime"],
  };
}

Client::Detail Client::Detail::fromJson(nlohmann::json json) {
  return Detail{
      json["addr"],
      SystemInfo::fromJson(json["system_info"]),
      json["target_platform"],
      json["version"],
  };
}

Client::Detail Client::getDetail() {
  auto res = cpr::Get(cpr::Url(base_url));
  return Detail::fromJson(getBodyJson(res));
}

Client::File Client::File::fromJson(nlohmann::json json) {
  return File{
      json["is_dir"], json["is_symlink"], json["name"],
      json["path"],   json["size"],
  };
}

std::vector<Client::File> Client::listFiles(const std::string &path) {
  auto res = cpr::Get(cpr::Url(
      fmt::format("{}/fs/dir/{}", base_url, cpr::util::urlEncode(path))));
  auto array = getBodyJson(res);
  auto ret = std::vector<Client::File>();

  for (auto &file : array) {
    ret.emplace_back(File::fromJson(file));
  }
  return ret;
}

std::string Client::getFile(const std::string &path) {
  auto res = cpr::Get(cpr::Url(
      fmt::format("{}/fs/file/{}", base_url, cpr::util::urlEncode(path))));
  return res.text;
}

void Client::deleteFile(const std::string &path) {
  auto res = cpr::Delete(cpr::Url(
      fmt::format("{}/fs/file/{}", base_url, cpr::util::urlEncode(path))));
  auto json = getBodyJson(res);
}

void Client::deleteDirectory(const std::string &path) {
  auto res = cpr::Delete(cpr::Url(
      fmt::format("{}/fs/dir/{}", base_url, cpr::util::urlEncode(path))));
  auto json = getBodyJson(res);
}

void Client::createDirectory(const std::string &path) {
  auto res = cpr::Put(cpr::Url(
      fmt::format("{}/fs/dir/{}", base_url, cpr::util::urlEncode(path))));
  auto json = getBodyJson(res);
}

void Client::uploadFile(const std::string &path, const std::string &content) {
  auto res = cpr::Put(cpr::Url(fmt::format("{}/fs/file/{}", base_url,
                                           cpr::util::urlEncode(path))),
                      cpr::Body(content));
  auto json = getBodyJson(res);
}

Client::File Client::getFileInfo(const std::string &path) {
  auto res = cpr::Get(cpr::Url(
      fmt::format("{}/fs/metadata/{}", base_url, cpr::util::urlEncode(path))));
  auto json = getBodyJson(res);
  return File::fromJson(json);
}

std::string Client::proxySocks5() {
  auto res =
      cpr::Get(cpr::Url(fmt::format("{}/proxy/socks5/noauth", base_url)));
  auto json = getBodyJson(res);
  return json;
}

std::string Client::proxySocks5(const std::string &name,
                                const std::string &password) {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/proxy/socks5/pass", base_url)),
                      cpr::Parameters{{"name", name}, {"password", password}});
  auto json = getBodyJson(res);
  return json;
}

Client::CommandOutput Client::CommandOutput::fromJson(nlohmann::json json) {
  return CommandOutput{
      json["stderr"],
      json["stdout"],
      json["success"],
  };
}

Client::CommandOutput Client::executeCommand(const std::string &command) {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/process/execute/{}", base_url,
                                           cpr::util::urlEncode(command))));
  auto json = getBodyJson(res);
  return CommandOutput::fromJson(json);
}

Client::Process Client::Process::fromJson(nlohmann::json json) {
  return Process{
      json["cmd"],
      json["exe"],
      json["name"],
      json["pid"],
  };
}

Client::Availability Client::Availability::fromJson(nlohmann::json json) {
  return Availability{
      json["has_permission"],
      json["support"],
  };
}

std::vector<Client::Process> Client::listProcesses() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/process", base_url)));
  auto array = getBodyJson(res);
  auto ret = std::vector<Client::Process>();

  for (auto &process : array) {
    ret.emplace_back(Process::fromJson(process));
  }
  return ret;
}

void Client::download(const std::string &path, const std::string &url) {
  auto res = cpr::Put(cpr::Url(fmt::format("{}/network/download/{}/{}",
                                           base_url, cpr::util::urlEncode(url),
                                           cpr::util::urlEncode(path))));
  auto json = getBodyJson(res);
}

Client::SystemInfo Client::getSystemInfo() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/info/system", base_url)));
  auto json = getBodyJson(res);
  return SystemInfo::fromJson(json);
}

Client::Availability Client::canCaptureScreen() {
  auto res = cpr::Get(cpr::Url(fmt::format("{}/screen", base_url)));
  auto json = getBodyJson(res);
  return Availability::fromJson(json);
}

bool Client::requestCapturePermission() {
  auto res = cpr::Put(cpr::Url(fmt::format("{}/screen", base_url)));
  auto json = getBodyJson(res);
  return true;
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
    auto on_close = [=, &c](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_message_handler(on_message);
    c.set_close_handler(on_close);

    ws_client::connection_ptr con =
        c.get_connection(fmt::format("ws://{}/process/interactive/{}", base_url,
                                     cpr::util::urlEncode(proc)),
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

std::thread
Client::captureScreen(std::function<bool(const std::string &)> callback,
                      std::function<void()> close, const std::string &format) {
  return std::thread([=]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_message = [=, &c](websocketpp::connection_hdl h,
                              ws_client::message_ptr msg) {
      if (msg->get_opcode() != websocketpp::frame::opcode::binary) {
        return;
      }

      auto handle = c.get_con_from_hdl(h);
      auto cont = callback(msg->get_payload());

      if (cont == false) {
        handle->close(websocketpp::close::status::normal, "Bye");
      }
    };
    auto on_close = [=, &c](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_message_handler(on_message);
    c.set_close_handler(on_close);

    ws_client::connection_ptr con = c.get_connection(
        fmt::format("ws://{}/screen/{}", base_url, format), ec);

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

std::thread Client::notifyClose(std::function<void()> close) {
  return std::thread([=]() {
    ws_client c;
    websocketpp::lib::error_code ec;
    auto on_close = [=, &c](websocketpp::connection_hdl) { close(); };

    c.init_asio();
    c.set_close_handler(on_close);

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
