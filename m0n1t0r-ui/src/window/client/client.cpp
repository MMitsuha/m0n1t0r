#include "window/client.h"
#include "ui_client.h"

namespace Window {
Client::Client(QUrl _base_url, QWidget *parent)
    : QDialog(parent), ui(new Ui::Client), base_url(_base_url) {
  ui->setupUi(this);
}

Client::~Client() { delete ui; }
} // namespace Window
