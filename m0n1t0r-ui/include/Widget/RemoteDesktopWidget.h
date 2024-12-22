#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_REMOTEDESKTOPWIDGET_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_REMOTEDESKTOPWIDGET_H_

#include <QGraphicsScene>
#include <QWidget>
#include <m0n1t0r-sdk.h>
#include <wels/codec_api.h>

namespace Ui {
class RemoteDesktopWidget;
}

class ScreenWidget : public QWidget {
  Q_OBJECT

public:
  explicit ScreenWidget(QWidget *parent = nullptr);
  ~ScreenWidget();

public Q_SLOTS:
  void onFrameUpdated(std::shared_ptr<uint8_t> frame, SBufferInfo info);

protected:
  void paintEvent(QPaintEvent *event) override;

private:
  std::shared_ptr<uint8_t> frame;
  QImage image;
  QPainter *painter;
};

class RemoteDesktopWidget : public QWidget {
  Q_OBJECT

  Q_PROPERTY(bool m_termination MEMBER m_termination READ termination WRITE
                 setTermination NOTIFY terminationChanged)

public:
  explicit RemoteDesktopWidget(std::shared_ptr<m0n1t0r::Client> client,
                               QWidget *parent = nullptr);
  ~RemoteDesktopWidget();

  bool termination() const;
  void setTermination(bool termination);

Q_SIGNALS:
  void terminationChanged(bool termination);
  void frameReceived(std::shared_ptr<std::string> frame);
  void frameUpdated(std::shared_ptr<uint8_t> frame, SBufferInfo info);

public Q_SLOTS:
  void on_pushButton_connect_clicked();
  void onFrameReceived(std::shared_ptr<std::string> frame);
  void on_pushButton_terminate_clicked();
  void onTerminationChanged(bool termination);
  void processFrame(uint8_t *data[3], SBufferInfo info);

private:
  Ui::RemoteDesktopWidget *ui;
  std::shared_ptr<m0n1t0r::Client> client;
  bool m_termination;
  ISVCDecoder *decoder;
  ScreenWidget *screen_widget;

  void initializeDecoder();
  void uninitializeDecoder();
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_REMOTEDESKTOPWIDGET_H_
