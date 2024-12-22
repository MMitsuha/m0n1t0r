#include "Widget/RemoteDesktopWidget.h"
#include "Util/Log.h"
#include "ui_RemoteDesktopWidget.h"
#include <QPainter>
#include <libyuv.h>
#include <spdlog/spdlog.h>

RemoteDesktopWidget::RemoteDesktopWidget(
    std::shared_ptr<m0n1t0r::Client> _client, QWidget *parent)
    : QWidget(parent), ui(new Ui::RemoteDesktopWidget), client(_client),
      screen_widget(new ScreenWidget(this)), m_termination(true) {
  ui->setupUi(this);
  ui->layout_screen->addWidget(screen_widget);

  initializeDecoder();

  connect(this, &RemoteDesktopWidget::frameReceived, this,
          &RemoteDesktopWidget::onFrameReceived);
  connect(this, &RemoteDesktopWidget::terminationChanged, this,
          &RemoteDesktopWidget::onTerminationChanged);
  connect(this, &RemoteDesktopWidget::frameUpdated, screen_widget,
          &ScreenWidget::onFrameUpdated);
}

RemoteDesktopWidget::~RemoteDesktopWidget() {
  setTermination(true);
  delete ui;
  screen_widget->deleteLater();
  uninitializeDecoder();
}

void RemoteDesktopWidget::on_pushButton_connect_clicked() {
  setTermination(false);
  try {
    client
        ->captureScreen(
            [=](const std::string &frame) {
              emit frameReceived(
                  std::make_shared<std::string>(std::move(frame)));
              return m_termination == false;
            },
            [=]() { setTermination(true); }, "raw")
        .detach();
  } catch (std::runtime_error &e) {
    logError(tr("Request capture permission failed"), e, tr("Screen Viewer"),
             this);
  }
}

bool RemoteDesktopWidget::termination() const { return m_termination; }

void RemoteDesktopWidget::setTermination(bool termination) {
  if (m_termination == termination) {
    return;
  }
  m_termination = termination;
  emit terminationChanged(termination);
}

void RemoteDesktopWidget::processFrame(uint8_t *data[3], SBufferInfo info) {
  auto buffer = std::shared_ptr<uint8_t>(
      new uint8_t[info.UsrData.sSystemBuffer.iWidth *
                  info.UsrData.sSystemBuffer.iHeight * 4]);

  libyuv::I420ToARGB(data[0], info.UsrData.sSystemBuffer.iStride[0], data[1],
                     info.UsrData.sSystemBuffer.iStride[1], data[2],
                     info.UsrData.sSystemBuffer.iStride[1], buffer.get(),
                     info.UsrData.sSystemBuffer.iWidth * 4,
                     info.UsrData.sSystemBuffer.iWidth,
                     info.UsrData.sSystemBuffer.iHeight);
  emit frameUpdated(buffer, info);
}

void RemoteDesktopWidget::onFrameReceived(std::shared_ptr<std::string> frame) {
  SBufferInfo info{};
  uint8_t *data[3]{};
  auto ret = decoder->DecodeFrameNoDelay((uint8_t *)frame->data(),
                                         frame->size(), data, &info);
  uint32_t num_of_frames_in_buffer = 0;

  if (ret != 0) {
    setTermination(true);
    logError(tr("Decode frame failed"), tr("Screen Viewer"), this);
    return;
  }

  if (info.iBufferStatus == 1) {
    processFrame(data, info);
  } else {
    decoder->GetOption(DECODER_OPTION_NUM_OF_FRAMES_REMAINING_IN_BUFFER,
                       &num_of_frames_in_buffer);
    for (uint32_t i = 0; i < num_of_frames_in_buffer; i++) {
      memset(data, 0, sizeof(data));
      memset(&info, 0, sizeof(info));
      decoder->FlushFrame(data, &info);
      if (info.iBufferStatus == 1) {
        processFrame(data, info);
      }
    }
  }
}

void RemoteDesktopWidget::on_pushButton_terminate_clicked() {
  setTermination(true);
}

void RemoteDesktopWidget::onTerminationChanged(bool) {
  ui->pushButton_connect->setEnabled(m_termination == true);
  ui->pushButton_terminate->setEnabled(m_termination == false);
}

void RemoteDesktopWidget::initializeDecoder() {
  SDecodingParam param{};

  WelsCreateDecoder(&decoder);
  decoder->Initialize(&param);
}

void RemoteDesktopWidget::uninitializeDecoder() {
  decoder->Uninitialize();
  WelsDestroyDecoder(decoder);
}

ScreenWidget::ScreenWidget(QWidget *parent)
    : QWidget(parent), painter(new QPainter(this)) {}

ScreenWidget::~ScreenWidget() { delete painter; }

void ScreenWidget::onFrameUpdated(std::shared_ptr<uint8_t> _frame,
                                  SBufferInfo info) {
  frame = _frame;
  image = QImage(frame.get(), info.UsrData.sSystemBuffer.iWidth,
                 info.UsrData.sSystemBuffer.iHeight,
                 info.UsrData.sSystemBuffer.iWidth * 4, QImage ::Format_ARGB32);

  repaint();
}

void ScreenWidget::paintEvent(QPaintEvent *event) {
  painter->begin(this);
  painter->drawImage(0, 0, image.scaled(size()));
  painter->end();
}
