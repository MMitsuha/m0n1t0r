#ifndef __COMMON_IP_H__
#define __COMMON_IP_H__

#include <QString>

namespace Common {
struct GeoIpDetail {
  QString country;
  QString country_code;
  QString region;
  QString region_name;
  QString city;
  QString zip;
  double lat;
  double lon;
  QString timezone;
  QString isp;
  QString org;
  QString as;
};
} // namespace Common

#endif // __COMMON_IP_H__
