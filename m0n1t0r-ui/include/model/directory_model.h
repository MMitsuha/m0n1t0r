#ifndef __DIR_MODEL_H__
#define __DIR_MODEL_H__

#include "network/client.h"
#include <QAbstractItemModel>

namespace Model {
class Directory : QObject {
  Q_OBJECT

public:
  explicit Directory(Network::Client *u_client, bool start,
                     Common::FileDetail current, Directory *parent = nullptr);
  ~Directory();

  Directory *child(int row) {
    if (start == false) {
      enumerate();
    }

    return children[row];
  }
  Directory *parent() { return parent_; }
  int childCount();
  int columnCount() const { return 5; };
  QVariant data(int column) const;
  int row() const;

public Q_SLOTS:
  void enumerate();

private:
  Network::Client *u_client;
  Directory *parent_;
  QVector<Directory *> children;
  bool start;
  Common::FileDetail current;
};

class DirectoryModel : public QAbstractItemModel {
  Q_OBJECT

public:
  explicit DirectoryModel(QString addr, QUrl base_url,
                          QObject *parent = nullptr);
  ~DirectoryModel();

  QVariant data(const QModelIndex &index, int role) const override;
  Qt::ItemFlags flags(const QModelIndex &index) const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;
  QModelIndex index(int row, int column,
                    const QModelIndex &parent = {}) const override;
  QModelIndex parent(const QModelIndex &index) const override;
  int rowCount(const QModelIndex &parent = {}) const override;
  int columnCount(const QModelIndex &parent = {}) const override;

private:
  Network::Client *u_client;
  Model::Directory *root;
};
} // namespace Model

#endif // __DIR_MODEL_H__
