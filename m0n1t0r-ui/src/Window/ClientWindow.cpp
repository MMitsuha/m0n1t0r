#include "Window/ClientWindow.h"
#include "ui_ClientWindow.h"

ClientWindow::ClientWindow(std::shared_ptr<m0n1t0r::Client> _client,
                           QWidget *parent)
    : QDialog(parent), ui(new Ui::ClientWindow), client(_client),
      process_table(new ProcessTable(client, this)),
      file_tree(new FileTree(client, this)),
      interactive_shell_tab(new InteractiveShellTab(client, this)),
      client_tab(new ClientTab(client, process_table, file_tree,
                               interactive_shell_tab, this)) {
  ui->setupUi(this);
  ui->gridLayout->addWidget(client_tab);

  setWindowTitle(
      tr("Client - %1").arg(QString::fromStdString(client->getAddr())));
}

ClientWindow::~ClientWindow() {
  delete ui;
  client_tab->deleteLater();
  process_table->deleteLater();
  file_tree->deleteLater();
  interactive_shell_tab->deleteLater();
}
