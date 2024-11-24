#ifndef __IP_H__
#define __IP_H__

#include "common/ip.h"
#include <QMap>
#include <QTimer>
#include <QtNetwork/QNetworkAccessManager>
#include <QtNetwork/QNetworkRequestFactory>
#include <QtNetwork/QRestAccessManager>
#include <QtWebSockets/QtWebSockets>

namespace Network {
class GeoIp : public QObject {
  Q_OBJECT

public:
  explicit GeoIp(QObject *parent = 0);
  ~GeoIp();

public Q_SLOTS:
  void queryIp(QString addr);

Q_SIGNALS:
  void queryIpFinished(QString addr, Common::GeoIpDetail detail);
  void queryIpError(QString message);

private:
  QRestAccessManager *rest_manager;
  QNetworkRequestFactory *factory;

private Q_SLOTS:
};
} // namespace Network

#endif // __IP_H__
