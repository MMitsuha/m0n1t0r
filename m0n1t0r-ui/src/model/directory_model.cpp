#include "model/directory_model.h"

namespace Model {
Directory::Directory(Network::Client *client, QString _path, bool _start,
                     Common::FileDetail _current, Directory *_parent)
    : QObject(_parent), parent_(_parent), path(_path), start(_start),
      current(_current) {
  u_client = (new Network::Client(client->getAddr(), this))
                 ->setBaseUrl(client->getBaseUrl());

  connect(u_client, &Network::Client::receiveFileDetail, this,
          [this](Common::FileDetail detail) {
            children.push_back(
                new Directory(u_client, detail.path, false, detail, this));
          });

  if (start == true) {
    enumerate();
  }
}

void Directory::enumerate() {
  start = true;
  u_client->enumerateDirectory(path);
}

QVariant Directory::data(int column) const {
  if (column == 0) {
    return current.is_dir;
  } else if (column == 1) {
    return current.is_symlink;
  } else if (column == 2) {
    return current.name;
  } else if (column == 3) {
    return current.path;
  } else if (column == 4) {
    return current.size;
  }
  return {};
}

int Directory::row() const {
  if (parent_ != nullptr) {
    return parent_->children.indexOf(this);
  }
  return 0;
}

int Directory::childCount() {
  if (start == false) {
    enumerate();
  }

  return children.size();
}

DirectoryModel::DirectoryModel(QString addr, QObject *parent)
    : QAbstractItemModel(parent) {
  u_client = new Network::Client(addr, this);
}

DirectoryModel::~DirectoryModel() {}

QVariant DirectoryModel::data(const QModelIndex &index, int role) const {}

Qt::ItemFlags DirectoryModel::flags(const QModelIndex &index) const {}

QVariant DirectoryModel::headerData(int section, Qt::Orientation orientation,
                                    int role) const {}

QModelIndex DirectoryModel::index(int row, int column,
                                  const QModelIndex &parent) const {}

QModelIndex DirectoryModel::parent(const QModelIndex &index) const {}

int DirectoryModel::rowCount(const QModelIndex &parent) const {}

int DirectoryModel::columnCount(const QModelIndex &parent) const {}
} // namespace Model
