#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_IPINFO_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_IPINFO_H_

#include <QMap>
#include <QTimer>
#include <QtNetwork/QNetworkAccessManager>
#include <QtNetwork/QNetworkRequestFactory>
#include <QtNetwork/QRestAccessManager>
#include <QtWebSockets/QtWebSockets>

class IpInfo : public QObject {
  Q_OBJECT

public:
  struct Detail {
    QString country;
    QString country_code;
    QString region;
    QString region_name;
    QString city;
    QString zip;
    double lat;
    double lon;
    QString timezone;
    QString isp;
    QString org;
    QString as;
  };

  explicit IpInfo(QObject *parent = 0);
  ~IpInfo();

public Q_SLOTS:
  void queryIp(QString addr);

Q_SIGNALS:
  void queryIpFinished(QString addr, Detail detail);
  void queryIpError(QString message);

private:
  QNetworkAccessManager *net_manager;
  QRestAccessManager *rest_manager;
  QNetworkRequestFactory *factory;

private Q_SLOTS:
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_UTIL_IPINFO_H_
