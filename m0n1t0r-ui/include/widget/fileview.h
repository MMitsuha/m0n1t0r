#ifndef FILEVIEW_H
#define FILEVIEW_H

#include <QWidget>

namespace Ui {
class FileView;
}

namespace Widget {
class FileView : public QWidget {
  Q_OBJECT

public:
  explicit FileView(QWidget *parent = nullptr);
  ~FileView();

private:
  Ui::FileView *ui;
};
} // namespace Widget

#endif // FILEVIEW_H
