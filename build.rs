fn main() {
    //Link the static library libdifferential_evolution.a stored in src/c/builddir
    println!("cargo:rustc-link-search=native=./src/c/builddir");
    //println!("cargo:rustc-link-lib=static=differential_evolution");
    //println!("cargo:rerun-if-changed=src/c/builddir/libdifferential_evolution.a");
}