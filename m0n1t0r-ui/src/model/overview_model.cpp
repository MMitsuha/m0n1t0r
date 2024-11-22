#include "model/overview_model.h"

namespace Model {
Overview::Overview(QObject *parent) : QAbstractTableModel(parent) {}

Overview::~Overview() {}

int Overview::rowCount(const QModelIndex &parent) const {
  return client_list.count();
}

int Overview::columnCount(const QModelIndex &parent) const {
  return header_data.count();
}

QVariant Overview::data(const QModelIndex &index, int role) const {
  if (role == Qt::DisplayRole) {
    return client_list[index.row()][index.column()];
  }
  return QVariant();
}

QVariant Overview::headerData(int section, Qt::Orientation orientation,
                              int role) const {
  if (role == Qt::DisplayRole) {
    if (orientation == Qt::Horizontal) {
      return header_data[section];
    }
  }
  return QVariant();
}

void Overview::updateClient(QVector<Common::ClientDetail> list) {
  beginResetModel();
  client_list.clear();
  for (auto &detail : list) {
    client_list.push_back({detail.addr, detail.version, detail.target_platform,
                           detail.name, detail.kernel_version,
                           detail.long_os_version, detail.distribution_id,
                           detail.host_name, detail.cpu_arch});
  }
  endResetModel();
}
} // namespace Model
