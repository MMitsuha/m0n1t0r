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

void MainTab::connectServer(QUrl url, QString password) {
  base_url = url;
  m_overview->connectServer(url, password);
}

void MainTab::on_tableView_overview_doubleClicked(const QModelIndex &index) {
  auto addr = m_overview->client_list[index.row()][0];
  auto relative = QString("client/%1").arg(addr);
  auto client = new Window::Client(base_url.resolved(relative), this);
  client->setWindowTitle(relative);
  w_clients.push_back(client);
  client->exec();
}
} // namespace Widget