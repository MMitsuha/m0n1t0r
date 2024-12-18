#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_MAINWINDOW_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_MAINWINDOW_H_

#include "Subscriber/ConnectionSubscriber.h"
#include "Widget/ServerTab.h"
#include "Window/ClientWindow.h"
#include "Window/ConnectDialog.h"
#include <QMainWindow>
#include <QtNetwork/QNetworkAccessManager>
#include <unordered_map>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  explicit MainWindow(QWidget *parent = 0);
  ~MainWindow();

public Q_SLOTS:
  void onServerConnected(std::shared_ptr<m0n1t0r::Server> server);
  void onServerDisconnected();
  void onClientDisconnected(std::string addr);
  void onClientDoubleClicked(std::shared_ptr<m0n1t0r::Client> client);

private:
  ConnectionSubscriber *subscriber;
  Ui::MainWindow *ui;
  ServerTab *server_tab;
  Connect *m_connect;
  std::unordered_map<std::string, ClientWindow *> client_windows;

private Q_SLOTS:
  void on_actionConnect_triggered();
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_MAINWINDOW_H_
