#include "network/client.h"
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QMessageBox>
#include <QRestReply>

namespace Network {
Client::Client(QString _addr, QObject *parent) : QObject(parent), addr(_addr) {
  rest_manager = new QRestAccessManager(new QNetworkAccessManager(this), this);
  factory = new QNetworkRequestFactory();
  web_socket =
      new QWebSocket(QString(), QWebSocketProtocol::VersionLatest, this);
  auto reportError = [this](QString message) {
    QMessageBox::critical(qobject_cast<QWidget *>(this), tr("Error"), message);
  };

  connect(this, &Network::Client::fetchListError, this, reportError);
  connect(this, &Network::Client::receiveNotificationError, this, reportError);
  connect(web_socket, &QWebSocket::textMessageReceived, this,
          &Network::Client::onWebSocketTextMessageReceived);
  connect(web_socket, &QWebSocket::disconnected, this,
          &Network::Client::disconnectedServer);
}

Client::~Client() {}

Client *Client::setBaseUrl(const QUrl &url) {
  factory->setBaseUrl(url.resolved(QUrl("client")));
  return this;
}

Client *Client::fetchList() {
  auto request = factory->createRequest();
  rest_manager->get(request, this, [this](QRestReply &reply) {
    auto [succeed, body] = isRequestSucceed(reply, &Client::fetchListError);
    if (succeed == false) {
      return;
    }

    auto array = body.toArray();
    for (auto value : array) {
      parseClientDetail(value.toObject());
    }
  });
  return this;
}

std::tuple<bool, QJsonValue>
Client::isRequestSucceed(QRestReply &reply, void (Client::*signal)(QString)) {
  if (reply.isSuccess() == false) {
    emit(this->*signal)(reply.errorString());
    return std::make_tuple(false, QJsonObject());
  }

  auto doc = reply.readJson();
  if (doc.has_value() == false) {
    emit(this->*signal)(tr("Invalid JSON"));
    return std::make_tuple(false, QJsonObject());
  }

  auto object = doc->object();
  if (object["code"].toInt() != 0) {
    emit(this->*signal)(object["body"].toString());
    return std::make_tuple(false, QJsonObject());
  }
  return std::make_tuple(true, object["body"]);
}

Client *Client::subscribeNotification() {
  auto url = factory->baseUrl().resolved(QUrl("client/notify"));
  url.setScheme("ws");
  web_socket->open(url);
  return this;
}

void Client::parseClientDetail(QJsonObject object) {
  auto system_info = object["system_info"].toObject();
  Common::ClientDetail detail = {object["addr"].toString(),
                                 object["version"].toString(),
                                 object["target_platform"].toString(),
                                 system_info["name"].toString(),
                                 system_info["kernel_version"].toString(),
                                 system_info["long_os_version"].toString(),
                                 system_info["distribution_id"].toString(),
                                 system_info["host_name"].toString(),
                                 system_info["cpu_arch"].toString()};
  emit connected(detail);
}

void Client::parseFileDetail(QJsonArray array) {
  for (auto value : array) {
    auto object = value.toObject();
    Common::FileDetail detail = {
        object["is_dir"].toBool(),  object["is_symlink"].toBool(),
        object["name"].toString(),  object["path"].toString(),
        object["size"].toInteger(),
    };
    emit receiveFileDetail(detail);
  }
}

void Client::onWebSocketTextMessageReceived(QString message) {
  auto object = QJsonDocument::fromJson(message.toUtf8()).object();
  if (object["event"].toString() == "Connect") {
    auto addr = object["addr"].toString();
    auto request = factory->createRequest(addr);
    rest_manager->get(request, this, [this](QRestReply &reply) {
      auto [succeed, body] = isRequestSucceed(reply, &Client::fetchListError);
      if (succeed == false) {
        return;
      }
      parseClientDetail(body.toObject());
    });
  } else if (object["event"].toString() == "Disconnect") {
    emit disconnected(object["addr"].toString());
  }
}

Client *Client::enumerateDirectory(QString path) {
  auto request = factory->createRequest(
      QString("%1/fs/dir/").arg(addr).append(QUrl::toPercentEncoding(path)));
  rest_manager->get(request, this, [this](QRestReply &reply) {
    auto [succeed, body] = isRequestSucceed(reply, &Client::fetchListError);
    if (succeed == false) {
      return;
    }

    parseFileDetail(body.toArray());
  });
  return this;
}
} // namespace Network
