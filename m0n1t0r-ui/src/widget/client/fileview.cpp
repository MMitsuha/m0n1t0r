#include "widget/fileview.h"
#include "ui_fileview.h"

namespace Widget {
FileView::FileView(QWidget *parent) : QWidget(parent), ui(new Ui::FileView) {
  ui->setupUi(this);
}

FileView::~FileView() { delete ui; }
} // namespace Widget
