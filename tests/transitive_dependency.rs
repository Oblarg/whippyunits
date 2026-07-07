//! Verifies that macros exported from `whippyunits` are self-contained, i.e. a
//! downstream crate that depends only on `whippyunits` (not `whippyunits-core`,
//! `whippyunits-proc-macros`, or `num-traits`) should still be able to compile
//! code that uses public macros from `whippyunits`.

use std::path::Path;
use std::process::Command;

#[test]
fn exported_macros_do_not_leak_internal_dependencies() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_manifest = manifest_dir.join("tests/transitive_dep_fixture/Cargo.toml");
    let target_dir = manifest_dir.join("target/transitive-dep-fixture");

    let output = Command::new(env!("CARGO"))
        .args(["build", "--manifest-path"])
        .arg(&fixture_manifest)
        .arg("--target-dir")
        .arg(&target_dir)
        .output()
        .expect("failed to invoke `cargo build` on the fixture crate");

    assert!(
        output.status.success(),
        "a downstream crate depending only on `whippyunits` (not on its internal \
         dependencies whippyunits-core/whippyunits-proc-macros/num-traits) failed to \
         compile against whippyunits's public macros:\n\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
