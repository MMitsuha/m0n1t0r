#ifndef __M0N1T0R_M0N1T0R_SDK_CPP_INCLUDE_M0N1T0R_SDK_H_
#define __M0N1T0R_M0N1T0R_SDK_CPP_INCLUDE_M0N1T0R_SDK_H_

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
    std::string build_time;
    std::string commit_hash;

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

  struct Notification {
    std::string addr;
    int16_t event;

    static Notification fromJson(nlohmann::json json);
  };

  struct QQAccount {
    int64_t uin;
    int64_t face_index;
    int64_t gender;
    // TODO: this may be null, so we should use optional?
    std::string nickname;
    int64_t client_type;
    int64_t uin_flag;
    int64_t account;

    static QQAccount fromJson(nlohmann::json json);
  };

  // Base
  std::thread notify(const std::string &base_url,
                     std::function<bool(const Notification &)> callback);
  static std::vector<std::shared_ptr<Client>> all(const std::string &base_url);
  Client(const std::string &base_url, const std::string &addr);
  ~Client() = default;
  inline std::string getBaseUrl() const { return base_url; }
  inline std::string getAddr() const { return addr; }
  Detail getDetail();
  SystemInfo getSystemInfo();
  void terminate();
  std::thread notifyClose(std::function<void()> callback);

  // File
  std::vector<File> listFiles(const std::string &path);
  std::string getFile(const std::string &path);
  void deleteFile(const std::string &path);
  void deleteDirectory(const std::string &path);
  void createDirectory(const std::string &path);
  void uploadFile(const std::string &path, const std::string &content);
  File getFileInfo(const std::string &path);

  // Proxy
  std::string proxySocks5();
  std::string proxySocks5(const std::string &name, const std::string &password);

  // Update
  void update(const std::string &url, const std::string &temp = "temp.bin");
  void update(const std::vector<uint8_t> &file,
              const std::string &temp = "temp.bin");

  // Network
  void download(const std::string &url, const std::string &path);

  // Process
  CommandOutput executeCommandBlocked(const std::string &command);
  void executeCommandDetached(const std::string &command);
  std::vector<Process> listProcesses();
  std::vector<Process> killProcess(const std::string &name);
  std::vector<Process> killProcess(const uint32_t pid);
  std::thread executeCommandInteractive(
      const std::string &proc,
      std::function<bool(const std::string & /*output*/)> callback,
      std::function<void()> close, msd::channel<std::string> &input);

  // QQ
  std::vector<QQAccount> listQQAccounts();

  // Environment
  std::unordered_map<std::string, std::string> listEnvironments();

private:
  std::string addr;
  std::string base_url;
};

class Server {
public:
  struct Detail {
    std::string version;
    std::string build_time;
    std::string commit_hash;

    static Detail fromJson(nlohmann::json json);
  };

  struct Proxy {
    std::string addr;
    std::string type;

    static Proxy fromJson(nlohmann::json json);
  };

  // Base
  explicit Server(const std::string &base_url);
  ~Server() = default;
  Detail getDetail();
  inline std::string getBaseUrl() const { return base_url; }
  inline std::shared_ptr<Client> client(const std::string &addr) {
    return std::make_shared<Client>(base_url, addr);
  }

  // Notify
  std::thread notifyClose(std::function<void()> callback);

  // Proxy
  std::vector<Proxy> listProxies();
  void deleteProxy(const std::string &addr);

private:
  std::string base_url;
};
} // namespace m0n1t0r

#endif // __M0N1T0R_M0N1T0R_SDK_CPP_INCLUDE_M0N1T0R_SDK_H_
