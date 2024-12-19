#ifndef __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_INTERACTIVESHELLTAB_H_
#define __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_INTERACTIVESHELLTAB_H_

#include <QWidget>
#include <m0n1t0r-sdk.h>

namespace Ui {
class InteractiveShellTab;
}

class InteractiveShellTab : public QWidget {
  Q_OBJECT

  Q_PROPERTY(bool m_termination MEMBER m_termination READ termination WRITE
                 setTermination NOTIFY terminationChanged)

public:
  explicit InteractiveShellTab(std::shared_ptr<m0n1t0r::Client> client,
                               QWidget *parent = nullptr);
  ~InteractiveShellTab();

  bool termination() const;
  void setTermination(bool termination);

Q_SIGNALS:
  void outputReceived(QString output);
  void terminationChanged(bool termination);

public Q_SLOTS:
  void on_pushButton_run_program_clicked();
  void on_pushButton_terminate_clicked();
  void on_pushButton_send_clicked();
  void onOutputReceived(QString output);
  void onTerminationChanged(bool termination);

private:
  Ui::InteractiveShellTab *ui;
  std::shared_ptr<m0n1t0r::Client> client;
  msd::channel<std::string> channel;
  bool m_termination;
};

#endif // __M0N1T0R_M0N1T0R_UI_INCLUDE_WIDGET_INTERACTIVESHELLTAB_H_
