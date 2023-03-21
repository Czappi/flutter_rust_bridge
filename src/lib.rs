//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge
#![allow(clippy::vec_init_then_push)]
// #![warn(clippy::wildcard_enum_match_arm)]

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use itertools::Itertools;
mod logs;
pub use crate::logs::init_logger;
use log::info;
use pathdiff::diff_paths;

use crate::commands::BindgenRustToDartArg;
use crate::others::*;
use crate::target::Acc;
use crate::target::Target;

mod config;

pub use crate::commands::ensure_tools_available;
pub use crate::config::parse as config_parse;
pub use crate::config::Opts;
pub use crate::config::RawOpts;
pub use crate::utils::misc::get_symbols_if_no_duplicates;

#[macro_use]
mod commands;
mod error;
mod generator;
mod ir;
mod markers;
mod method_utils;
mod others;
mod parser;
mod source_graph;
mod target;
mod transformer;
pub mod utils;
use error::*;
use crate::utils::misc::{BlockIndex, with_changed_file};

pub mod dump;

/// When the API is only defined in 1 rust file(block), take this one for generation, where `config`
/// is the instance containing all information to the API file(block), and `all_symbols` contains
/// all unique APIs defined in the file mentioned above.
/// If APIs are defined in more than 1 rust file(block), use `frb_codegen_multi` instead.
pub fn frb_codegen(config: &config::Opts, all_symbols: &[String]) -> anyhow::Result<()> {
    frb_codegen_multi(&[config.clone()], 0, all_symbols)
}

/// This function is used only for cases with multi-blocks when there are
/// more than one API block.
/// In addition, because the current block to deal with needs information
/// from all other blocks, `all_configs` is used here,
/// with `index` referring to the place of the current block to deal with.
/// For details on how to take advantage of multi-blocks, please refers to this article: https://cjycode.com/flutter_rust_bridge/feature/multiple_files.html
pub fn frb_codegen_multi(
    all_configs: &[config::Opts],
    index: usize,
    all_symbols: &[String],
) -> anyhow::Result<()> {
    info!("Phase: Validate config(s)");
    for (i, config) in all_configs.iter().enumerate() {
        assert_eq!(
            BlockIndex(i),
            config.block_index,
            "order index({}) != block_index({})",
            i,
            config.block_index
        );
    }

    let config = &all_configs[index];
    info!("Picked config: {:?}", config);

    info!("Phase: Parse source code to AST, then to IR");
    let raw_ir_file = config.get_ir_file()?;

    info!("Phase: Transform IR");
    let ir_file = transformer::transform(raw_ir_file);

    info!("Phase: Generate Rust code");
    let generated_rust = generate_rust_code(config, &ir_file)?;

    info!("Phase: Generate Dart code");
    generate_dart_code(config, all_configs, &ir_file, generated_rust, all_symbols)?;

    info!("Success!");
    Ok(())
}

fn generate_rust_code(
    config: &Opts,
    ir_file: &ir::IrFile,
) -> anyhow::Result<generator::rust::Output> {
    let rust_output_paths = config.get_rust_output_paths();

    let rust_output_dir = Path::new(&rust_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(rust_output_dir)?;

    let generated_rust = ir_file.generate_rust(config);
    write_rust_modules(config, &generated_rust)?;

    if !config.skip_add_mod_to_lib {
        others::try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }

    command_run!(
        commands::format_rust,
        &config.rust_output_path,
        (
            config.wasm_enabled && !config.inline_rust,
            config.rust_io_output_path(),
            config.rust_wasm_output_path(),
        )
    )?;

    Ok(generated_rust)
}

fn generate_dart_code(
    config: &Opts,
    all_configs: &[Opts],
    ir_file: &ir::IrFile,
    generated_rust: generator::rust::Output,
    all_symbols: &[String],
) -> anyhow::Result<()> {
    let dart_root = config.dart_root_or_default();
    ensure_tools_available(&dart_root, config.skip_deps_check)?;

    info!("Phase: Generating Dart bindings for Rust");
    // phase-step1: generate temporary c file
    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    let exclude_symbols = generated_rust.get_exclude_symbols(all_symbols);
    with_changed_file(
        &config.rust_output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                BindgenRustToDartArg {
                    rust_crate_dir: &config.rust_crate_dir,
                    c_output_path: temp_bindgen_c_output_file
                        .path()
                        .as_os_str()
                        .to_str()
                        .unwrap(),
                    dart_output_path: temp_dart_wire_file.path().as_os_str().to_str().unwrap(),
                    dart_class_name: &config.dart_wire_class_name(),
                    c_struct_names: ir_file.get_c_struct_names(),
                    exclude_symbols,
                    llvm_install_path: &config.llvm_path[..],
                    llvm_compiler_opts: &config.llvm_compiler_opts,
                },
                &dart_root,
            )
        },
    )?;

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();

    for (i, each_path) in config.c_output_path.iter().enumerate() {
        let c_dummy_code =
            generator::c::generate_dummy(config, all_configs, &effective_func_names, i);
        println!("the path is {each_path:?}");
        fs::create_dir_all(Path::new(each_path).parent().unwrap())?;
        fs::write(
            each_path,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    // phase-step2: generate raw dart code instance from the c file
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        &config.dart_wire_class_name(),
    ));
    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    // phase-step3: compose dart codes and write to file
    let generated_dart = ir_file.generate_dart(config, &generated_rust.wasm_exports);
    let generated_dart_decl_all = &generated_dart.decl_code;
    let generated_dart_impl_io_wire = &generated_dart.impl_code.io + &generated_dart_wire;

    let dart_output_paths = config.get_dart_output_paths();
    let dart_output_dir = Path::new(&dart_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(dart_output_dir)?;

    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        write_dart_decls(
            config,
            dart_decl_output_path,
            dart_output_dir,
            &generated_dart,
            generated_dart_decl_all,
            &generated_dart_impl_io_wire,
        )?;
    } else if config.wasm_enabled {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + generated_dart_decl_all
                + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            &dart_output_paths.io_path,
            (&generated_dart.file_prelude + &generated_dart_impl_io_wire).to_text(),
        )?;
        fs::write(
            &dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &generated_dart.impl_code.wasm).to_text(),
        )?;
    } else {
        let mut out = generated_dart.file_prelude
            + generated_dart_decl_all
            + &generated_dart.impl_code.common
            + &generated_dart_impl_io_wire;
        out.import = out.import.lines().unique().join("\n");
        fs::write(&dart_output_paths.base_path, out.to_text())?;
    }

    info!("Phase: Running build_runner");
    let dart_root = &config.dart_root;
    if generated_dart.needs_freezed && config.build_runner {
        let dart_root = dart_root.as_ref().ok_or_else(|| {
            Error::string(
                "build_runner configured to run, but Dart root could not be inferred.
        Please specify --dart-root, or disable build_runner with --no-build-runner.",
            )
        })?;
        commands::build_runner(dart_root)?;
    }

    info!("Phase: Formatting Dart code");
    command_run!(
        commands::format_dart[config.dart_format_line_length],
        &dart_output_paths.base_path,
        ?config.dart_decl_output_path,
        (
            config.wasm_enabled,
            dart_output_paths.wasm_path,
            dart_output_paths.io_path,
        ),
        (
            generated_dart.needs_freezed && config.build_runner,
            config.dart_freezed_path(),
        )
    )?;

    Ok(())
}

