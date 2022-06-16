use cpp_core::{
    Ptr, StaticUpcast
};
use qt_core::{
    QBox, QObject, WindowType
};
use qt_widgets::{
    QWidget, QVBoxLayout, QApplication
};
use std::rc::Rc;
use crate::header::Header;
use crate::content::Content;

pub(crate) struct MainWindow {
    widget: QBox<QWidget>,
    // header: Rc<Header>,
    // content: Rc<Content>,
}

impl StaticUpcast<QObject> for MainWindow {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl MainWindow {
    pub fn new(app: Ptr<QApplication>) -> Rc<MainWindow> {
        unsafe {
            let widget = QWidget::new_0a();
            widget.set_window_flag_1a(WindowType::FramelessWindowHint);
            let main_layout = QVBoxLayout::new_1a(&widget);
            main_layout.set_contents_margins_4a(0, 0, 0, 0);
            main_layout.set_spacing(0);

            let header = Header::new(app);
            main_layout.add_widget(header.widget());

            let content = Content::new();
            main_layout.add_widget(content.widget());

            let tail_play_bar = QWidget::new_0a();
            main_layout.add_widget(&tail_play_bar);

            widget.show();
            let this = Rc::new(Self {
                widget,
                // header,
                // content,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
    }
}
