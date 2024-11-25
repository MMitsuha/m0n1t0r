#ifndef CLIENT_TAB_H
#define CLIENT_TAB_H

#include "model/overview_model.h"
#include "window/client.h"
#include <QVector>
#include <QWidget>

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
  void connectServer(QUrl url, QString password);
  void on_tableView_overview_doubleClicked(const QModelIndex &index);

private:
  Ui::MainTab *ui;
  Model::Overview *m_overview;
  QVector<Window::Client *> w_clients;
  QUrl base_url;
};
} // namespace Widget

#endif // CLIENT_TAB_H
