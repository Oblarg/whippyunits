fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_generic_const_exprs)");
    if cfg!(feature = "cge") {
        println!("cargo::rustc-cfg=has_generic_const_exprs");
    }
}
