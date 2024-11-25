#include "window/connect.h"
#include "ui_connect.h"
#include <spdlog/spdlog.h>

namespace Window {
Connect::Connect(QWidget *parent) : QDialog(parent), ui(new Ui::Connect) {
  ui->setupUi(this);
}

Connect::~Connect() { delete ui; }

void Connect::on_pushButton_connect_clicked() {
  emit connectServer(QUrl(ui->lineEdit_address->text()),
                     ui->lineEdit_password->text());
  close();
}
} // namespace Window
