#ifndef CONNECT_H
#define CONNECT_H

#include <QDialog>
#include <QUrl>

namespace Ui {
class Connect;
}

namespace Window {
class Connect : public QDialog {
  Q_OBJECT

public:
  explicit Connect(QWidget *parent = nullptr);
  ~Connect();

Q_SIGNALS:
  void connectServer(QUrl url, QString password);

private:
  Ui::Connect *ui;

private Q_SLOTS:
  void on_pushButton_connect_clicked();
};
} // namespace Window

#endif // CONNECT_H
