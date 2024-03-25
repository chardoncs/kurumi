use std::{cell::RefCell, sync::OnceLock};

use gtk::{glib::{self, prelude::*, subclass::Signal, Properties}, subclass::prelude::*, CompositeTemplate, DrawingArea};

use super::PageData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::PdfPageObject)]
pub struct PdfPageObject {
    #[property(name = "page", get, set, type = i32, member = page)]
    #[property(name = "scale", get, set, type = f64, member = scale)]
    pub data: RefCell<PageData>,
}

#[glib::object_subclass]
impl ObjectSubclass for PdfPageObject {
    const NAME: &'static str = "PdfPageObject";
    type Type = super::PdfPageObject;
}

#[glib::derived_properties]
impl ObjectImpl for PdfPageObject {}

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/chardon55/kurumi/pdf-page.ui")]
pub struct PdfPage {
    #[template_child]
    pub drawing_area: TemplateChild<DrawingArea>,
}

#[glib::object_subclass]
impl ObjectSubclass for PdfPage {
    const NAME: &'static str = "PdfPage";
    type Type = super::PdfPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PdfPage {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("scroll-offset")
                    .param_types([f64::static_type(), f64::static_type()])
                    .build(),
            ]
        })
    }
}

impl WidgetImpl for PdfPage {}

impl BoxImpl for PdfPage {}
