#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=wait_for_debugger");
}

#[rustversion::not(nightly)]
fn main() {}
