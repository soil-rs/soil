#[rustversion::before(1.68)]
fn main() {
	if !cfg!(feature = "std") {
		println!("cargo:rustc-cfg=enable_alloc_error_handler");
	}
}

#[rustversion::since(1.68)]
fn main() {}
