#include "model/overview_model.h"
#include <QUrl>

namespace Model {
Overview::Overview(QObject *parent) : QAbstractTableModel(parent) {
  u_client = new Network::Client(this);
  u_geoip = new Network::GeoIp(this);

  connect(u_client, &Network::Client::connected, this,
          &Model::Overview::onConnect);
  connect(
      u_client, &Network::Client::connected, this,
      [this](Common::ClientDetail detail) { u_geoip->queryIp(detail.addr); });
  connect(u_client, &Network::Client::disconnected, this,
          &Model::Overview::onDisconnect);
  connect(u_geoip, &Network::GeoIp::queryIpFinished, this,
          &Model::Overview::onQueryIpFinished);
}

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
      client_list[i][10] = detail.region_name;
      client_list[i][11] = detail.isp;
      emit dataChanged(index(i, 9), index(i, 11));
      break;
    }
  }
}

void Overview::clear() {
  beginResetModel();
  client_list.clear();
  endResetModel();
}

void Overview::connectServer(QUrl url, QString password) {
  clear();
  u_client->setBaseUrl(url)->fetchList()->subscribeNotification();
}
} // namespace Model
