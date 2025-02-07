#include "Subscriber/ConnectionSubscriber.h"
#include "Util/Log.h"

ConnectionSubscriber::ConnectionSubscriber(QObject *parent)
    : QObject(parent), server(nullptr) {}

ConnectionSubscriber::~ConnectionSubscriber() {}

void ConnectionSubscriber::onServerConnected(
    std::shared_ptr<m0n1t0r::Server> _server) {
  server = _server;

  emit serverConnected(server);
  try {
    for (auto &client : server->allClient()) {
      emit clientConnected(client);
    }

    server
        ->notify([this](const m0n1t0r::Server::Notification &notification) {
          if (notification.event == 0) {
            emit clientConnected(server->client(notification.addr));
          } else if (notification.event == 1) {
            emit clientDisconnected(notification.addr);
          }
          return true;
        })
        .detach();

    server->notifyClose([this]() { emit serverDisconnected(); }).detach();
  } catch (std::runtime_error &e) {
    logError(tr("Failed to listen connection events on server"), e,
             tr("Subscribe Error"));
  }
}
