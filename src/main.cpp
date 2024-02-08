#include "main_window.h"
int main() {
    const auto main_window = std::make_unique<MainWindow>();
    main_window->create()->run();
    return 0;
}
