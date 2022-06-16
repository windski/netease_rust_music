use cpp_core::{
    StaticUpcast, Ptr
};
use qt_core::{
    QBox, QObject
};
use qt_widgets::{
    QWidget, QHBoxLayout, QListWidget
};

use std::rc::Rc;

pub(crate) struct Content {
    widget: QBox<QWidget>,
}

impl StaticUpcast<QObject> for Content {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Content {
    pub fn new() -> Rc<Content> {
        unsafe {
            let widget = QWidget::new_0a();
            let main_layout = QHBoxLayout::new_1a(&widget);
            main_layout.set_contents_margins_4a(0, 0, 0, 0);
            main_layout.set_spacing(0);

            let left_bar = QListWidget::new_0a();
            main_layout.add_widget(&left_bar);

            let play_list = QListWidget::new_0a();
            main_layout.add_widget(&play_list);

            let this = Rc::new(Self {
                widget
            });

            this
        }
    }

    pub unsafe fn widget(self: &Rc<Self>) -> Ptr<QWidget> {
        self.widget.as_ptr()
    }
}
