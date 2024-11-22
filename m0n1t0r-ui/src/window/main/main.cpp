#include "window/main.h"
#include "ui_main.h"
#include "widget/client_tab.h"
#include <QMetaObject>
#include <spdlog/spdlog.h>

namespace Window {
Main::Main(QWidget *parent) : QMainWindow(parent), ui(new Ui::Main) {
  ui->setupUi(this);
  // menuBar()->setNativeMenuBar(false);

  w_connect = new Connect(this);
  w_client = new Widget::ClientTab(this);

  setCentralWidget(w_client);

  connect(ui->actionConnect, &QAction::triggered, w_connect, &Connect::exec);
  connect(w_connect, &Connect::connectServer, w_client,
          &Widget::ClientTab::connectServer);

  QMetaObject::invokeMethod(this, &Main::initialize, Qt::QueuedConnection);
}

Main::~Main() { delete ui; }

void Main::initialize() {}
} // namespace Window
