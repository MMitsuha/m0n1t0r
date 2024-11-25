#include "network/ip.h"
#include <QMessageBox>

namespace Network {
GeoIp::GeoIp(QObject *parent) : QObject(parent) {
  rest_manager = new QRestAccessManager(new QNetworkAccessManager(this), this);
  factory = new QNetworkRequestFactory(QUrl("http://ip-api.com/json"));
  //   auto reportError = [this](QString message) {
  //     QMessageBox::critical(qobject_cast<QWidget *>(this), tr("Error"),
  //     message);
  //   };

  //   connect(this, &Network::GeoIp::queryIpError, this, reportError);
}

GeoIp::~GeoIp() {}

void GeoIp::queryIp(QString addr) {
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
      emit queryIpError(object["message"].toString());
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
} // namespace Network
