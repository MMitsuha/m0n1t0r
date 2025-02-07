#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto clients = m0n1t0r::Client::all(base_url);
  auto accounts = clients.front()->listQQAccounts();
  assert(accounts.size() > 0);
}
