extern crate pkg_config;

fn main() {
    pkg_config::probe_library("hidapi-hidraw").expect("Could not find hidapi-hidraw");
}
