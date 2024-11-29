#ifndef CONNECT_H
#define CONNECT_H

#include <QDialog>
#include <QUrl>
#include <m0n1t0r-sdk.h>

namespace Ui {
class Connect;
}

namespace Window {
class Connect : public QDialog {
  Q_OBJECT

public:
  explicit Connect(QWidget *parent = nullptr);
  ~Connect();

Q_SIGNALS:
  void connectServer(std::shared_ptr<m0n1t0r::Server> server);

private:
  Ui::Connect *ui;

private Q_SLOTS:
  void on_pushButton_connect_clicked();
};
} // namespace Window

#endif // CONNECT_H
