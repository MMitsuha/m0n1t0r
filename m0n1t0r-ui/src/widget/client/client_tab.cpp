#include "widget/client_tab.h"
#include "ui_client_tab.h"

namespace Widget {
ClientTab::ClientTab(QString addr, QUrl _base_url, QString _password,
                     QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab), base_url(_base_url),
      password(_password) {
  ui->setupUi(this);

  w_fileview = new Widget::FileView(addr, base_url, ui->tabFiles);

  connectServer(base_url, password);
}

ClientTab::~ClientTab() { delete ui; }

void ClientTab::connectServer(QUrl url, QString _password) {
  base_url = url;
  password = _password;
}
} // namespace Widget
