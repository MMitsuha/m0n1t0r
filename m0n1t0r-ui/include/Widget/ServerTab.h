#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_SERVERTAB_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_SERVERTAB_H_

#include "Model/ClientTableModel.h"
#include "Subscriber/ConnectionSubscriber.h"
#include <QUrl>
#include <QWidget>
#include <m0n1t0r-sdk-cpp/m0n1t0r-sdk.h>

namespace Ui {
class ServerTab;
}

class ServerTab : public QWidget {
  Q_OBJECT

public:
  explicit ServerTab(ConnectionSubscriber *subscriber,
                     QWidget *parent = nullptr);
  ~ServerTab();

Q_SIGNALS:
  void clientDoubleClicked(std::shared_ptr<m0n1t0r::Client> client);

public Q_SLOTS:
  void on_tableView_overview_doubleClicked(const QModelIndex &index);

private:
  Ui::ServerTab *ui;
  ConnectionSubscriber *subscriber;
  ClientTableModel *client_table_model;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_SERVERTAB_H_
