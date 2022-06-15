use qt_widgets::{
    QApplication, QWidget, QHBoxLayout, QPushButton, QLineEdit, QLabel
};
use qt_core::{
    QBox, QString, CursorShape, q_init_resource
};
use qt_gui::{
    QCursor, QPixmap
};

use std::rc::Rc;

struct Header {
    widget: QBox<QWidget>,
    redo_button: QBox<QPushButton>,
    undo_button: QBox<QPushButton>,
    search_edit: QBox<QLineEdit>,
}

impl Header {
    fn new() -> Rc<Header> {
        unsafe {
            let content_style = QString::from_std_str(
                "QWidget {\
                    background-color: #ec4141;\
                }"
            );
            let button_style = QString::from_std_str(
                "QPushButton {\
                    border: none;\
                    min-width: 36px;\
                    min-height: 36px;\
                    background-color: #d83b3b;\
                    border-radius: 18px;\
                }"
            );
            let edit_style = QString::from_std_str(
                "QLineEdit {\
                    border: none;\
                    min-width: 241px;\
                    min-height: 48px;\
                    background-color: #e13e3e;\
                    border-radius: 24px;\
                }\
                QLineEdit::indicator {\
                    width: 13px;
                    height: 13px;
                }"
            );

            let widget = QWidget::new_0a();
            widget.set_style_sheet(&content_style);

            let layout = QHBoxLayout::new_1a(&widget);
            layout.set_spacing(0);
            layout.set_contents_margins_4a(0, 24, 0, 24);
            layout.add_spacing(26);

            let title_icon = QLabel::new();
            title_icon.set_fixed_height(45);
            title_icon.set_pixmap(&QPixmap::from_q_string(&QString::from_std_str(":/image/netease_title_icon.png")));
            layout.add_widget(&title_icon);

            layout.add_spacing(122);

            let undo_button = QPushButton::new();
            undo_button.set_cursor(&QCursor::from_cursor_shape(CursorShape::PointingHandCursor));
            undo_button.set_style_sheet(&button_style);
            layout.add_widget(&undo_button);

            layout.add_spacing(12);

            let redo_button = QPushButton::new();
            redo_button.set_cursor(&QCursor::from_cursor_shape(CursorShape::PointingHandCursor));
            redo_button.set_style_sheet(&button_style);
            layout.add_widget(&redo_button);

            layout.add_spacing(18);

            let search_edit = QLineEdit::new();
            search_edit.set_style_sheet(&edit_style);
            layout.add_widget(&search_edit);
    
            let this = Rc::new(Self {
                widget,
                redo_button,
                undo_button,
                search_edit,
            });
            this.init();
            this
        }
    }

    fn init(self: &Rc<Self>) {

    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.widget.show();
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        q_init_resource!("resources");
        let widget = Header::new();
        widget.show();
        QApplication::exec()
    })
}
