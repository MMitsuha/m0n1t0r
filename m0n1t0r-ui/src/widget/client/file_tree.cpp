#include "widget/file_tree.h"
#include "ui_file_tree.h"
#include <QMessageBox>

namespace Widget {
FileTree::FileTree(std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QWidget(parent), ui(new Ui::FileTree), client(_client) {
  ui->setupUi(this);

  ui->tableWidget_files->horizontalHeader()->setSectionResizeMode(
      QHeaderView::ResizeToContents);
  ui->treeWidget_dir->insertTopLevelItem(
      0, new QTreeWidgetItem(QStringList{"/", "/"}));
}

FileTree::~FileTree() { delete ui; }

void FileTree::on_treeWidget_dir_itemClicked(QTreeWidgetItem *item,
                                             int column) {
  try {
    auto files = client->listFiles(item->text(1).toStdString());
    int row = 0;

    ui->tableWidget_files->setRowCount(0);
    for (auto i = 0; i < files.size(); i++) {
      auto file = files[i];
      if (file.is_dir == false) {
        ui->tableWidget_files->insertRow(row);
        ui->tableWidget_files->setItem(
            row, 0, new QTableWidgetItem(QString::fromStdString(file.name)));
        ui->tableWidget_files->setItem(
            row, 1, new QTableWidgetItem(QString::fromStdString(file.path)));
        ui->tableWidget_files->setItem(
            row, 2,
            new QTableWidgetItem(file.is_symlink ? tr("YES") : tr("NO")));
        row++;
      }
    }
  } catch (const std::exception &e) {
    QMessageBox::critical(this, tr("Error"), e.what());
  }
}

void FileTree::on_treeWidget_dir_itemDoubleClicked(QTreeWidgetItem *item,
                                                   int column) {
  try {
    for (auto &files : client->listFiles(item->text(1).toStdString())) {
      if (files.is_dir == true) {
        item->addChild(new QTreeWidgetItem(
            QStringList{QString::fromStdString(files.name),
                        QString::fromStdString(files.path)}));
      }
    }
  } catch (const std::exception &e) {
    QMessageBox::critical(this, tr("Error"), e.what());
  }
}
} // namespace Widget
