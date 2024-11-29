#ifndef PROCESS_TABLE_H
#define PROCESS_TABLE_H

#include "model/process_model.h"
#include <QWidget>

namespace Ui {
class ProcessTable;
}

namespace Widget {
class ProcessTable : public QWidget {
  Q_OBJECT

public:
  explicit ProcessTable(std::shared_ptr<m0n1t0r::Client> client,
                        QWidget *parent = nullptr);
  ~ProcessTable();

private:
  Ui::ProcessTable *ui;
  std::shared_ptr<m0n1t0r::Client> client;
  Model::Process *m_process;
};
} // namespace Widget

#endif // PROCESS_TABLE_H
