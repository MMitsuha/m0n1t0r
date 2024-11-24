#ifndef CLIENT_TAB_H
#define CLIENT_TAB_H

#include "model/overview_model.h"
#include "network/client.h"
#include <QWidget>

namespace Ui {
class ClientTab;
}

namespace Widget {
class ClientTab : public QWidget {
  Q_OBJECT

public:
  explicit ClientTab(QWidget *parent = nullptr);
  ~ClientTab();

public Q_SLOTS:
  void connectServer(QUrl url, QString password);

private:
  Ui::ClientTab *ui;
  Model::Overview *m_overview;
  Network::Client *u_client;
  QTimer *timer;
};
} // namespace Widget

#endif // CLIENT_TAB_H
