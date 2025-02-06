#ifndef __M0N1T0R_M0N1T0R_SDK_SRC_COMMON_H_
#define __M0N1T0R_M0N1T0R_SDK_SRC_COMMON_H_

#include <cpr/cpr.h>
#include <nlohmann/json.hpp>

namespace m0n1t0r {
std::string normalizeUrl(std::string url);
nlohmann::json getBodyJson(const cpr::Response &res);
} // namespace m0n1t0r

#endif // __M0N1T0R_M0N1T0R_SDK_SRC_COMMON_H_
