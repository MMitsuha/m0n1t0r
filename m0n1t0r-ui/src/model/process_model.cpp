#include "model/process_model.h"

namespace Model {
Process::Process(std::shared_ptr<m0n1t0r::Client> _client, QObject *parent)
    : QAbstractTableModel(parent), client(_client) {
  refresh();
}

Process::~Process() {}

QVariant Process::headerData(int section, Qt::Orientation orientation,
                             int role) const {
  if (role == Qt::DisplayRole) {
    if (orientation == Qt::Horizontal) {
      return header_data[section];
    }
  }
  return QVariant();
}

int Process::rowCount(const QModelIndex &parent) const {
  return process_list.count();
}

int Process::columnCount(const QModelIndex &parent) const {
  return header_data.count();
}

QVariant Process::data(const QModelIndex &index, int role) const {
  if (role == Qt::DisplayRole) {
    return std::get<1>(process_list[index.row()])[index.column()];
  }
  return QVariant();
}

void Process::refresh() {
  beginResetModel();
  process_list.clear();
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
    process_list.append(std::make_tuple(process.pid, row));
  }
  endResetModel();
}
} // namespace Model
