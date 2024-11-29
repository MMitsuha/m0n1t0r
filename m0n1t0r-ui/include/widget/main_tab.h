#ifndef MAIN_TAB_H
#define MAIN_TAB_H

#include "model/overview_model.h"
#include "window/client.h"
#include <QVector>
#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class MainTab;
}

namespace Widget {
class MainTab : public QWidget {
  Q_OBJECT

public:
  explicit MainTab(QWidget *parent = nullptr);
  ~MainTab();

public Q_SLOTS:
  void connectServer(std::shared_ptr<m0n1t0r::Server> server);
  void on_tableView_overview_doubleClicked(const QModelIndex &index);

private:
  Ui::MainTab *ui;
  Model::Overview *m_overview;
  QVector<Window::Client *> w_clients;
  std::shared_ptr<m0n1t0r::Server> server;
};
} // namespace Widget

#endif // MAIN_TAB_H
