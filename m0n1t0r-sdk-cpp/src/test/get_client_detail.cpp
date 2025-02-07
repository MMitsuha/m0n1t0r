#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto clients = m0n1t0r::Client::all(base_url);
  auto detail = clients.front()->getDetail();
  assert(detail.target_platform.size() > 0);
}
