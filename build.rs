extern crate pkg_config;

fn main() {
    let pkg = pkg_config::Config::new();

    if pkg.probe("hidapi-hidraw").is_err() {
        pkg.probe("hidapi-libusb")
            .expect("Either hidraw or libusb backends should be present");
    };
}
