use gtk::{gdk::Key, glib::{self, closure_local, prelude::*, subclass::types::ObjectSubclassIsExt}, prelude::*, EventControllerKey};

use super::window::KurumiMainWindow;

pub trait BindKeys {
    fn bind_keys(&self);
}

impl BindKeys for KurumiMainWindow {
    fn bind_keys(&self) {
        let key_binding = EventControllerKey::builder()
            .name("key-binding")
            .build();

        self.connect_closure("control-escape-push", false, closure_local!(|win: KurumiMainWindow| {
            let con = &win.imp().control_stack;
            con.set(con.get() + 1);
        }));

        self.connect_closure("control-escape-pop", false, closure_local!(|win: KurumiMainWindow| {
            let con = &win.imp().control_stack;
            let cur_con = con.get();

            if cur_con > 0 {
                con.set(cur_con - 1);
            }
        }));

        let win = self.clone();
        
        key_binding.connect_key_pressed(move |_, key, _, _| {

            match key {
                Key::Control_L | Key::Control_R => {
                    win.emit_by_name::<()>("control-escape-push", &[]);
                }
                _ => {}
            }

            gtk::glib::Propagation::Proceed
        });

        let win = self.clone();

        key_binding.connect_key_released(move |_, key, _, _| {

            match key {
                Key::Control_L | Key::Control_R => {
                    win.emit_by_name::<()>("control-escape-pop", &[]);
                }
                _ => {}
            }
        });

        self.add_controller(key_binding);
    }
}
