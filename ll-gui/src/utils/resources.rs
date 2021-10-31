use super::consts::RESOURCES_BYTES;

// Load & regiser resources
pub fn load_resources() {

    let glib_resource_bytes = glib::Bytes::from_static(RESOURCES_BYTES);
    let resources = gio::Resource::from_data(&glib_resource_bytes).expect("Failed to load resources");
    gio::resources_register(&resources);

}
