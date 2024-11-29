#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESS_MODEL_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESS_MODEL_H_

#include "common/client.h"
#include <QAbstractTableModel>
#include <QArrayData>
#include <m0n1t0r-sdk.h>

namespace Model {
class Process : public QAbstractTableModel {
  Q_OBJECT

public:
  const QVector<QString> header_data = {
      tr("Process ID"),
      tr("Name"),
      tr("Executable"),
      tr("Command"),
  };

  explicit Process(std::shared_ptr<m0n1t0r::Client> client,
                   QObject *parent = nullptr);
  ~Process();

  int rowCount(const QModelIndex &parent = {}) const override;
  int columnCount(const QModelIndex &parent = {}) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;

  QVector<std::tuple<uint64_t, QVector<QString>>> process_list;

public Q_SLOTS:
  void refresh();

private:
  std::shared_ptr<m0n1t0r::Client> client;

private Q_SLOTS:
};
} // namespace Model

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESS_MODEL_H_
