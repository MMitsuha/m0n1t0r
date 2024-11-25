#include "widget/client_tab.h"
#include "ui_client_tab.h"
#include <QMessageBox>
#include <QTimer>

namespace Widget {
ClientTab::ClientTab(QWidget *parent) : QWidget(parent), ui(new Ui::ClientTab) {
  ui->setupUi(this);

  u_client = new Network::Client(this);
  m_overview = new Model::Overview(this);
  u_geoip = new Network::GeoIp(this);

  ui->tableView_overview->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView_overview->setModel(m_overview);
  ui->tableView_overview->setSelectionBehavior(QAbstractItemView::SelectRows);
  ui->tableView_overview->setSelectionMode(QAbstractItemView::SingleSelection);

  connect(u_client, &Network::Client::connected, m_overview,
          &Model::Overview::onConnect);
  connect(
      u_client, &Network::Client::connected, this,
      [this](Common::ClientDetail detail) { u_geoip->queryIp(detail.addr); });
  connect(u_client, &Network::Client::disconnected, m_overview,
          &Model::Overview::onDisconnect);
  connect(u_geoip, &Network::GeoIp::queryIpFinished, m_overview,
          &Model::Overview::onQueryIpFinished);
}

ClientTab::~ClientTab() { delete ui; }

void ClientTab::connectServer(QUrl url, QString password) {
  u_client->setBaseUrl(url)->fetchList()->subscribeNotification();
}
} // namespace Widget
