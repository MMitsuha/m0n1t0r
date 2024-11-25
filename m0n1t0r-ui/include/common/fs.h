#ifndef __FS_H__
#define __FS_H__

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

#endif // __FS_H__
