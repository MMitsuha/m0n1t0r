#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_

#include "Widget/FileTree.h"
#include "Widget/ProcessTable.h"
#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class ClientTab;
}

class ClientTab : public QWidget {
  Q_OBJECT

public:
  explicit ClientTab(ProcessTable *process_table, FileTree *file_tree,
                     QWidget *parent = nullptr);
  ~ClientTab();

public Q_SLOTS:

private:
  Ui::ClientTab *ui;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_CLIENTTAB_H_
