#include "window/client.h"
#include "ui_client.h"

namespace Window {
Client::Client(std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QDialog(parent), ui(new Ui::Client), client(_client) {
  ui->setupUi(this);

  setWindowTitle(QString::fromStdString(client->getAddr()));

  w_tab = new Widget::ClientTab(client, this);

  ui->gridLayout->addWidget(w_tab);
}

Client::~Client() { delete ui; }
} // namespace Window
