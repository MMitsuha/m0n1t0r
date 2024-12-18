#include "Window/MainWindow.h"
#include "ui_MainWindow.h"
#include <QMetaObject>
#include <spdlog/spdlog.h>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow),
      subscriber(new ConnectionSubscriber(this)),
      server_tab(new ServerTab(subscriber, this)),
      m_connect(new Connect(this)) {
  ui->setupUi(this);
  // menuBar()->setNativeMenuBar(false);

  setCentralWidget(server_tab);

  connect(m_connect, &Connect::serverConnected, subscriber,
          &ConnectionSubscriber::onServerConnected);
  connect(subscriber, &ConnectionSubscriber::serverConnected, this,
          &MainWindow::onServerConnected);
  connect(subscriber, &ConnectionSubscriber::serverDisconnected, this,
          &MainWindow::onServerDisconnected);
  connect(subscriber, &ConnectionSubscriber::clientDisconnected, this,
          &MainWindow::onClientDisconnected);
  connect(server_tab, &ServerTab::clientDoubleClicked, this,
          &MainWindow::onClientDoubleClicked);
}

MainWindow::~MainWindow() {
  delete ui;
  m_connect->deleteLater();
  server_tab->deleteLater();
  subscriber->deleteLater();
}

void MainWindow::on_actionConnect_triggered() { m_connect->exec(); }

void MainWindow::onServerConnected(std::shared_ptr<m0n1t0r::Server> server) {
  setWindowTitle(
      tr("m0n1t0r - %1").arg(QString::fromStdString(server->getBaseUrl())));
}

void MainWindow::onServerDisconnected() {
  setWindowTitle(tr("m0n1t0r - Unknown"));

  for (auto &client_window : client_windows) {
    client_window.second->deleteLater();
  }
  client_windows.clear();
}

void MainWindow::onClientDoubleClicked(
    std::shared_ptr<m0n1t0r::Client> client) {
  auto client_window = new ClientWindow(client, this);
  client_windows[client->getAddr()] = client_window;
  client_window->show();
}

void MainWindow::onClientDisconnected(std::string addr) {
  client_windows[addr]->deleteLater();
  client_windows.erase(addr);
}
