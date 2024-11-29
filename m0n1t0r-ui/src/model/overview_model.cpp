#include "model/overview_model.h"
#include <QMessageBox>
#include <QUrl>

namespace Model {
Overview::Overview(QObject *parent) : QAbstractTableModel(parent) {
  u_geoip = new Network::GeoIp(this);

  connect(u_geoip, &Network::GeoIp::queryIpFinished, this,
          &Overview::onQueryIpFinished);
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
    return std::get<1>(client_list[index.row()])[index.column()];
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

void Overview::onConnect(std::shared_ptr<m0n1t0r::Client> client) {
  auto detail = client->getDetail();
  beginInsertRows(QModelIndex(), rowCount(), rowCount());
  QVector<QString> row = {
      QString::fromStdString(detail.addr),
      QString::fromStdString(detail.version),
      QString::fromStdString(detail.target_platform),
      QString::fromStdString(detail.system_info.name),
      QString::fromStdString(detail.system_info.kernel_version),
      QString::fromStdString(detail.system_info.long_os_version),
      QString::fromStdString(detail.system_info.distribution_id),
      QString::fromStdString(detail.system_info.host_name),
      QString::fromStdString(detail.system_info.cpu_arch),
      tr("Unknown"),
      tr("Unknown"),
      tr("Unknown")};
  client_list.append(std::make_tuple(client, row));
  endInsertRows();
  u_geoip->queryIp(QString::fromStdString(detail.addr));
}

void Overview::onDisconnect(std::string addr) {
  for (int i = 0; i < client_list.count(); i++) {
    if (std::get<1>(client_list[i])[0] == addr) {
      beginRemoveRows(QModelIndex(), i, i);
      client_list.remove(i);
      endRemoveRows();
      break;
    }
  }
}

void Overview::onQueryIpFinished(QString addr, Common::GeoIpDetail detail) {
  for (int i = 0; i < client_list.count(); i++) {
    if (std::get<1>(client_list[i])[0] == addr) {
      std::get<1>(client_list[i])[9] = detail.country;
      std::get<1>(client_list[i])[10] = detail.region_name;
      std::get<1>(client_list[i])[11] = detail.isp;
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

void Overview::connectServer(std::shared_ptr<m0n1t0r::Server> _server) {
  server = _server;

  for (auto &client : server->allClient()) {
    onConnect(client);
  }

  server
      ->notify([this](const m0n1t0r::Server::Notification &notification) {
        if (notification.event == 0) {
          onConnect(server->client(notification.addr));
        } else if (notification.event == 1) {
          onDisconnect(notification.addr);
        }
        return true;
      })
      .detach();
}
} // namespace Model
