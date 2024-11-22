#ifndef __OVERVIEW_MODEL_H__
#define __OVERVIEW_MODEL_H__

#include "common/client.h"
#include <QAbstractTableModel>
#include <QArrayData>

namespace Model {
class Overview : public QAbstractTableModel {
  Q_OBJECT

public:
  const QVector<QString> header_data = {
      tr("Address"),         tr("Version"),
      tr("Target Platform"), tr("Name"),
      tr("Kernel Version"),  tr("Long OS Version"),
      tr("Distribution ID"), tr("Host Name"),
      tr("CPU Architecture")};

  explicit Overview(QObject *parent = nullptr);
  ~Overview();

  int rowCount(const QModelIndex &parent = QModelIndex()) const override;
  int columnCount(const QModelIndex &parent = QModelIndex()) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;

  QVector<QVector<QString>> client_list;

public Q_SLOTS:
  void updateClient(QVector<Common::ClientDetail> list);
};
} // namespace Model

#endif // __OVERVIEW_MODEL_H__
