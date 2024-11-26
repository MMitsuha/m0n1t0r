#ifndef WINDOW_CLIENT_H
#define WINDOW_CLIENT_H

#include "widget/client_tab.h"
#include <QDialog>

namespace Ui {
class Client;
}

namespace Window {
class Client : public QDialog {
  Q_OBJECT

public:
  explicit Client(QString addr, QUrl base_url, QString password,
                  QWidget *parent = nullptr);
  ~Client();

private:
  Ui::Client *ui;
  Widget::ClientTab *w_tab;
};
} // namespace Window

#endif // WINDOW_CLIENT_H
