#include "model/directory_model.h"

namespace Model {
Directory::~Directory() {
  for (auto *child : children) {
    delete child;
  }
}

Directory::Directory(Network::Client *client, bool _start,
                     Common::FileDetail _current, Directory *_parent)
    : QObject(_parent), parent_(_parent), start(_start), current(_current) {
  u_client = (new Network::Client(client->getAddr(), this))
                 ->setBaseUrl(client->getBaseUrl());

  connect(u_client, &Network::Client::receiveFileDetail, this,
          [this](Common::FileDetail detail) {
            children.push_back(new Directory(u_client, false, detail, this));
          });

  if (start == true) {
    enumerate();
  }
}

void Directory::enumerate() {
  start = true;
  u_client->enumerateDirectory(current.path);
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

DirectoryModel::DirectoryModel(QString addr, QUrl base_url, QObject *parent)
    : QAbstractItemModel(parent) {
  u_client = (new Network::Client(addr, this))->setBaseUrl(base_url);
  root = new Directory(u_client, false,
                       Common::FileDetail{true, false, "/", "/", 0}, nullptr);
}

DirectoryModel::~DirectoryModel() {}

QVariant DirectoryModel::data(const QModelIndex &index, int role) const {
  if (index.isValid() == false || role != Qt::DisplayRole)
    return {};

  const auto *item = (Directory *)index.internalPointer();
  return item->data(index.column());
}

Qt::ItemFlags DirectoryModel::flags(const QModelIndex &index) const {
  return index.isValid() ? QAbstractItemModel::flags(index)
                         : Qt::ItemFlags(Qt::NoItemFlags);
}

QVariant DirectoryModel::headerData(int section, Qt::Orientation orientation,
                                    int role) const {
  return orientation == Qt::Horizontal && role == Qt::DisplayRole
             ? root->data(section)
             : QVariant{};
}

QModelIndex DirectoryModel::index(int row, int column,
                                  const QModelIndex &parent) const {
  if (hasIndex(row, column, parent) == false) {
    return {};
  }

  Directory *parentItem =
      parent.isValid() ? (Directory *)parent.internalPointer() : root;

  if (auto *childItem = parentItem->child(row)) {
    return createIndex(row, column, childItem);
  }

  return {};
}

QModelIndex DirectoryModel::parent(const QModelIndex &index) const {
  if (index.isValid() == false) {
    return {};
  }

  auto *childItem = (Directory *)index.internalPointer();
  Directory *parentItem = childItem->parent();

  return parentItem != root ? createIndex(parentItem->row(), 0, parentItem)
                            : QModelIndex{};
}

int DirectoryModel::rowCount(const QModelIndex &parent) const {
  if (parent.column() > 0) {
    return 0;
  }

  Directory *parentItem =
      parent.isValid() ? (Directory *)parent.internalPointer() : root;

  return parentItem->childCount();
}

int DirectoryModel::columnCount(const QModelIndex &parent) const {
  if (parent.isValid())
    return ((Directory *)parent.internalPointer())->columnCount();
  return root->columnCount();
}
} // namespace Model
