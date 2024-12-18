#include "Util/IpInfo.h"
#include <QMessageBox>
#include <spdlog/spdlog.h>

IpInfo::IpInfo(QObject *parent)
    : QObject(parent), net_manager(new QNetworkAccessManager(this)),
      rest_manager(new QRestAccessManager(net_manager, this)),
      factory(new QNetworkRequestFactory(QUrl("http://ip-api.com/json"))) {}

IpInfo::~IpInfo() {
  delete factory;
  rest_manager->deleteLater();
  net_manager->deleteLater();
}

void IpInfo::queryIp(QString addr) {
  auto request = factory->createRequest(addr.section(':', 0, 0));
  rest_manager->get(request, this, [this, addr](QRestReply &reply) {
    if (reply.isSuccess() == false) {
      emit queryIpError(tr("Request failed"));
      return;
    }

    auto doc = reply.readJson();
    if (doc.has_value() == false) {
      emit queryIpError(tr("Invalid JSON"));
      return;
    }

    auto object = doc->object();
    if (object["status"].toString() != "success") {
      auto message = object["message"].toString();
      spdlog::error("Failed to query IP: {}", message.toStdString());
      emit queryIpError(message);
      return;
    }

    emit queryIpFinished(
        addr, {object["country"].toString(), object["countryCode"].toString(),
               object["region"].toString(), object["regionName"].toString(),
               object["city"].toString(), object["zip"].toString(),
               object["lat"].toDouble(), object["lon"].toDouble(),
               object["timezone"].toString(), object["isp"].toString(),
               object["org"].toString(), object["as"].toString()});
  });
}
