#include "Window/ConnectDialog.h"
#include "Util/Log.h"
#include "ui_ConnectDialog.h"

Connect::Connect(QWidget *parent) : QDialog(parent), ui(new Ui::ConnectDialog) {
  ui->setupUi(this);
}

Connect::~Connect() { delete ui; }

void Connect::on_pushButton_connect_clicked() {
  std::shared_ptr<m0n1t0r::Server> server = nullptr;
  try {
    server = std::make_shared<m0n1t0r::Server>(
        ui->lineEdit_address->text().toStdString());
  } catch (std::runtime_error &e) {
    logError(tr("Failed to connect to server"), e, tr("Connection Error"),
             this);
    return;
  }

  emit serverConnected(server);
  close();
}
