#ifndef NETWORK_CLIENT_H
#define NETWORK_CLIENT_H

#include "common/client.h"
#include <QMap>
#include <QTimer>
#include <QtNetwork/QNetworkAccessManager>
#include <QtNetwork/QNetworkRequestFactory>
#include <QtNetwork/QRestAccessManager>
#include <QtWebSockets/QtWebSockets>

namespace Network {
class Client : public QObject {
  Q_OBJECT

public:
  explicit Client(QObject *parent = 0);
  ~Client();

public Q_SLOTS:
  Client *fetchList();
  Client *subscribeNotification();
  Client *setBaseUrl(const QUrl &url);

Q_SIGNALS:
  void fetchListError(QString message);
  void receiveNotificationError(QString message);
  void connected(Common::ClientDetail detail);
  void disconnected(QString addr);

private:
  QRestAccessManager *rest_manager;
  QNetworkRequestFactory *factory;
  QWebSocket *web_socket;

  std::tuple<bool, QJsonValue>
  isRequestSucceed(QRestReply &reply, void (Client::*signal)(QString));
  void parseClientDetail(QJsonObject object);

private Q_SLOTS:
  void onWebSocketTextMessageReceived(QString message);
};
} // namespace Network

#endif // NETWORK_CLIENT_H
