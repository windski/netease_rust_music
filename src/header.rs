
use cpp_core::{
    StaticUpcast, Ptr
};
use qt_core::{
    QBox, QObject, SlotNoArgs, slot, QString, CursorShape, qs
};
use qt_widgets::{
    QWidget, QHBoxLayout, QLineEdit, QPushButton, QLabel, QApplication
};

use qt_gui::{
    QCursor
};

use std::rc::Rc;

pub(crate) struct Header {
    app: Ptr<QApplication>,
    widget: QBox<QWidget>,
    redo_button: QBox<QPushButton>,
    undo_button: QBox<QPushButton>,
    search_edit: QBox<QLineEdit>,

    // minimum_button: QBox<QPushButton>,
    // maximum_button: QBox<QPushButton>,
    close_button: QBox<QPushButton>,
}

impl StaticUpcast<QObject> for Header {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Header {
    pub fn new(app: Ptr<QApplication>) -> Rc<Header> {
        unsafe {
            let content_style = QString::from_std_str(
                "QWidget {\
                    background-color: #ec4141;\
                }"
            );
            let button_style = QString::from_std_str(
                "QPushButton {\
                    border: none;\
                    min-width: 24px;\
                    max-width: 24px;\
                    min-height: 24px;\
                    background-color: #d83b3b;\
                    border-radius: 12px;\
                }"
            );
            let close_button_style = QString::from_std_str(
                "QPushButton {\
                    border:none;\
                    min-width: 24px;\
                    max-width: 24px;\
                    background-color: #d83b3b;\
                }"
            );
            let edit_style = QString::from_std_str(
                "QLineEdit {\
                    border: none;\
                    min-width: 146px;\
                    max-width: 146px;\
                    min-height: 33px;\
                    background-color: #e13e3e;\
                    border-radius: 15px;\
                    padding-left: 16px;\
                }\
                QLineEdit::indicator {\
                    width: 7px;
                    height: 7px;
                }"
            );

            let widget = QWidget::new_0a();
            widget.set_style_sheet(&content_style);

            let layout = QHBoxLayout::new_1a(&widget);
            layout.set_spacing(0);
            layout.set_contents_margins_4a(0, 13, 0, 12);
            layout.add_spacing(17);

            let title_icon = QLabel::new();
            title_icon.set_style_sheet(&qs(
                "QLabel {\
                    min-width: 128px;\
                    max-width: 128px;\
                    min-height: 30px;\
                    max-height: 30px;\
                    border-image: url(:/image/netease_title_icon.png);\
                }"
            ));
            layout.add_widget(&title_icon);

            layout.add_spacing(83);

            let undo_button = QPushButton::new();
            undo_button.set_cursor(&QCursor::from_cursor_shape(CursorShape::PointingHandCursor));
            undo_button.set_style_sheet(&button_style);
            layout.add_widget(&undo_button);

            layout.add_spacing(8);

            let redo_button = QPushButton::new();
            redo_button.set_cursor(&QCursor::from_cursor_shape(CursorShape::PointingHandCursor));
            redo_button.set_style_sheet(&button_style);
            layout.add_widget(&redo_button);

            layout.add_spacing(12);

            let search_edit = QLineEdit::new();
            search_edit.set_style_sheet(&edit_style);
            layout.add_widget(&search_edit);

            layout.add_spacing(447);

            let minimum_button = QPushButton::new();
            minimum_button.set_style_sheet(&close_button_style);
            layout.add_widget(&minimum_button);

            layout.add_spacing(12);

            let maximum_button = QPushButton::new();
            maximum_button.set_style_sheet(&close_button_style);
            layout.add_widget(&maximum_button);

            layout.add_spacing(12);

            let close_button = QPushButton::new();
            close_button.set_style_sheet(&close_button_style);
            layout.add_widget(&close_button);
    
            layout.add_spacing(20);

            let this = Rc::new(Self {
                app,
                widget,
                redo_button,
                undo_button,
                search_edit,
                close_button,
            });
            this.init();
            this
        }
    }

    pub unsafe fn widget(self: &Rc<Self>) -> Ptr<QWidget> {
        self.widget.as_ptr()
    }

    unsafe fn init(self: &Rc<Self>) {
        self.redo_button.clicked().connect(&self.slot_on_redo_button_clicked());
        self.undo_button.clicked().connect(&self.slot_on_undo_button_clicked());
        self.search_edit.editing_finished().connect(&self.slot_on_editing_finished());
        self.close_button.clicked().connect(self.app.slot_quit());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_redo_button_clicked(self: &Rc<Self>) {

    }

    #[slot(SlotNoArgs)]
    unsafe fn on_undo_button_clicked(self: &Rc<Self>) {

    }

    #[slot(SlotNoArgs)]
    unsafe fn on_editing_finished(self: &Rc<Self>) {

    }
}
