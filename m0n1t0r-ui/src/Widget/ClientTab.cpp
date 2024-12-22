#include "Widget/ClientTab.h"
#include "Util/Log.h"
#include "ui_ClientTab.h"

ClientTab::ClientTab(std::shared_ptr<m0n1t0r::Client> _client,
                     ProcessTable *process_table, FileTree *file_tree,
                     InteractiveShellTab *interactive_shell_tab,
                     RemoteDesktopWidget *remote_desktop_widget,
                     QWidget *parent)
    : QWidget(parent), ui(new Ui::ClientTab), client(_client) {
  ui->setupUi(this);
  ui->layout_processes->addWidget(process_table);
  ui->layout_files->addWidget(file_tree);
  ui->layout_interactive_shell->addWidget(interactive_shell_tab);
  ui->layout_remote_desktop->addWidget(remote_desktop_widget);
  ui->checkBox_has_permission->setAttribute(Qt::WA_TransparentForMouseEvents);
  ui->checkBox_support->setAttribute(Qt::WA_TransparentForMouseEvents);

  connect(ui->pushButton_refresh, &QPushButton::clicked, process_table,
          &ProcessTable::refresh);
  connect(this, &ClientTab::permissionStatusChanged, this,
          &ClientTab::onPermissionStatusChanged);

  emit permissionStatusChanged();
}

ClientTab::~ClientTab() { delete ui; }

void ClientTab::on_pushButton_request_permission_clicked() {
  try {
    client->requestCapturePermission();
    emit permissionStatusChanged();
  } catch (std::runtime_error &e) {
    logError(tr("Request capture permission failed"), e, tr("Screen Viewer"),
             this);
  }
}

void ClientTab::onPermissionStatusChanged() {
  try {
    auto availability = client->canCaptureScreen();
    ui->checkBox_has_permission->setChecked(availability.has_permission);
    ui->checkBox_support->setChecked(availability.support);
  } catch (std::runtime_error &e) {
    logError(tr("Get screen availability failed"), e, tr("Screen Viewer"),
             this);
  }
}
