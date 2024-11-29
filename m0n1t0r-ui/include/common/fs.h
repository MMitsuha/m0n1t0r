#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_COMMON_FS_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_COMMON_FS_H_

#include <QString>

namespace Common {
struct FileDetail {
  bool is_dir;
  bool is_symlink;
  QString name;
  QString path;
  qint64 size;
};
} // namespace Common

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_COMMON_FS_H_
