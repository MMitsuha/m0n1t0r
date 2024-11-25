#ifndef CLIENT_TAB_H
#define CLIENT_TAB_H

#include "widget/fileview.h"
#include <QUrl>
#include <QWidget>

namespace Ui {
class ClientTab;
}

namespace Widget {
class ClientTab : public QWidget {
  Q_OBJECT

public:
  explicit ClientTab(QUrl base_url, QString password,
                     QWidget *parent = nullptr);
  ~ClientTab();

public Q_SLOTS:
  void connectServer(QUrl url, QString password);

private:
  Ui::ClientTab *ui;
  QUrl base_url;
  QString password;
  Widget::FileView *w_fileview;
};
} // namespace Widget

#endif // CLIENT_TAB_H
