#include "Widget/InteractiveShellTab.h"
#include "Util/Log.h"
#include "ui_InteractiveShellTab.h"

InteractiveShellTab::InteractiveShellTab(
    std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QWidget(parent), ui(new Ui::InteractiveShellTab), client(_client),
      m_termination(true) {
  ui->setupUi(this);

  connect(this, &InteractiveShellTab::outputReceived, this,
          &InteractiveShellTab::onOutputReceived);
  connect(this, &InteractiveShellTab::terminationChanged, this,
          &InteractiveShellTab::onTerminationChanged);
}

InteractiveShellTab::~InteractiveShellTab() {
  setTermination(true);
  delete ui;
}

void InteractiveShellTab::on_pushButton_terminate_clicked() {
  setTermination(true);
}

void InteractiveShellTab::on_pushButton_run_program_clicked() {
  setTermination(false);
  auto program = ui->lineEdit_program->text();
  try {
    client
        ->executeCommandInteractive(
            program.toStdString(),
            [=](const std::string &output) {
              emit outputReceived(
                  std::make_shared<QString>(QString::fromStdString(output)));
              return m_termination == false;
            },
            [=]() { setTermination(true); }, channel)
        .detach();
  } catch (std::runtime_error &e) {
    logError(tr("Run program failed"), e, tr("Interactive Shell"), this);
  }
}

void InteractiveShellTab::on_pushButton_send_clicked() {
  auto input = ui->lineEdit_command->text().append("\n");
  ui->plainTextEdit_output->insertPlainText(">> ");
  ui->plainTextEdit_output->insertPlainText(input);
  ui->plainTextEdit_output->ensureCursorVisible();
  ui->lineEdit_command->clear();
  channel << input.toStdString();
}

void InteractiveShellTab::onOutputReceived(std::shared_ptr<QString> output) {
  ui->plainTextEdit_output->insertPlainText("<< ");
  ui->plainTextEdit_output->insertPlainText(*output);
  ui->plainTextEdit_output->ensureCursorVisible();
}

void InteractiveShellTab::onTerminationChanged(bool) {
  ui->pushButton_run_program->setEnabled(m_termination == true);
  ui->pushButton_send->setEnabled(m_termination == false);
  ui->pushButton_terminate->setEnabled(m_termination == false);
  ui->lineEdit_command->setEnabled(m_termination == false);
  ui->lineEdit_program->setEnabled(m_termination == true);
}

bool InteractiveShellTab::termination() const { return m_termination; }

void InteractiveShellTab::setTermination(bool termination) {
  if (m_termination == termination) {
    return;
  }
  m_termination = termination;
  emit terminationChanged(termination);
}
