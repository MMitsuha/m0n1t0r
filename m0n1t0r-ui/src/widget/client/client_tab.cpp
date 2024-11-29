#include "widget/client_tab.h"
#include "ui_client_tab.h"

namespace Widget {
ClientTab::ClientTab(std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab), client(_client) {
  ui->setupUi(this);

  w_filetree = new Widget::FileTree(client, ui->tabFiles);
  w_processtable = new Widget::ProcessTable(client, ui->tabProcesses);

  ui->tabFiles->layout()->addWidget(w_filetree);
  ui->tabProcesses->layout()->addWidget(w_processtable);
}

ClientTab::~ClientTab() { delete ui; }
} // namespace Widget
