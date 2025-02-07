#include "m0n1t0r-sdk.h"
#include "test.h"
#include <cassert>

int main() {
  auto clients = m0n1t0r::Client::all(base_url);
  auto environments = clients.front()->listEnvironments();
  assert(environments.size() > 0);
}
