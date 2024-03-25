use std::cell::RefCell;

use gtk::{prelude::*, glib::{self, Properties}, subclass::prelude::*};

use super::StatusData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::StatusObject)]
pub struct StatusObject {
    #[property(name = "scale", get, set, type = f64, member = scale)]
    #[property(name = "cur_page", get, set, type = i32, member = cur_page)]
    pub data: RefCell<StatusData>,
}

#[glib::object_subclass]
impl ObjectSubclass for StatusObject {
    const NAME: &'static str = "StatusObject";
    type Type = super::StatusObject;
}

#[glib::derived_properties]
impl ObjectImpl for StatusObject {}
