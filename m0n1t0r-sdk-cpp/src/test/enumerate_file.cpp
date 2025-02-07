#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto clients = m0n1t0r::Client::all(base_url);
  auto client = clients.front();
  auto detail = client->getDetail();
  std::vector<m0n1t0r::Client::File> files;
  if (detail.system_info.long_os_version.find("Windows") == std::string::npos) {
    files = clients.front()->listFiles("/");
  } else {
    files = clients.front()->listFiles("C:\\");
  }
  assert(files.size() > 0);
}
