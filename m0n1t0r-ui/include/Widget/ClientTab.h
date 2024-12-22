#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_

#include "Widget/FileTree.h"
#include "Widget/InteractiveShellTab.h"
#include "Widget/ProcessTable.h"
#include "Widget/RemoteDesktopWidget.h"
#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class ClientTab;
}

class ClientTab : public QWidget {
  Q_OBJECT

public:
  explicit ClientTab(std::shared_ptr<m0n1t0r::Client> client,
                     ProcessTable *process_table, FileTree *file_tree,
                     InteractiveShellTab *interactive_shell_tab,
                     RemoteDesktopWidget *remote_desktop_widget,
                     QWidget *parent = nullptr);
  ~ClientTab();

Q_SIGNALS:
  void permissionStatusChanged();

public Q_SLOTS:
  void on_pushButton_request_permission_clicked();
  void onPermissionStatusChanged();

private:
  Ui::ClientTab *ui;
  std::shared_ptr<m0n1t0r::Client> client;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_
