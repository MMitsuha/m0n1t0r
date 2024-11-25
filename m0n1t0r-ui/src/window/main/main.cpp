#include "window/main.h"
#include "ui_main.h"
#include "widget/main_tab.h"
#include <QMetaObject>
#include <spdlog/spdlog.h>

namespace Window {
Main::Main(QWidget *parent) : QMainWindow(parent), ui(new Ui::Main) {
  ui->setupUi(this);
  // menuBar()->setNativeMenuBar(false);

  w_connect = new Window::Connect(this);
  w_tab = new Widget::MainTab(this);

  setCentralWidget(w_tab);

  connect(ui->actionConnect, &QAction::triggered, w_connect,
          &Window::Connect::exec);
  connect(w_connect, &Window::Connect::connectServer, w_tab,
          &Widget::MainTab::connectServer);

  QMetaObject::invokeMethod(this, &Main::initialize, Qt::QueuedConnection);
}

Main::~Main() { delete ui; }

void Main::initialize() {}
} // namespace Window
