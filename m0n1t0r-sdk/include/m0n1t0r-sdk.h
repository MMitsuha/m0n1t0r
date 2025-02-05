#ifndef M0N1T0R_SDK_H
#define M0N1T0R_SDK_H

#include <memory>
#include <msd/channel.hpp>
#include <nlohmann/json.hpp>
#include <string>
#include <websocketpp/client.hpp>
#include <websocketpp/config/asio_no_tls_client.hpp>

namespace m0n1t0r {
using ws_client = websocketpp::client<websocketpp::config::asio_client>;

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
    // TODO: this may be null, so we should use optional?
    std::string exe;
    std::string name;
    uint64_t pid;

    static Process fromJson(nlohmann::json json);
  };

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
  void download(const std::string &url, const std::string &path);
  void update(const std::string &url, const std::string &temp = "temp.bin");
  std::thread executeCommandInteractive(
      const std::string &proc,
      std::function<bool(const std::string & /*output*/)> callback,
      std::function<void()> close, msd::channel<std::string> &input);
  std::thread notifyClose(std::function<void()> callback);

private:
  std::string addr;
  std::string base_url;
};

class Server {
public:
  struct Notification {
    std::string addr;
    int16_t event;

    static Notification fromJson(nlohmann::json json);
  };

  explicit Server(const std::string &base_url);
  ~Server() = default;

  std::string getBaseUrl() const { return base_url; }
  std::vector<std::shared_ptr<Client>> allClient();
  std::thread notifyConnect(std::function<bool(const Notification &)> callback);
  std::thread notifyClose(std::function<void()> callback);
  std::shared_ptr<Client> client(const std::string &addr);

private:
  std::string base_url;
};
} // namespace m0n1t0r

#endif // M0N1T0R_SDK_H
