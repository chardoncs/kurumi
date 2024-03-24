use std::cell::RefCell;

use gtk::{gio::glib, glib::subclass::{object::{ObjectImpl, ObjectImplExt}, types::{ObjectSubclass, ObjectSubclassExt}}, subclass::{application_window::ApplicationWindowImpl, widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl}, window::WindowImpl}, ApplicationWindow, CompositeTemplate, Label, ListView, TemplateChild};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardon55/kurumi/window.ui")]
pub struct KurumiMainWindow {
    #[template_child]
    pub page_container: TemplateChild<ListView>,
    #[template_child]
    pub status_line: TemplateChild<gtk::Box>,
    #[template_child]
    pub escape_cmd: TemplateChild<Label>,

    pub pages: RefCell<Option<gtk::gio::ListStore>>,

    pub doc: RefCell<Option<poppler::Document>>,
}

#[glib::object_subclass]
impl ObjectSubclass for KurumiMainWindow {
    const NAME: &'static str = "KurumiMainWindow";
    type Type = super::KurumiMainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for KurumiMainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_page_model();
    }
}

impl WidgetImpl for KurumiMainWindow {}

impl WindowImpl for KurumiMainWindow {}

impl ApplicationWindowImpl for KurumiMainWindow {}
