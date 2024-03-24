mod imp;

use gtk::{gio, glib::{self, object::Cast, property::PropertySet, subclass::types::ObjectSubclassIsExt, Object}, prelude::*, Application, ListItem, NoSelection, SignalListItemFactory};

use crate::{error::gtk_mismatching_error, util::patch_title};

use super::{key_binding::BindKeys, pdfpage::{PdfPage, PdfPageObject}};

glib::wrapper! {
    pub struct KurumiMainWindow(ObjectSubclass<imp::KurumiMainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

impl KurumiMainWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app)
            .property("title", patch_title(None))
            .build()
    }

    fn pages(&self) -> gio::ListStore {
        self.imp()
            .pages
            .borrow()
            .clone()
            .unwrap()
    }

    fn doc(&self) -> Option<poppler::Document> {
        self.imp()
            .doc
            .borrow()
            .clone()
    }

    pub fn set_doc(&self, doc: Option<poppler::Document>) {
        self.imp()
            .doc
            .set(doc);
    }

    fn setup_page_model(&self) {
        let model = gio::ListStore::new::<PdfPageObject>();

        let imp = self.imp();

        imp.pages.replace(Some(model));
        imp.page_container.set_model(Some(&NoSelection::new(Some(self.pages()))));
    }

    fn setup_factory(&self) {
        let doc = self.doc();

        if doc.is_none() {
            return;
        }

        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, item| {
            let page = PdfPage::new();

            item.downcast_ref::<gtk::ListItem>()
                .expect(gtk_mismatching_error("gtk::ListItem").as_str())
                .set_child(Some(&page));
        });

        factory.connect_bind(move |_, item| {
            let obj = item
                .downcast_ref::<ListItem>()
                .expect(gtk_mismatching_error("gtk::ListItem").as_str())
                .item()
                .and_downcast::<PdfPageObject>()
                .expect(gtk_mismatching_error("kurumi page model").as_str());

            let page = item
                .downcast_ref::<ListItem>()
                .expect(gtk_mismatching_error("gtk::ListItem").as_str())
                .child()
                .and_downcast::<PdfPage>()
                .expect(gtk_mismatching_error("kurumi page").as_str());

            page.bind(&obj, &doc.clone().unwrap());
        });

        factory.connect_unbind(move |_, item| {
            let page = item
                .downcast_ref::<ListItem>()
                .expect(gtk_mismatching_error("gtk::ListItem").as_str())
                .child()
                .and_downcast::<PdfPage>()
                .expect(gtk_mismatching_error("kurumi page").as_str());

            page.unbind();
        });
        
        self.imp().page_container.set_factory(Some(&factory));
    }

    fn load_document(&self) {
        if let Some(doc) = self.doc() {

            let total_pages = doc.n_pages();

            // Load window models using a dumb but working way ;)
            for i in 0..total_pages {
                self.pages().append(&PdfPageObject::new(i));
            }
        }
    }

    pub fn init(&self) {
        self.setup_factory();
        self.load_document();
        self.bind_keys();
    }
}
