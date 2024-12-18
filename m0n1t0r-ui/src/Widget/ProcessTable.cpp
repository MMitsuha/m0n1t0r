#include "Widget/ProcessTable.h"
#include "ui_ProcessTable.h"

ProcessTable::ProcessTable(std::shared_ptr<m0n1t0r::Client> client,
                           QWidget *parent)
    : QWidget(parent), ui(new Ui::ProcessTable),
      process_model(new ProcessModel(client, this)) {
  ui->setupUi(this);
  ui->tableView->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView->setModel(process_model);

  connect(this, &ProcessTable::refresh, process_model,
          &ProcessModel::onRefresh);
}

ProcessTable::~ProcessTable() {
  delete ui;
  process_model->deleteLater();
}
