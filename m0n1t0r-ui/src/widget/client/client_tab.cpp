#include "widget/client_tab.h"
#include "ui_client_tab.h"

namespace Widget {
ClientTab::ClientTab(std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab), client(_client) {
  ui->setupUi(this);

  w_filewidget = new Widget::FileTree(client, ui->tabFiles);
}

ClientTab::~ClientTab() { delete ui; }
} // namespace Widget
