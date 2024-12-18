#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_PROCESSTABLE_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_PROCESSTABLE_H_

#include "Model/ProcessModel.h"
#include <QWidget>

namespace Ui {
class ProcessTable;
}

class ProcessTable : public QWidget {
  Q_OBJECT

public:
  explicit ProcessTable(std::shared_ptr<m0n1t0r::Client> client,
                        QWidget *parent = nullptr);
  ~ProcessTable();

Q_SIGNALS:
  void refresh();

private:
  Ui::ProcessTable *ui;
  ProcessModel *process_model;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_PROCESSTABLE_H_
