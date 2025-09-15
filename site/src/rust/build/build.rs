
fn main() {
    println!("cargo::rerun-if-changed=src/css/main");
    println!("cargo::rerun-if-changed=src/js/loader");
    println!("cargo::rerun-if-changed=src/resources");
}