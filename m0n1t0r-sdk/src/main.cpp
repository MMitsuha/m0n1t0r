#include "m0n1t0r-sdk.h"
#include <boost/beast.hpp>
#include <cpr/cpr.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
std::string normalizeUrl(std::string url) {
  auto prefix = url.find("://");
  auto suffix = url.find_last_of("/");
  if (prefix != std::string::npos) {
    url.erase(0, prefix + 3);
  }
  if (suffix != std::string::npos) {
    url.erase(url.find_last_of("/"));
  }
  return url;
}

nlohmann::json getBodyJson(const cpr::Response &res) {
  if (res.error) {
    auto message = std::format("Failed send request: {}", res.error.message);
    spdlog::error(message);
    throw std::runtime_error(message);
  }

  auto json = json::parse(res.text);

  if (json["code"] != 0) {
    auto message = std::format("Failed to get JSON: {}",
                               static_cast<std::string>(json["body"]));
    spdlog::error(message);
    throw std::runtime_error(message);
  }

  return json["body"];
}
} // namespace m0n1t0r

int main() {
  auto clients = m0n1t0r::Client::all("http://127.0.0.1:10801/");

  for (auto client : clients) {
    spdlog::info("Addr: {}", client->getAddr());
    spdlog::info("OS: {}", client->getDetail().system_info.long_os_version);
    spdlog::info("First directory: {}", client->listFiles("/").front().path);
    spdlog::info("File: {}", client->getFile("/Users/mitsuha/.zprofile"));
    spdlog::info("File: {}",
                 client->getFileInfo("/Users/mitsuha/.zprofile").name);
    spdlog::info("Proxy: {}", client->proxySocks5("qwq", "qwq"));
    auto output = client->executeCommand("ls");
    spdlog::info("Command: {}",
                 std::string(output._stdout.begin(), output._stdout.end()));
    spdlog::info("Process: {}", client->listProcesses().front().name);
    spdlog::info("System: {}", client->getSystemInfo().long_os_version);
    spdlog::info("--------------");
    client
        ->executeCommandInteractive(
            "sh", "uname\n",
            [](const std::string &input, std::string &output) {
              spdlog::info(input);
              return false;
            })
        .join();
    spdlog::info("--------------");
    client
        ->captureScreen([](const std::string &frame) {
          spdlog::info(frame.size());
          return false;
        })
        .join();
    spdlog::info("--------------");
    m0n1t0r::Client::notify("127.0.0.1:10801",
                            [](const m0n1t0r::Client::Notification &msg) {
                              spdlog::info("Received message: {}", msg.addr);
                              return true;
                            })
        .join();
  }
}
