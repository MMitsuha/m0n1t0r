#ifndef FILEVIEW_H
#define FILEVIEW_H

#include "model/directory_model.h"
#include <QWidget>

namespace Ui {
class FileView;
}

namespace Widget {
class FileView : public QWidget {
  Q_OBJECT

public:
  explicit FileView(QString addr, QUrl base_url, QWidget *parent = nullptr);
  ~FileView();

private:
  Ui::FileView *ui;
  Model::DirectoryModel *m_directory;
};
} // namespace Widget

#endif // FILEVIEW_H
