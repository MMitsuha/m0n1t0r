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
      QMessageBox::critical(qobject_cast<QWidget *>(parent()),
                            tr("Network::Client Error"), tr("Request failed"));
      return;
    }

    auto doc = reply.readJson();
    if (doc.has_value() == false) {
      QMessageBox::critical(qobject_cast<QWidget *>(parent()),
                            tr("Network::Client Error"), tr("Invalid JSON"));
      return;
    }

    auto object = doc->object();
    if (object["code"].toInt() != 0) {
      QMessageBox::critical(qobject_cast<QWidget *>(parent()),
                            "Network::Client Error", object["body"].toString());
      return;
    }

    QVector<Common::ClientDetail> ret;
    auto array = object["body"].toArray();
    for (auto value : array) {
      auto system_info = value.toObject()["system_info"].toObject();
      auto detail =
          ret.emplace_back(value.toObject()["addr"].toString(),
                           value.toObject()["version"].toString(),
                           value.toObject()["target_platform"].toString(),
                           system_info["name"].toString(),
                           system_info["kernel_version"].toString(),
                           system_info["long_os_version"].toString(),
                           system_info["distribution_id"].toString(),
                           system_info["host_name"].toString(),
                           system_info["cpu_arch"].toString());
    }

    emit getListFinished(ret);
  });
}
} // namespace Network
