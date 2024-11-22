#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "widget/client_tab.h"
#include "window/connect.h"
#include <QMainWindow>
#include <QtNetwork/QNetworkAccessManager>

namespace Ui {
class Main;
}

namespace Window {
class Main : public QMainWindow {
  Q_OBJECT

public:
  explicit Main(QWidget *parent = 0);
  ~Main();

private:
  Ui::Main *ui;
  Connect *w_connect;
  Widget::ClientTab *w_client;

private Q_SLOTS:
  void initialize();
};
} // namespace Window

#endif // MAINWINDOW_H
