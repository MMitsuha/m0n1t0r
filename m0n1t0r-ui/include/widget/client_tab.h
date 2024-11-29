#ifndef CLIENT_TAB_H
#define CLIENT_TAB_H

#include "widget/filetree.h"
#include <QUrl>
#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class ClientTab;
}

namespace Widget {
class ClientTab : public QWidget {
  Q_OBJECT

public:
  explicit ClientTab(std::shared_ptr<m0n1t0r::Client> client,
                     QWidget *parent = nullptr);
  ~ClientTab();

public Q_SLOTS:

private:
  Ui::ClientTab *ui;
  std::shared_ptr<m0n1t0r::Client> client;
  Widget::FileTree *w_filewidget;
};
} // namespace Widget

#endif // CLIENT_TAB_H
