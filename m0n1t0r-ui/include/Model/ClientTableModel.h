#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_CLIENTTABLEMODEL_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_CLIENTTABLEMODEL_H_

#include "Subscriber/ConnectionSubscriber.h"
#include "Util/IpInfo.h"
#include <QAbstractTableModel>
#include <QArrayData>
#include <m0n1t0r-sdk-cpp/m0n1t0r-sdk.h>

class ClientTableModel : public QAbstractTableModel {
  Q_OBJECT

  const QVector<QString> header_data = {
      tr("Address"),          tr("Version"),
      tr("Target Platform"),  tr("Name"),
      tr("Kernel Version"),   tr("Long OS Version"),
      tr("Distribution ID"),  tr("Host Name"),
      tr("CPU Architecture"), tr("Country"),
      tr("Region Name"),      tr("ISP"),
  };

public:
  explicit ClientTableModel(ConnectionSubscriber *subscriber,
                            QObject *parent = nullptr);
  ~ClientTableModel();

  int rowCount(const QModelIndex &parent = {}) const override;
  int columnCount(const QModelIndex &parent = {}) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;

  std::shared_ptr<m0n1t0r::Client> getClientByRow(int row);

public Q_SLOTS:
  void onClientConnected(std::shared_ptr<m0n1t0r::Client> client);
  void onClientDisconnected(std::string addr);
  void onServerDisconnected();
  void onQueryIpFinished(QString addr, IpInfo::Detail detail);
  void clear();

private:
  QVector<std::tuple<std::shared_ptr<m0n1t0r::Client>, QVector<QString>>>
      client_list;
  ConnectionSubscriber *subscriber;
  IpInfo *ip_info;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_CLIENTTABLEMODEL_H_
