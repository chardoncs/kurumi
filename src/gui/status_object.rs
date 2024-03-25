mod imp;

gtk::glib::wrapper! {
    pub struct StatusObject(ObjectSubclass<imp::StatusObject>);
}

impl StatusObject {
    pub fn new() -> Self {
        gtk::glib::Object::builder()
            .property("scale", 1.0f64)
            .property("cur_page", 0)
            .build()
    }
}

#[derive(Default)]
pub struct StatusData {
    pub scale: f64,

    pub cur_page: i32,
}
