#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CLIENTWINDOW_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CLIENTWINDOW_H_

#include "Widget/ClientTab.h"
#include "Widget/FileTree.h"
#include "Widget/ProcessTable.h"
#include <QDialog>
#include <m0n1t0r-sdk.h>

namespace Ui {
class ClientWindow;
}

class ClientWindow : public QDialog {
  Q_OBJECT

public:
  explicit ClientWindow(std::shared_ptr<m0n1t0r::Client> client,
                        QWidget *parent = nullptr);
  ~ClientWindow();

Q_SIGNALS:

public Q_SLOTS:

private:
  Ui::ClientWindow *ui;
  std::shared_ptr<m0n1t0r::Client> client;
  ProcessTable *process_table;
  FileTree *file_tree;
  ClientTab *client_tab;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WINDOW_CLIENTWINDOW_H_
