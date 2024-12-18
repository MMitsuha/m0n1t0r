#include "Widget/ServerTab.h"
#include "ui_ServerTab.h"

ServerTab::ServerTab(ConnectionSubscriber *_subscriber, QWidget *parent)
    : QWidget(parent), ui(new Ui::ServerTab), subscriber(_subscriber),
      client_table_model(new ClientTableModel(subscriber, this)) {
  ui->setupUi(this);
  ui->tableView_overview->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView_overview->setModel(client_table_model);
}

ServerTab::~ServerTab() {
  delete ui;
  client_table_model->deleteLater();
}

void ServerTab::on_tableView_overview_doubleClicked(const QModelIndex &index) {
  emit clientDoubleClicked(client_table_model->getClientByRow(index.row()));
}
