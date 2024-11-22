#include "window/main.h"
#include <QApplication>

int main(int argc, char *argv[]) {
  QApplication a(argc, argv);
  Window::Main w;
  w.show();
  return a.exec();
}
