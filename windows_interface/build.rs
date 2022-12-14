fn main() {
	const RES: &str = "resources/resources.res";
	println!("cargo:rustc-link-lib=dylib:+verbatim={}", RES);
}