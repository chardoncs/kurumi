use gtk::{glib::{self, Object}, prelude::*, subclass::prelude::*};

mod imp;

glib::wrapper! {
    pub struct PdfPageObject(ObjectSubclass<imp::PdfPageObject>);
}

impl PdfPageObject {
    pub fn new(page: i32) -> Self {
        Object::builder()
            .property("page", page)
            .build()
    }
}

#[derive(Default)]
pub struct PageData {
    /// 0-based page index
    ///
    /// ## Example
    ///
    /// $0$ is the first page;
    /// while $n - 1$ is the last page.
    pub page: i32,
}

glib::wrapper! {
    pub struct PdfPage(ObjectSubclass<imp::PdfPage>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for PdfPage {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, obj: &PdfPageObject, doc: &poppler::Document) {
        let da = self.imp().drawing_area.get();

        if let Some(page) = doc.page(obj.page()) {
            let (width, height) = page.size();

            da.set_size_request(width.ceil() as i32, height.ceil() as i32);

            da.set_draw_func(move |_, ctx, _, _| {
                // Draw background
                ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                ctx.rectangle(0.0, 0.0, width, height);
                ctx.fill().expect("Background filling failed.");

                // Render PDF
                page.render(ctx);
            });
        }
    }

    pub fn unbind(&self) {
        let da = self.imp().drawing_area.get();

        da.unset_draw_func();
    }

    pub fn refresh(&self) {
        let da = self.imp().drawing_area.get();

        if da.is_drawable() {
            da.queue_draw();
        }
    }
}

