use qt_widgets::QApplication;
use qt_core::q_init_resource;

mod content;
mod header;
mod mainwindow;

use mainwindow::MainWindow;

fn main() {
    QApplication::init(|app| unsafe {
        q_init_resource!("resources");
        let _main_window = MainWindow::new(app);
        QApplication::exec()
    })
}
