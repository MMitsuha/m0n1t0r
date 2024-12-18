#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CONNECT_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CONNECT_H_

#include <QDialog>
#include <QUrl>
#include <m0n1t0r-sdk.h>

namespace Ui {
class ConnectDialog;
}

class Connect : public QDialog {
  Q_OBJECT

public:
  explicit Connect(QWidget *parent = nullptr);
  ~Connect();

Q_SIGNALS:
  void serverConnected(std::shared_ptr<m0n1t0r::Server> server);

private:
  Ui::ConnectDialog *ui;

private Q_SLOTS:
  void on_pushButton_connect_clicked();
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CONNECT_H_
