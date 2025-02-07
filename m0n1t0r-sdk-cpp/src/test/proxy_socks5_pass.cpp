#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto server = m0n1t0r::Server(base_url);
  assert(server.listProxies().size() == 0);

  auto clients = m0n1t0r::Client::all(base_url);
  auto client = clients.front();
  auto addr = client->proxySocks5("qwq", "qwq");
  assert(addr.size() > 0);

  auto proxies = server.listProxies();
  assert(proxies.size() == 1);
  assert(proxies.front().addr == addr);

  server.deleteProxy(addr);
  assert(server.listProxies().size() == 0);
}
