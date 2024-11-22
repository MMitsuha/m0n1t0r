#include "widget/client_tab.h"
#include "ui_client_tab.h"

namespace Widget {
ClientTab::ClientTab(QWidget *parent) : QWidget(parent), ui(new Ui::ClientTab) {
  ui->setupUi(this);

  u_client = new Network::Client(this);
  overview_model = new Model::Overview(this);

  ui->tableView_overview->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView_overview->setModel(overview_model);

  connect(u_client, &Network::Client::getListFinished, overview_model,
          &Model::Overview::updateClient);
}

ClientTab::~ClientTab() { delete ui; }

void ClientTab::connectServer(QString url, QString password) {
  auto client_url = QUrl(url).resolved(QUrl("client"));

  u_client->setBaseUrl(client_url);
  u_client->getList();
}
} // namespace Widget
