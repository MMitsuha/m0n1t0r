#include "widget/main_tab.h"
#include "ui_main_tab.h"
#include <QMessageBox>
#include <QTimer>

namespace Widget {
MainTab::MainTab(QWidget *parent) : QWidget(parent), ui(new Ui::MainTab) {
  ui->setupUi(this);

  m_overview = new Model::Overview(this);

  ui->tableView_overview->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView_overview->setModel(m_overview);
  ui->tableView_overview->setSelectionBehavior(QAbstractItemView::SelectRows);
  ui->tableView_overview->setSelectionMode(QAbstractItemView::SingleSelection);
}

MainTab::~MainTab() { delete ui; }

void MainTab::connectServer(std::shared_ptr<m0n1t0r::Server> _server) {
  server = _server;
  m_overview->connectServer(server);
}

void MainTab::on_tableView_overview_doubleClicked(const QModelIndex &index) {
  auto addr = std::get<0>(m_overview->client_list[index.row()]);
  auto client = new Window::Client(addr, this);
  w_clients.push_back(client);
  client->show();
}
} // namespace Widget
