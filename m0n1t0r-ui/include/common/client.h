#ifndef __CLIENT_H__
#define __CLIENT_H__

#include <QString>

namespace Common {
struct ClientDetail {
  QString addr;
  QString version;
  QString target_platform;
  QString name;
  QString kernel_version;
  QString long_os_version;
  QString distribution_id;
  QString host_name;
  QString cpu_arch;
};
} // namespace Common

#endif // __CLIENT_H__