#include "m0n1t0r-sdk.h"
#include <boost/beast.hpp>
#include <cpr/cpr.h>
#include <spdlog/spdlog.h>

namespace m0n1t0r {
Server::Server(const std::string &_base_url) : base_url(_base_url) {}
} // namespace m0n1t0r
