use std::{cell::{Cell, RefCell}, sync::OnceLock};

use gtk::{glib::{self, prelude::*, subclass::{object::{ObjectImpl, ObjectImplExt}, types::{ObjectSubclass, ObjectSubclassExt}, Signal}}, subclass::{application_window::ApplicationWindowImpl, widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl}, window::WindowImpl}, ApplicationWindow, CompositeTemplate, Label, ListView, TemplateChild};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardon55/kurumi/window.ui")]
pub struct KurumiMainWindow {
    #[template_child]
    pub page_container: TemplateChild<ListView>,

    #[template_child]
    pub status_line: TemplateChild<gtk::Box>,

    #[template_child]
    pub escape_cmd: TemplateChild<Label>,

    #[template_child]
    pub pos_percentage: TemplateChild<Label>,

    #[template_child]
    pub page_info: TemplateChild<Label>,

    #[template_child]
    pub scale_percentage: TemplateChild<Label>,

    pub pages: RefCell<Option<gtk::gio::ListStore>>,

    pub doc: RefCell<Option<poppler::Document>>,

    pub scale: Cell<f64>,

    pub control_stack: Cell<u32>,

    pub cur_page: Cell<i32>,
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
        obj.set_scale(1.0);
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("control-escape-push")
                    .build(),
                Signal::builder("control-escape-pop")
                    .build(),
                Signal::builder("zoom")
                    .param_types([f64::static_type()])
                    .build(),
            ]
        })
    }
}

impl WidgetImpl for KurumiMainWindow {}

impl WindowImpl for KurumiMainWindow {}

impl ApplicationWindowImpl for KurumiMainWindow {}
