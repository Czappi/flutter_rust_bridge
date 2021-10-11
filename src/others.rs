use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use log::{error, info, warn};
use pathdiff::diff_paths;

// NOTE [DartPostCObjectFnType] was originally [*mut DartCObject] but I changed it to [*mut c_void]
// because cannot automatically generate things related to [DartCObject]. Anyway this works fine.
pub const DUMMY_WIRE_CODE_FOR_BINDGEN: &str = r#"
    // ----------- DUMMY CODE FOR BINDGEN ----------
    
    // copied from: allo-isolate
    pub type DartPort = i64;
    pub type DartPostCObjectFnType = unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool;
    #[no_mangle] pub unsafe extern "C" fn store_dart_post_cobject(ptr: DartPostCObjectFnType) { panic!("dummy code") }
    
    // https://github.com/fzyzcjy/flutter_rust_bridge/issues/50
    #[no_mangle] pub unsafe extern "C" fn rust_dummy_method_to_enforce_bundling() { /* nothing */ }
    
    // ---------------------------------------------
    "#;

pub fn modify_dart_wire_content(content_raw: &str, dart_wire_class_name: &str) -> String {
    content_raw.replace(
        &format!("class {} {{", dart_wire_class_name),
        &format!(
            "class {} implements FlutterRustBridgeWireBase {{",
            dart_wire_class_name
        ),
    )
}

pub fn extract_dart_wire_content(content: &str) -> (String, String) {
    let (mut imports, mut body) = (Vec::new(), Vec::new());
    for line in content.split('\n') {
        (if line.starts_with("import ") {
            &mut imports
        } else {
            &mut body
        })
        .push(line);
    }
    (imports.join("\n"), body.join("\n"))
}

pub fn sanity_check(generated_dart_wire_code: &str, dart_wire_class_name: &str) {
    if !generated_dart_wire_code.contains(dart_wire_class_name) {
        error!(
            "Nothing is generated for dart wire class. \
            Maybe you forget to put code like `mod the_generated_bridge_code;` to your `lib.rs`?"
        );
    }
}

pub fn try_add_mod_to_lib(rust_crate_dir: &str, rust_output_path: &str) {
    if let Err(e) = auto_add_mod_to_lib_core(rust_crate_dir, rust_output_path) {
        warn!(
            "auto_add_mod_to_lib fail, the generated code may or may not have problems. \
            Please ensure you have add code like `mod the_generated_bridge_code;` to your `lib.rs`. \
            Details: {}",
            e
        );
    }
}

pub fn auto_add_mod_to_lib_core(rust_crate_dir: &str, rust_output_path: &str) -> Result<()> {
    let path_src_folder = Path::new(rust_crate_dir).join("src");
    let rust_output_path_relative_to_src_folder =
        diff_paths(rust_output_path, path_src_folder.clone()).ok_or_else(|| {
            anyhow!(
                "rust_output_path={} is unrelated to path_src_folder={:?}",
                rust_output_path,
                &path_src_folder,
            )
        })?;

    let mod_name = rust_output_path_relative_to_src_folder
        .file_stem()
        .ok_or_else(|| anyhow!(""))?
        .to_str()
        .ok_or_else(|| anyhow!(""))?
        .to_string()
        .replace("/", "::");
    let expect_code = format!("mod {};", mod_name);

    let path_lib_rs = path_src_folder.join("lib.rs");

    let raw_content_lib_rs = fs::read_to_string(path_lib_rs.clone())?;
    if !raw_content_lib_rs.contains(&expect_code) {
        info!("Inject `{}` into {:?}", &expect_code, &path_lib_rs);

        let comments = " /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */";
        let modified_content_lib_rs =
            format!("{}{}\n{}", expect_code, comments, raw_content_lib_rs);

        fs::write(&path_lib_rs, modified_content_lib_rs).unwrap();
    }

    Ok(())
}
