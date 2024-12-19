#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_LOG_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_LOG_H_

#include <QObject>
#include <QString>

void logError(const QString &message,
              const QString &title = QObject::tr("Undefined"),
              QWidget *parent = nullptr);
void logError(const QString &_message, const std::exception &e,
              const QString &title = QObject::tr("Undefined"),
              QWidget *parent = nullptr);

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_LOG_H_
