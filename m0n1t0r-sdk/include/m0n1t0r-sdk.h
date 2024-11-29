#ifndef M0N1T0R_SDK_H
#define M0N1T0R_SDK_H

#include <cpr/cpr.h>
#include <memory>
#include <nlohmann/json.hpp>
#include <string>
#include <websocketpp/config/asio_no_tls_client.hpp>

#include <websocketpp/client.hpp>

namespace m0n1t0r {
class Server {
public:
  explicit Server(const std::string &base_url);
  ~Server() = default;

  std::string getBaseUrl() const { return base_url; }

private:
  std::string base_url;
};

class Client {
public:
  struct SystemInfo {
    uint64_t boot_time;
    std::string cpu_arch;
    std::string distribution_id;
    std::string host_name;
    std::string kernel_version;
    std::string long_os_version;
    std::string name;
    uint64_t uptime;

    static SystemInfo fromJson(nlohmann::json json);
  };

  struct Detail {
    std::string addr;
    SystemInfo system_info;
    std::string target_platform;
    std::string version;

    static Detail fromJson(nlohmann::json json);
  };

  struct File {
    bool is_dir;
    bool is_symlink;
    std::string name;
    std::string path;
    int64_t size;

    static File fromJson(nlohmann::json json);
  };

  struct CommandOutput {
    std::vector<uint8_t> _stderr;
    std::vector<uint8_t> _stdout;
    bool success;

    static CommandOutput fromJson(nlohmann::json json);
  };

  struct Process {
    std::vector<std::string> cmd;
    std::string exe;
    std::string name;
    uint64_t pid;

    static Process fromJson(nlohmann::json json);
  };

  struct Notification {
    std::string addr;
    int16_t event;

    static Notification fromJson(nlohmann::json json);
  };

  using ws_client = websocketpp::client<websocketpp::config::asio_client>;

  Client(const std::string &base_url, const std::string &addr);
  ~Client() = default;

  std::string getBaseUrl() const { return base_url; }
  std::string getAddr() const { return addr; }
  Detail getDetail();
  std::vector<File> listFiles(const std::string &path);
  std::string getFile(const std::string &path);
  void deleteFile(const std::string &path);
  void deleteDirectory(const std::string &path);
  void createDirectory(const std::string &path);
  void uploadFile(const std::string &path, const std::string &content);
  File getFileInfo(const std::string &path);
  SystemInfo getSystemInfo();
  std::string proxySocks5();
  std::string proxySocks5(const std::string &name, const std::string &password);
  CommandOutput executeCommand(const std::string &command);
  std::vector<Process> listProcesses();
  void download(const std::string &path, const std::string &url);
  static std::thread notify(const std::string &base_url,
                            std::function<bool(const Notification &)> callback);
  std::thread executeCommandInteractive(
      const std::string &proc, const std::string &command,
      std::function<bool(const std::string &, std::string &)> callback);
  std::thread captureScreen(std::function<bool(const std::string &)> callback);
  static std::vector<std::shared_ptr<Client>> all(const std::string &base_url);

private:
  std::string addr;
  std::string base_url;
};

std::string normalizeUrl(std::string url);
nlohmann::json getBodyJson(const cpr::Response &res);
} // namespace m0n1t0r

#endif // M0N1T0R_SDK_H