fn write_dart_decls(
    config: &Opts,
    dart_decl_output_path: &str,
    dart_output_dir: &Path,
    generated_dart: &crate::generator::dart::Output,
    generated_dart_decl_all: &DartBasicCode,
    generated_dart_impl_io_wire: &DartBasicCode,
) -> anyhow::Result<()> {
    let impl_import_decl = DartBasicCode {
        import: format!(
            "import \"{}\";",
            diff_paths(dart_decl_output_path, dart_output_dir)
                .unwrap()
                .to_str()
                .unwrap()
        ),
        ..Default::default()
    };

    let common_import = DartBasicCode {
        import: if config.wasm_enabled {
            format!(
                "import '{}' if (dart.library.html) '{}';",
                config
                    .dart_io_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
                config
                    .dart_wasm_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
            )
        } else {
            "".into()
        },
        ..Default::default()
    };

    fs::write(
        dart_decl_output_path,
        (&generated_dart.file_prelude + &common_import + generated_dart_decl_all).to_text(),
    )?;

    let dart_output_paths = config.get_dart_output_paths();
    if config.wasm_enabled {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.io_path,
            (&generated_dart.file_prelude + &impl_import_decl + generated_dart_impl_io_wire)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.wasm)
                .to_text(),
        )?;
    } else {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + &impl_import_decl
                + &generated_dart.impl_code.common
                + generated_dart_impl_io_wire)
                .to_text(),
        )?;
    }
    Ok(())
}

fn write_rust_modules(
    config: &Opts,
    generated_rust: &crate::generator::rust::Output,
) -> anyhow::Result<()> {
    let Acc { common, io, wasm } = &generated_rust.code;
    fn emit_platform_module(name: &str, body: &str, config: &Opts, target: Target) -> String {
        if config.inline_rust {
            format!("mod {name} {{ use super::*;\n {body} }}")
        } else {
            let path = match target {
                Target::Io => config.rust_io_output_path(),
                Target::Wasm => config.rust_wasm_output_path(),
                _ => panic!("unsupported target: {:?}", target),
            };
            let path = path.file_name().and_then(OsStr::to_str).unwrap();
            format!("#[path = \"{path}\"] mod {name};")
        }
    }
    let common = format!(
        "{}
{mod_web}

#[cfg(not(target_family = \"wasm\"))]
{mod_io}
#[cfg(not(target_family = \"wasm\"))]
pub use io::*;
",
        common,
        mod_web = if config.wasm_enabled {
            format!(
                "
/// cbindgen:ignore
#[cfg(target_family = \"wasm\")]
{}
#[cfg(target_family = \"wasm\")]
pub use web::*;",
                emit_platform_module("web", wasm, config, Target::Wasm)
            )
        } else {
            "".into()
        },
        mod_io = emit_platform_module("io", io, config, Target::Io),
    );
    fs::write(&config.rust_output_path, common)?;
    if !config.inline_rust {
        fs::write(config.rust_io_output_path(), format!("use super::*;\n{io}"))?;
        if config.wasm_enabled {
            fs::write(
                config.rust_wasm_output_path(),
                format!("use super::*;\n{wasm}"),
            )?;
        }
    }
    Ok(())
}
