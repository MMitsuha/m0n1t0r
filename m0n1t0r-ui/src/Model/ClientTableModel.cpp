#include "Model/ClientTableModel.h"
#include "Util/Log.h"

ClientTableModel::ClientTableModel(ConnectionSubscriber *_subscriber,
                                   QObject *parent)
    : QAbstractTableModel(parent), subscriber(_subscriber),
      ip_info(new IpInfo) {
  connect(subscriber, &ConnectionSubscriber::clientConnected, this,
          &ClientTableModel::onClientConnected);
  connect(subscriber, &ConnectionSubscriber::clientDisconnected, this,
          &ClientTableModel::onClientDisconnected);
  connect(subscriber, &ConnectionSubscriber::serverDisconnected, this,
          &ClientTableModel::onServerDisconnected);
  connect(ip_info, &IpInfo::queryIpFinished, this,
          &ClientTableModel::onQueryIpFinished);
}

ClientTableModel::~ClientTableModel() { ip_info->deleteLater(); }

int ClientTableModel::rowCount(const QModelIndex &parent) const {
  return client_list.count();
}

int ClientTableModel::columnCount(const QModelIndex &parent) const {
  return header_data.count();
}

QVariant ClientTableModel::data(const QModelIndex &index, int role) const {
  if (role == Qt::DisplayRole) {
    return std::get<1>(client_list[index.row()])[index.column()];
  }
  return QVariant();
}

QVariant ClientTableModel::headerData(int section, Qt::Orientation orientation,
                                      int role) const {
  if (orientation == Qt::Horizontal) {
    if (role == Qt::DisplayRole) {
      return header_data[section];
    }
  }
  return QVariant();
}

void ClientTableModel::onClientConnected(
    std::shared_ptr<m0n1t0r::Client> client) {
  try {
    auto detail = client->getDetail();
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

    beginInsertRows(QModelIndex(), rowCount(), rowCount());
    client_list.append(std::make_tuple(client, row));
    endInsertRows();
  } catch (std::runtime_error &e) {
    logError(tr("Failed to get client detail"), e, tr("Connection Error"));
  }
  ip_info->queryIp(QString::fromStdString(client->getAddr()));
}

void ClientTableModel::onClientDisconnected(std::string addr) {
  for (int i = 0; i < client_list.count(); i++) {
    if (std::get<1>(client_list[i])[0] == addr) {
      beginRemoveRows(QModelIndex(), i, i);
      client_list.remove(i);
      endRemoveRows();
      break;
    }
  }
}

void ClientTableModel::onServerDisconnected() { clear(); }

void ClientTableModel::clear() {
  beginResetModel();
  client_list.clear();
  endResetModel();
}

std::shared_ptr<m0n1t0r::Client> ClientTableModel::getClientByRow(int row) {
  return std::get<0>(client_list[row]);
}

void ClientTableModel::onQueryIpFinished(QString addr, IpInfo::Detail detail) {
  for (qsizetype i = 0; i < client_list.count(); i++) {
    if (std::get<1>(client_list[i])[0] == addr) {
      std::get<1>(client_list[i])[9] = detail.country;
      std::get<1>(client_list[i])[10] = detail.region_name;
      std::get<1>(client_list[i])[11] = detail.isp;
      emit dataChanged(index(i, 9), index(i, 11));
      break;
    }
  }
}
