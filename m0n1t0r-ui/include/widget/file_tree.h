#ifndef FILETREE_H
#define FILETREE_H

#include <QTreeWidget>
#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class FileTree;
}

namespace Widget {
class FileTree : public QWidget {
  Q_OBJECT

public:
  struct File {
    QString name;
  };

  struct Directory {
    QString name;
    QString path;
    QVector<Directory *> directories;
    QVector<File> files;
  };

  explicit FileTree(std::shared_ptr<m0n1t0r::Client> _client,
                    QWidget *parent = nullptr);
  ~FileTree();

private:
  Ui::FileTree *ui;
  std::shared_ptr<m0n1t0r::Client> client;

private Q_SLOTS:
  void on_treeWidget_dir_itemClicked(QTreeWidgetItem *item, int column);
  void on_treeWidget_dir_itemDoubleClicked(QTreeWidgetItem *item, int column);
};
} // namespace Widget

#endif // FILETREE_H