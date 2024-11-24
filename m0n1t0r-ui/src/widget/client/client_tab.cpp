#include "widget/client_tab.h"
#include "ui_client_tab.h"
#include <QMessageBox>
#include <QTimer>

namespace Widget {
ClientTab::ClientTab(QWidget *parent) : QWidget(parent), ui(new Ui::ClientTab) {
  ui->setupUi(this);

  u_client = new Network::Client(this);
  m_overview = new Model::Overview(this);
  timer = new QTimer(this);

  ui->tableView_overview->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView_overview->setModel(m_overview);
  ui->tableView_overview->setSelectionBehavior(QAbstractItemView::SelectRows);
  ui->tableView_overview->setSelectionMode(QAbstractItemView::SingleSelection);

  connect(u_client, &Network::Client::getListFinished, m_overview,
          &Model::Overview::updateClient);
  connect(u_client, &Network::Client::getListError, m_overview,
          [this](QString message) {
            timer->stop();
            QMessageBox::critical(this, tr("Error"), message);
          });
  connect(timer, &QTimer::timeout, u_client, &Network::Client::getList);
}

ClientTab::~ClientTab() { delete ui; }

void ClientTab::connectServer(QUrl url, QString password) {
  u_client->setBaseUrl(url.resolved(QUrl("client")));
  timer->start(1000);
}
} // namespace Widget
