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

void Overview::onConnect(Common::ClientDetail detail) {
  beginInsertRows(QModelIndex(), rowCount(), rowCount());
  QVector<QString> row = {detail.addr,
                          detail.version,
                          detail.target_platform,
                          detail.name,
                          detail.kernel_version,
                          detail.long_os_version,
                          detail.distribution_id,
                          detail.host_name,
                          detail.cpu_arch,
                          tr("Unknown"),
                          tr("Unknown"),
                          tr("Unknown")};
  client_list.append(row);
  endInsertRows();
}

void Overview::onDisconnect(QString addr) {
  for (int i = 0; i < client_list.count(); i++) {
    if (client_list[i][0] == addr) {
      beginRemoveRows(QModelIndex(), i, i);
      client_list.remove(i);
      endRemoveRows();
      break;
    }
  }
}

void Overview::onQueryIpFinished(QString addr, Common::GeoIpDetail detail) {
  for (int i = 0; i < client_list.count(); i++) {
    if (client_list[i][0] == addr) {
      client_list[i][9] = detail.country;
      client_list[i][10] = detail.region;
      client_list[i][11] = detail.isp;
      emit dataChanged(index(i, 9), index(i, 11));
      break;
    }
  }
}
} // namespace Model
