#include "network/client.h"
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QMessageBox>
#include <QRestReply>

namespace Network {
Client::Client(QObject *parent) : QObject(parent) {
  n_manager = new QNetworkAccessManager(this);
  r_manager = new QRestAccessManager(n_manager, this);
  factory = new QNetworkRequestFactory();
}

Client::~Client() {}

void Client::setBaseUrl(const QUrl &url) { factory->setBaseUrl(url); }

void Client::getList() {
  auto request = factory->createRequest();
  r_manager->get(request, this, [this](QRestReply &reply) {
    if (reply.isSuccess() == false) {
      emit getListError(reply.errorString());
      return;
    }

    auto doc = reply.readJson();
    if (doc.has_value() == false) {
      emit getListError(tr("Invalid JSON"));
      return;
    }

    auto object = doc->object();
    if (object["code"].toInt() != 0) {
      emit getListError(object["body"].toString());
      return;
    }

    QVector<Common::ClientDetail> ret;
    auto array = object["body"].toArray();
    for (auto value : array) {
      auto system_info = value.toObject()["system_info"].toObject();
      Common::ClientDetail detail = {
          value.toObject()["addr"].toString(),
          value.toObject()["version"].toString(),
          value.toObject()["target_platform"].toString(),
          system_info["name"].toString(),
          system_info["kernel_version"].toString(),
          system_info["long_os_version"].toString(),
          system_info["distribution_id"].toString(),
          system_info["host_name"].toString(),
          system_info["cpu_arch"].toString()};
      ret.emplace_back(detail);
    }
    emit getListFinished(ret);
  });
}
} // namespace Network
