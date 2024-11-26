#include "widget/fileview.h"
#include "ui_fileview.h"

namespace Widget {
FileView::FileView(QString addr, QUrl base_url, QWidget *parent)
    : QWidget(parent), ui(new Ui::FileView) {
  ui->setupUi(this);

  m_directory = new Model::DirectoryModel(addr, base_url, this);

  ui->treeView_dirs->setModel(m_directory);
}

FileView::~FileView() { delete ui; }
} // namespace Widget
