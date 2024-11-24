#ifndef CLIENT_1_H
#define CLIENT_1_H

#include "common/client.h"
#include <QMap>
#include <QtNetwork/QNetworkAccessManager>
#include <QtNetwork/QNetworkRequestFactory>
#include <QtNetwork/QRestAccessManager>

namespace Network {
class Client : public QObject {
  Q_OBJECT

public:
  explicit Client(QObject *parent = 0);
  ~Client();

  void setBaseUrl(const QUrl &url);

public Q_SLOTS:
  void getList();

Q_SIGNALS:
  void getListFinished(QVector<Common::ClientDetail> list);
  void getListError(QString message);

private:
  QNetworkAccessManager *n_manager;
  QRestAccessManager *r_manager;
  QNetworkRequestFactory *factory;
};
} // namespace Network

#endif // CLIENT_1_H
