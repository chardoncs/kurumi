use gtk::{gio::glib, {subclass::prelude::*, ApplicationWindow, CompositeTemplate, Label, ListView}};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardon55/kurumi/window.ui")]
pub struct KurumiMainWindowImpl {
    #[template_child]
    pub page_container: TemplateChild<ListView>,
    #[template_child]
    pub status_line: TemplateChild<gtk::Box>,
    #[template_child]
    pub escape_cmd: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for KurumiMainWindowImpl {
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

impl ObjectImpl for KurumiMainWindowImpl {
    fn constructed(&self) {
        self.parent_constructed();

        //let obj = self.obj();
    }
}

impl WidgetImpl for KurumiMainWindowImpl {}

impl WindowImpl for KurumiMainWindowImpl {}

impl ApplicationWindowImpl for KurumiMainWindowImpl {}
