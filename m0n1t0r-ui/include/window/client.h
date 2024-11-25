#ifndef WINDOW_CLIENT_H
#define WINDOW_CLIENT_H

#include <QDialog>
#include <QUrl>

namespace Ui {
class Client;
}

namespace Window {
class Client : public QDialog {
  Q_OBJECT

public:
  explicit Client(QUrl base_url, QWidget *parent = nullptr);
  ~Client();

private:
  Ui::Client *ui;
  QUrl base_url;
};
} // namespace Window

#endif // WINDOW_CLIENT_H
