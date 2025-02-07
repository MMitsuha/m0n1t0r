#include "common.h"
#include "m0n1t0r-sdk.h"
#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>

using json = nlohmann::json;

namespace m0n1t0r {
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
  auto ret = std::vector<File>();

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
} // namespace m0n1t0r
