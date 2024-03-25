mod imp;

use gtk::{gio, glib::{self, object::Cast, property::PropertySet, subclass::types::ObjectSubclassIsExt, Object}, prelude::*, Application, EventControllerScroll, EventControllerScrollFlags, ListItem, NoSelection, ScrolledWindow, SignalListItemFactory};

use crate::{mismatching_error, util::{patch_title, pos_percentage}};

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

    pub fn doc(&self) -> Option<poppler::Document> {
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
                .expect(mismatching_error!("gtk::ListItem"))
                .set_child(Some(&page));
        });

        let scale = self.imp().scale.borrow().value();

        factory.connect_bind(move |_, item| {
            let obj = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .item()
                .and_downcast::<PdfPageObject>()
                .expect(mismatching_error!("kurumi page model"));

            let page = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .child()
                .and_downcast::<PdfPage>()
                .expect(mismatching_error!("kurumi page"));

            page.bind(&obj, &doc.clone().unwrap(), scale);
        });

        factory.connect_unbind(move |_, item| {
            let page = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .child()
                .and_downcast::<PdfPage>()
                .expect(mismatching_error!("kurumi page"));

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

    fn load_scroll_event(&self) {
        let container = self.imp().page_container.get();

        let pos = self.imp().pos_percentage.get();

        let controller = EventControllerScroll::builder()
            .flags(EventControllerScrollFlags::VERTICAL)
            .build();

        let adj = container.vadjustment();

        let pos1 = pos.clone();

        controller.connect_scroll(move |_, _, dy| {
            if dy <= 0.0 {
                return gtk::glib::Propagation::Proceed;
            }

            if let Some(adj) = &adj {
                pos1.set_label(pos_percentage(adj.value(), adj.upper() - adj.lower()).as_str());
            }

            gtk::glib::Propagation::Proceed
        });

        container.add_controller(controller);

        container.parent()
            .and_downcast_ref::<ScrolledWindow>()
            .expect(mismatching_error!("gtk::ScrolledWindow"))
            .connect_edge_reached(move |_, pos_type| {
                match pos_type {
                    gtk::PositionType::Top => pos.set_label("Top"),
                    gtk::PositionType::Bottom => pos.set_label("Bot"),
                    _ => {}
                }
            });
    }

    pub fn init(&self) {
        self.setup_factory();
        self.load_document();
        self.bind_keys();
        self.load_scroll_event();
    }
}
