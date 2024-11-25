#include "window/client.h"
#include "ui_client.h"

namespace Window {
Client::Client(QUrl base_url, QString password, QWidget *parent)
    : QDialog(parent), ui(new Ui::Client) {
  ui->setupUi(this);

  w_tab = new Widget::ClientTab(base_url, password, this);

  ui->gridLayout->addWidget(w_tab);
}

Client::~Client() { delete ui; }
} // namespace Window
