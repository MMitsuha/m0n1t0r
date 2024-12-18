#include "Util/Log.h"
#include <QMessageBox>
#include <spdlog/spdlog.h>

void logError(const QString &message, const QString &title, QWidget *parent) {
  // spdlog::error(QString("%1: %2").arg(title).arg(message).toStdString());
  QMessageBox::critical(parent, title, message);
}
void logError(const QString &_message, const std::runtime_error &e,
              const QString &title, QWidget *parent) {
  auto message = QString("%1: %2").arg(_message).arg(e.what());
  logError(message, title, parent);
}
