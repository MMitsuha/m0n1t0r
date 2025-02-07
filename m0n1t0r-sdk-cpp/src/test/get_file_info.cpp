#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto clients = m0n1t0r::Client::all(base_url);
  auto client = clients.front();
  auto detail = client->getDetail();
  m0n1t0r::Client::File file;
  if (detail.system_info.long_os_version.find("Windows") == std::string::npos) {
    file = clients.front()->getFileInfo("/");
  } else {
    file = clients.front()->getFileInfo("C:\\");
  }
  assert(file.is_dir == true);
}
