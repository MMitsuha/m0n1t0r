

#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESSMODEL_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESSMODEL_H_

#include <QAbstractTableModel>
#include <QArrayData>
#include <m0n1t0r-sdk.h>

class ProcessModel : public QAbstractTableModel {
  Q_OBJECT

public:
  const QVector<QString> header_data = {
      tr("Process ID"),
      tr("Name"),
      tr("Executable"),
      tr("Command"),
  };

  explicit ProcessModel(std::shared_ptr<m0n1t0r::Client> client,
                        QObject *parent = nullptr);
  ~ProcessModel();

  int rowCount(const QModelIndex &parent = {}) const override;
  int columnCount(const QModelIndex &parent = {}) const override;
  QVariant data(const QModelIndex &index,
                int role = Qt::DisplayRole) const override;
  QVariant headerData(int section, Qt::Orientation orientation,
                      int role = Qt::DisplayRole) const override;

Q_SIGNALS:
  void refresh();

public Q_SLOTS:
  void onRefresh();

private:
  std::shared_ptr<m0n1t0r::Client> client;
  QVector<std::tuple<uint64_t, QVector<QString>>> list;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_MODEL_PROCESSMODEL_H_
