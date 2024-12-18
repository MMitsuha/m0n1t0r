#include "Widget/ClientTab.h"
#include "ui_ClientTab.h"

ClientTab::ClientTab(ProcessTable *process_table, FileTree *file_tree,
                     QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab) {
  ui->setupUi(this);
  ui->layout_processes->addWidget(process_table);
  ui->layout_files->addWidget(file_tree);

  connect(ui->pushButton_refresh, &QPushButton::clicked, process_table,
          &ProcessTable::refresh);
}

ClientTab::~ClientTab() { delete ui; }
