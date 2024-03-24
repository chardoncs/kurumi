use std::borrow::Borrow;

use gtk::{glib::subclass::types::ObjectSubclassIsExt, prelude::*, EventControllerKey};

use super::window::KurumiMainWindow;

pub trait BindKeys {
    fn bind_keys(&self);
}

impl BindKeys for KurumiMainWindow {
    fn bind_keys(&self) {
        let key_binding = EventControllerKey::builder()
            .name("key-binding")
            .build();
        
        let _container = self.imp().page_container.borrow().get();

        key_binding.connect_key_pressed(move |_, key, _, _modifier| {

            match key {
                // TODO
                _ => {}
            }

            gtk::glib::Propagation::Proceed
        });

        self.add_controller(key_binding);
    }
}
