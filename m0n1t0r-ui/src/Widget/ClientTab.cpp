#include "Widget/ClientTab.h"
#include "ui_ClientTab.h"

ClientTab::ClientTab(std::shared_ptr<m0n1t0r::Client> _client,
                     ProcessTable *process_table, FileTree *file_tree,
                     InteractiveShellTab *interactive_shell_tab,
                     QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab), client(_client) {
  ui->setupUi(this);
  ui->layout_processes->addWidget(process_table);
  ui->layout_files->addWidget(file_tree);
  ui->layout_interactive_shell->addWidget(interactive_shell_tab);

  connect(ui->pushButton_refresh, &QPushButton::clicked, process_table,
          &ProcessTable::refresh);
}

ClientTab::~ClientTab() { delete ui; }
