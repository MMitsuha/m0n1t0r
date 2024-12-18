#include "Model/ProcessModel.h"
#include "Util/Log.h"
#include <QWidget>

ProcessModel::ProcessModel(std::shared_ptr<m0n1t0r::Client> _client,
                           QObject *parent)
    : QAbstractTableModel(parent), client(_client) {
  connect(this, &ProcessModel::refresh, this, &ProcessModel::onRefresh);
  emit refresh();
}

ProcessModel::~ProcessModel() {}

QVariant ProcessModel::headerData(int section, Qt::Orientation orientation,
                                  int role) const {
  if (role == Qt::DisplayRole) {
    if (orientation == Qt::Horizontal) {
      return header_data[section];
    }
  }
  return QVariant();
}

int ProcessModel::rowCount(const QModelIndex &parent) const {
  return list.count();
}

int ProcessModel::columnCount(const QModelIndex &parent) const {
  return header_data.count();
}

QVariant ProcessModel::data(const QModelIndex &index, int role) const {
  if (role == Qt::DisplayRole) {
    return std::get<1>(list[index.row()])[index.column()];
  }
  return QVariant();
}

void ProcessModel::onRefresh() {
  beginResetModel();
  list.clear();
  try {
    for (auto &process : client->listProcesses()) {
      std::string command;
      for (auto &cmd : process.cmd) {
        command.append(cmd);
        command.push_back(' ');
      }
      QVector<QString> row = {
          QString::number(process.pid),
          QString::fromStdString(process.name),
          QString::fromStdString(process.exe),
          QString::fromStdString(command),
      };
      list.append(std::make_tuple(process.pid, row));
    }
  } catch (const std::runtime_error &e) {
    logError(tr("Failed to list process"), e, tr("Process Model"),
             qobject_cast<QWidget *>(parent()));
  }
  std::sort(list.begin(), list.end(),
            [](auto a, auto b) { return std::get<0>(a) < std::get<0>(b); });
  endResetModel();
}
