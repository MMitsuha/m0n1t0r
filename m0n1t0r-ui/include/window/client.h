#ifndef WINDOW_CLIENT_H
#define WINDOW_CLIENT_H

#include "widget/client_tab.h"
#include <QDialog>
#include <m0n1t0r-sdk.h>

namespace Ui {
class Client;
}

namespace Window {
class Client : public QDialog {
  Q_OBJECT

public:
  explicit Client(std::shared_ptr<m0n1t0r::Client> client,
                  QWidget *parent = nullptr);
  ~Client();

private:
  Ui::Client *ui;
  Widget::ClientTab *w_tab;
  std::shared_ptr<m0n1t0r::Client> client;
};
} // namespace Window

#endif // WINDOW_CLIENT_H
