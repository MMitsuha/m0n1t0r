#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_SUBSCRIBER_CONNECTIONSUBSCRIBER_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_SUBSCRIBER_CONNECTIONSUBSCRIBER_H_

#include <QMetaObject>
#include <QObject>
#include <m0n1t0r-sdk.h>

class ConnectionSubscriber : public QObject {
  Q_OBJECT

public:
  explicit ConnectionSubscriber(QObject *parent = nullptr);
  ~ConnectionSubscriber();

Q_SIGNALS:
  void clientConnected(std::shared_ptr<m0n1t0r::Client> client);
  void clientDisconnected(std::string addr);
  void serverConnected(std::shared_ptr<m0n1t0r::Server> server);
  void serverDisconnected();

public Q_SLOTS:
  void onServerConnected(std::shared_ptr<m0n1t0r::Server> server);

private:
  std::shared_ptr<m0n1t0r::Server> server;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_SUBSCRIBER_CONNECTIONSUBSCRIBER_H_
