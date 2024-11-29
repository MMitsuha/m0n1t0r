#include "widget/process_table.h"
#include "ui_process_table.h"

namespace Widget {
ProcessTable::ProcessTable(std::shared_ptr<m0n1t0r::Client> _client,
                           QWidget *parent)
    : QWidget(parent), ui(new Ui::ProcessTable), client(_client) {
  ui->setupUi(this);

  m_process = new Model::Process(client, this);
  m_process->refresh();

  ui->tableView->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->tableView->setModel(m_process);
}

ProcessTable::~ProcessTable() { delete ui; }
} // namespace Widget
