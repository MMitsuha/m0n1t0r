#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto server = m0n1t0r::Server(base_url);
  auto detail = server.getDetail();
  assert(detail.build_time.size() > 0);
}
