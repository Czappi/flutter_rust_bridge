use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use clap::IntoApp;
use clap::Parser;
use convert_case::{Case, Casing};
use serde::Deserialize;
use toml::Value;

use crate::ir::IrFile;
use crate::utils::{BlockIndex, ExtraTraitForVec, PathExt};
use crate::{get_symbols_if_no_duplicates, parser, transformer};

#[derive(Parser, Debug, PartialEq, Eq, Deserialize, Default)]
#[clap(
    bin_name = "flutter_rust_bridge_codegen",
    version,
    setting(clap::AppSettings::DeriveDisplayOrder)
)]
pub struct RawOpts {
    /// Path of input Rust code
    #[clap(short, long, required = true, multiple_values = true)]
    pub rust_input: Vec<String>,
    /// Path of output generated Dart code
    #[clap(short, long, required = true, multiple_values = true)]
    pub dart_output: Vec<String>,
    /// If provided, generated Dart declaration code to this separate file
    #[clap(long)]
    pub dart_decl_output: Option<String>,
    /// Output path (including file name) of generated C header, each field corresponding to that of `rust-input`
    #[clap(short, long)]
    pub c_output: Option<Vec<String>>,
    /// Extra output path (excluding file name) of generated C header
    #[clap(short, long)]
    pub extra_c_output_path: Option<Vec<String>>,
    /// Crate directory for your Rust project
    #[clap(long, multiple_values = true)]
    pub rust_crate_dir: Option<Vec<String>>,
    /// Output path of generated Rust code
    #[clap(long, multiple_values = true)]
    pub rust_output: Option<Vec<String>>,
    /// Generated class name
    #[clap(long, multiple_values = true)]
    pub class_name: Option<Vec<String>>,
    /// Line length for Dart formatting
    #[clap(long, default_value = "80")]
    pub dart_format_line_length: u32,
    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[clap(long)]
    pub skip_add_mod_to_lib: bool,
    /// Path to the installed LLVM
    #[clap(long, multiple_values = true)]
    pub llvm_path: Option<Vec<String>>,
    /// LLVM compiler opts
    #[clap(long)]
    pub llvm_compiler_opts: Option<String>,
    /// Path to root of Dart project, otherwise inferred from --dart-output
    #[clap(long, multiple_values = true)]
    pub dart_root: Option<Vec<String>>,
    /// Skip running build_runner even when codegen-required code is detected
    #[clap(long)]
    pub no_build_runner: bool,
    /// Show debug messages.
    #[clap(short, long)]
    pub verbose: bool,
    /// Enable WASM module generation.
    /// Requires: --dart-decl-output
    #[clap(long)]
    pub wasm: bool,
    /// Inline declaration of Rust bridge modules
    #[clap(long)]
    pub inline_rust: bool,
    /// Skip dependencies check.
    #[clap(long)]
    pub skip_deps_check: bool,

    // Output path of auto-generated rust file, which is used to store shared stuff among regular API blocks,
    // it could be something like `src/my_shared.rs`, `"my_shared.rs"`...
    // if it is none, the default path would be `./bridge_generated_shares.rs`
    #[clap(short, long)]
    pub shared_rust_output: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_path: Vec<String>,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: u32,
    pub skip_add_mod_to_lib: bool,
    pub llvm_path: Vec<String>,
    pub llvm_compiler_opts: String,
    pub manifest_path: String,
    pub dart_root: Option<String>,
    pub build_runner: bool,
    pub block_index: BlockIndex,
    pub skip_deps_check: bool,
    pub wasm_enabled: bool,
    pub inline_rust: bool,
    pub shared: bool, // it is true if this Opts instance is for auto-generated shared API block. Otherwise, it is false,
    // for the below 2 fields:
    // whatever the Opts is for regular or auto-generated shared API block,
    // and whatever there is or no shared stuff,
    // these fields should always contain (default) shared paths.
    pub shared_rust_output_path: String,
    pub shared_dart_output_path: String,
}

fn bail(err: clap::ErrorKind, message: Cow<str>) {
    log::error!("<{}> {}", err, message);
    std::process::exit(1);
    // RawOpts::command().error(err, "").exit()
}

pub fn parse(raw: RawOpts) -> Result<(Vec<Opts>, Vec<String>)> {
    // rust input path(s)
    let rust_input_paths = get_valid_canon_paths(&raw.rust_input);

    // dart output path(s)
    let dart_output_paths = get_valid_canon_paths(&raw.dart_output);
    if dart_output_paths.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--dart-output's inputs should match --rust-input's length".into(),
        )
    }

    // rust crate dir(s)
    let rust_crate_dirs = raw.rust_crate_dir.unwrap_or_else(|| {
        rust_input_paths
            .iter()
            .map(|each_rust_input_path| {
                fallback_rust_crate_dir(each_rust_input_path)
                    .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("rust_crate_dir")))
            })
            .collect::<Vec<_>>()
    });
    let rust_crate_dirs = rust_crate_dirs
        .iter()
        .map(|each_path| canon_path(each_path))
        .collect::<Vec<_>>();
    if rust_crate_dirs.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--rust-crate-dir's inputs should match --rust-input's length".into(),
        );
    }

    // manifest path(s)
    let manifest_paths = rust_crate_dirs
        .iter()
        .map(|each| {
            let mut path = std::path::PathBuf::from_str(each).unwrap();
            path.push("Cargo.toml");
            path_to_string(path).unwrap()
        })
        .collect::<Vec<_>>();

    // rust output path(s)
    let rust_output_paths = get_outputs_for_flag_requires_full_data(
        &raw.rust_output,
        &rust_input_paths,
        &fallback_rust_output_path,
        "rust_output",
    );
    let rust_output_paths = rust_output_paths
        .iter()
        .map(|each_path| canon_path(each_path))
        .collect::<Vec<_>>();
    if rust_output_paths.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--rust-output's inputs should match --rust-input's length".into(),
        );
    }

    // rust/dart shared output path, used for multi API blocks
    let cloned_rust_output = raw.rust_output.clone();
    let shared_path = raw.shared_rust_output.unwrap_or_else(|| {
        let p = cloned_rust_output
            .map(|vec| vec.get(0).cloned().unwrap_or_default())
            .unwrap_or_default();
        let guessed_relative_parent_dir = Path::new(&p).parent().unwrap_or(Path::new(""));
        Path::join(guessed_relative_parent_dir, "./bridge_generated_shares.rs")
            .into_os_string()
            .into_string()
            .unwrap()
    });
    let shared_rust_output_path = get_valid_canon_paths(&[shared_path])[0].clone();
    log::debug!("shared_rust_output_path:{:?}", shared_rust_output_path);
    let shared_dart_output_path = Path::join(
        Path::new(&dart_output_paths[0]).parent().unwrap(),
        format!(
            "{}.dart",
            Path::new(&shared_rust_output_path).file_name_str().unwrap()
        ),
    )
    .into_os_string()
    .into_string()
    .unwrap();

    // class name(s)
    let class_names = get_outputs_for_flag_requires_full_data(
        &raw.class_name,
        &rust_crate_dirs,
        &fallback_class_name,
        "class_name",
    );
    if !class_names.find_duplicates().is_empty() {
        bail(
            clap::ErrorKind::TooManyOccurrences,
            "there should be no duplication in --class-name's inputs".into(),
        );
    };
    if class_names.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--class-name's inputs should match --rust-input's length".into(),
        );
    }

    let skip_deps_check = raw.skip_deps_check;

    // get correct c outputs for all rust inputs
    let refined_c_outputs =
        get_refined_c_output(&raw.c_output, &raw.extra_c_output_path, &rust_input_paths);

    // dart root(s)
    let dart_roots: Vec<_> = match raw.dart_root {
        Some(dart_roots) => dart_roots
            .into_iter()
            .map(|each_path| Some(canon_path(&each_path)))
            .collect(),
        None => dart_output_paths
            .iter()
            .map(|each_dart_output_path| fallback_dart_root(each_dart_output_path).ok())
            .collect(),
    };

    // build Opt for each rust api block
    let dart_decl_output_path = raw
        .dart_decl_output
        .as_ref()
        .map(|s| canon_path(s.as_str()));
    let dart_format_line_length = raw.dart_format_line_length;
    let llvm_paths = get_llvm_paths(&raw.llvm_path);
    let llvm_compiler_opts = raw.llvm_compiler_opts.clone().unwrap_or_default();
    let skip_add_mod_to_lib = raw.skip_add_mod_to_lib;
    let build_runner = !raw.no_build_runner;
    let wasm = raw.wasm;
    let inline_rust = raw.inline_rust;

    let regular_configs = (0..rust_input_paths.len())
        .map(|i| {
            Opts {
                rust_input_path: rust_input_paths[i].clone(),
                dart_output_path: dart_output_paths[i].clone(),
                dart_decl_output_path: dart_decl_output_path.clone(),
                c_output_path: refined_c_outputs[i].clone(),
                rust_crate_dir: rust_crate_dirs[i].clone(),
                rust_output_path: rust_output_paths[i].clone(),
                class_name: class_names[i].clone(),
                dart_format_line_length,
                skip_add_mod_to_lib, //same for all rust api blocks
                llvm_path: llvm_paths.clone(),
                llvm_compiler_opts: llvm_compiler_opts.clone(),
                manifest_path: manifest_paths[i].clone(),
                dart_root: dart_roots[i].clone(),
                build_runner, //same for all rust api blocks
                block_index: BlockIndex(i),
                skip_deps_check,
                wasm_enabled: wasm,
                inline_rust,
                shared: false,
                shared_rust_output_path: shared_rust_output_path.clone(),
                shared_dart_output_path: shared_dart_output_path.clone(),
            }
        })
        .collect::<Vec<_>>();

    // if shared API-block(config) is essential, generate and add it to vec
    let (no_duplicated_symbols, shared_symbols) = get_symbols_if_no_duplicates(&regular_configs)?;
    let all_configs = if shared_symbols.is_empty() {
        regular_configs
    } else {
        let shared_config = Opts {
            rust_input_path: "".into(), // this field is meangingless for shared block, so it's empty.
            rust_output_path: shared_rust_output_path.clone(),
            dart_output_path: shared_dart_output_path.clone(),
            dart_decl_output_path: dart_decl_output_path.clone(),
            rust_crate_dir: regular_configs[0].rust_crate_dir.clone(),
            c_output_path: vec![], // no need to gnerated other c header files, even there is a shared block.
            class_name: Path::new(&shared_dart_output_path)
                .file_name_str()
                .unwrap()
                .to_case(Case::Pascal),
            dart_format_line_length,
            skip_add_mod_to_lib,
            llvm_path: llvm_paths.clone(),
            llvm_compiler_opts,
            manifest_path: manifest_paths[0].clone(),
            dart_root: dart_roots[0].clone(),
            build_runner,
            block_index: BlockIndex(regular_configs.len()),
            skip_deps_check: true,
            wasm_enabled: wasm,
            inline_rust,
            shared: true,
            shared_rust_output_path,
            shared_dart_output_path,
        };
        [regular_configs, vec![shared_config]].concat()
    };

    Ok((all_configs, no_duplicated_symbols))
}

fn get_refined_c_output(
    c_output: &Option<Vec<String>>,
    extra_c_output_path: &Option<Vec<String>>,
    rust_input_paths: &Vec<String>,
) -> Vec<Vec<String>> {
    assert!(!rust_input_paths.is_empty());

    // 1.c path with file name from flag rawOpt.c_output
    let c_output_paths = c_output
        .as_ref()
        .map(|outputs| {
            outputs
                .iter()
                .map(|output| canon_path(output))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            (0..rust_input_paths.len())
                .map(|_| {
                    fallback_c_output_path()
                        .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("c_output")))
                })
                .collect()
        });
    if c_output_paths.len() != rust_input_paths.len() {
        bail(
            clap::ErrorKind::WrongNumberOfValues,
            "--c-output's inputs should match --rust-input's length".into(),
        );
    }
    // 2.extra c path from flag rawOpt.extra_c_output_path
    let extra_c_output_paths = extra_c_output_path
        .as_deref()
        .unwrap_or_default()
        .iter()
        .flat_map(|extra_path| {
            c_output_paths.iter().map(move |c| {
                let path = Path::new(c);
                let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
                Path::join(extra_path.as_ref(), file_name)
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
        })
        .collect::<Vec<_>>();
    // 3.integrate c output path(s) for each rust input API block
    let refined_c_outputs = c_output_paths
        .iter()
        .enumerate()
        .map(|(i, each_a)| {
            let mut first = vec![each_a.clone()];
            if !extra_c_output_paths.is_empty() {
                let iter = extra_c_output_paths[i..]
                    .iter()
                    .step_by(c_output_paths.len())
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();
                first.extend(iter);
            }

            first
        })
        .collect::<Vec<_>>();

    refined_c_outputs
}

fn get_outputs_for_flag_requires_full_data(
    strings: &Option<Vec<String>>,
    fallback_paths: &[String],
    fallback_func: &dyn Fn(&str) -> Result<String>,
    field_str: &str, // str with underline, like "class_name"
) -> Vec<String> {
    strings.clone().unwrap_or_else(|| -> Vec<String> {
        if fallback_paths.len() == 1 {
            vec![fallback_func(&fallback_paths[0])
                .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error(field_str)))]
        } else {
            let strs = field_str.split('_').collect::<Vec<_>>();
            let raw_str = strs.join(" ");
            let flag_str = strs.join("-");
            RawOpts::command()
                .error(
                    clap::ErrorKind::ValueValidation,
                    format!(
                        "for more than 1 rust blocks, please specify each {raw_str} clearly with flag \"{flag_str}\""
                    ),
                )
                .exit()
        }
    })
}

fn get_llvm_paths(llvm_path: &Option<Vec<String>>) -> Vec<String> {
    llvm_path.clone().unwrap_or_else(|| {
        vec![
            "/opt/homebrew/opt/llvm".to_owned(), // Homebrew root
            "/usr/local/opt/llvm".to_owned(),    // Homebrew x86-64 root
            // Possible Linux LLVM roots
            "/usr/lib/llvm-9".to_owned(),
            "/usr/lib/llvm-10".to_owned(),
            "/usr/lib/llvm-11".to_owned(),
            "/usr/lib/llvm-12".to_owned(),
            "/usr/lib/llvm-13".to_owned(),
            "/usr/lib/llvm-14".to_owned(),
            "/usr/lib/".to_owned(),
            "/usr/lib64/".to_owned(),
            "C:/Program Files/llvm".to_owned(), // Default on Windows
            "C:/msys64/mingw64".to_owned(), // https://packages.msys2.org/package/mingw-w64-x86_64-clang
        ]
    })
}

fn get_valid_canon_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|p| !p.trim().is_empty())
        .map(|p| canon_path(p))
        .collect::<Vec<_>>()
}

fn format_fail_to_guess_error(name: &str) -> String {
    format!("fail to guess {name}, please specify it manually in command line arguments")
}

fn fallback_rust_crate_dir(rust_input_path: &str) -> Result<String> {
    let mut dir_curr = Path::new(rust_input_path)
        .parent()
        .ok_or_else(|| anyhow!(""))?;

    loop {
        let path_cargo_toml = dir_curr.join("Cargo.toml");

        if path_cargo_toml.exists() {
            return Ok(dir_curr
                .as_os_str()
                .to_str()
                .ok_or_else(|| anyhow!(""))?
                .to_string());
        }

        if let Some(next_parent) = dir_curr.parent() {
            dir_curr = next_parent;
        } else {
            break;
        }
    }
    Err(anyhow!(
        "look at parent directories but none contains Cargo.toml"
    ))
}

fn fallback_c_output_path() -> Result<String> {
    let named_temp_file = Box::leak(Box::new(tempfile::Builder::new().suffix(".h").tempfile()?));
    Ok(named_temp_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow!(""))?
        .to_string())
}

fn fallback_rust_output_path(rust_input_path: &str) -> Result<String> {
    Ok(Path::new(rust_input_path)
        .parent()
        .ok_or_else(|| anyhow!(""))?
        .join("bridge_generated.rs")
        .to_str()
        .ok_or_else(|| anyhow!(""))?
        .to_string())
}

fn fallback_dart_root(dart_output_path: &str) -> Result<String> {
    let mut res = canon_pathbuf(dart_output_path);
    while res.pop() {
        if res.join("pubspec.yaml").is_file() {
            return res
                .to_str()
                .map(ToString::to_string)
                .ok_or_else(|| anyhow!("Non-utf8 path"));
        }
    }
    Err(anyhow!(
        "Root of Dart library could not be inferred from Dart output"
    ))
}

fn fallback_class_name(rust_crate_dir: &str) -> Result<String> {
    let cargo_toml_path = Path::new(rust_crate_dir).join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;

    let cargo_toml_value = cargo_toml_content.parse::<Value>()?;
    let package_name = cargo_toml_value
        .get("package")
        .ok_or_else(|| anyhow!("no `package` in Cargo.toml"))?
        .get("name")
        .ok_or_else(|| anyhow!("no `name` in Cargo.toml"))?
        .as_str()
        .ok_or_else(|| anyhow!(""))?;

    Ok(package_name.to_case(Case::Pascal))
}

fn canon_path(sub_path: &str) -> String {
    let path = canon_pathbuf(sub_path);
    path_to_string(path).unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path))
}

fn canon_pathbuf(sub_path: &str) -> PathBuf {
    let mut path =
        env::current_dir().unwrap_or_else(|_| panic!("fail to parse path: {}", sub_path));
    path.push(sub_path);
    path
}

fn path_to_string(path: PathBuf) -> Result<String, OsString> {
    path.into_os_string().into_string()
}

impl Opts {
    pub fn get_ir_file(&self, all_configs: Option<&[Opts]>) -> Result<IrFile> {
        let raw_ir_file = if !self.shared {
            self.get_regular_ir_file()?
        } else {
            self.get_shared_ir_file(all_configs)?
        };
        log::debug!("Phase: Transform IR");
        Ok(transformer::transform(raw_ir_file))
    }

    fn get_regular_ir_file(&self) -> Result<IrFile> {
        log::debug!("Phase: Parse source code to AST");
        let source_rust_content = fs::read_to_string(&self.rust_input_path).with_context(|| {
            let err_msg = format!(
                "Failed to read rust input file \"{}\"",
                self.rust_input_path
            );
            log::error!("{}", err_msg);
            err_msg
        })?;
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        log::debug!("Phase: Parse AST to IR");
        Ok(parser::parse(
            &source_rust_content,
            file_ast,
            &self.manifest_path,
            self.block_index,
            self.shared,
        ))
    }

    fn get_shared_ir_file(&self, all_configs: Option<&[Opts]>) -> Result<IrFile> {
        log::debug!("get_shared_ir_file 1"); //TODO: delete
        let all_configs = all_configs.expect("to get shared ir_file, `all_configs` should not be none!");
        let (regular_configs, shared_index) = if !all_configs.last().unwrap().shared {
            (all_configs, all_configs.len())
        } else {
            let last_index = all_configs.len() - 1;
            (&all_configs[0..last_index], last_index)
        };

        log::debug!("get_shared_ir_file 2"); //TODO: delete
        log::debug!("configs len:{}", regular_configs.len()); //TODO: delete

        assert!(regular_configs.len() > 1);
        assert!(regular_configs.iter().all(|c| c.shared == false));

        log::debug!("get_shared_ir_file 3"); //TODO: delete

        // get regular ir_files
        let mut funcs = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        for config in regular_configs {
            let ir_file = config.get_ir_file(None)?;

            // funcs.extend(raw_ir_file.funcs);// TODO: only methods of shared structs needed
            structs.extend(ir_file.struct_pool);
            enums.extend(ir_file.enum_pool);
        }
        let shared_structs= structs.find_duplicates();
        let shared_enums = enums.find_duplicates();

        let struct_pool = shared_structs
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();
        let enum_pool = shared_enums
            .into_iter()
            .map(|x| (x.0, x.1))
            .collect::<HashMap<_, _>>();

        log::debug!("the shared struct_pool:{:?}", struct_pool); //TODO: delete
        log::debug!("the shared enum_pool:{:?}", enum_pool); //TODO: delete

        Ok(IrFile {
            funcs: funcs, // this field would effect `IrFile.visit_types(...)` and others
            struct_pool: struct_pool,
            enum_pool: enum_pool, // TODO: check with customized enum struct
            has_executor: true,   // set true, in case for the methods of a shared struct,
            block_index: BlockIndex(shared_index),
            shared: true,
        })
    }

    pub fn dart_api_class_name(&self) -> String {
        self.class_name.clone()
    }

    pub fn dart_api_impl_class_name(&self) -> String {
        format!("{}Impl", self.class_name)
    }

    pub fn dart_wire_class_name(&self) -> String {
        format!("{}Wire", self.class_name)
    }

    pub fn dart_platform_class_name(&self) -> String {
        format!("{}Platform", self.class_name)
    }

    pub fn dart_wasm_module(&self) -> String {
        format!("{}WasmModule", self.class_name)
    }

    pub(crate) fn dart_output_root(&self) -> Option<&str> {
        Path::new(
            self.dart_decl_output_path
                .as_ref()
                .unwrap_or(&self.dart_output_path),
        )
        .file_stem()?
        .to_str()
    }

    pub fn dart_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("web.dart")
    }

    pub fn dart_io_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("io.dart")
    }

    pub fn dart_common_output_path(&self) -> PathBuf {
        Path::new(&self.dart_output_path).with_extension("common.dart")
    }

    pub fn rust_wasm_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("web.rs")
    }

    pub fn rust_io_output_path(&self) -> PathBuf {
        Path::new(&self.rust_output_path).with_extension("io.rs")
    }

    pub fn dart_root_or_default(&self) -> String {
        self.dart_root
            .clone()
            .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").to_string())
    }

    pub fn dart_freezed_path(&self) -> PathBuf {
        PathBuf::from(
            self.dart_decl_output_path
                .as_deref()
                .unwrap_or(&self.dart_output_path),
        )
        .with_extension("freezed.dart")
    }

    pub fn get_rust_output_paths(&self) -> PathForGeneration {
        PathForGeneration {
            base_path: PathBuf::from(self.rust_output_path.clone()),
            io_path: self.rust_io_output_path(),
            wasm_path: self.rust_wasm_output_path(),
        }
    }

    pub fn get_dart_output_paths(&self) -> PathForGeneration {
        PathForGeneration {
            base_path: PathBuf::from(self.dart_output_path.clone()),
            io_path: self.dart_io_output_path(),
            wasm_path: self.dart_wasm_output_path(),
        }
    }
}

pub struct PathForGeneration {
    pub base_path: PathBuf,
    pub io_path: PathBuf,
    pub wasm_path: PathBuf,
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_dir_and_file_str(path: &str) -> (String, String) {
        let path = PathBuf::from(path);
        let directory = path.parent().unwrap().display().to_string();
        let file_name = path.file_name().unwrap().to_owned().into_string().unwrap();
        (directory, file_name)
    }

    #[test]
    #[should_panic]
    fn test_coutput_with_no_input_block() {
        let rust_input = vec![];
        let c_output = None::<Vec<String>>;
        let extra_c_output_path = None::<Vec<String>>;
        let _ = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);
    }

    // #[test]
    // TODO: how to catch panic from clap error?
    // #[should_panic]
    // fn test_coutput_with_inconsistent_number_of_input_block_api() {
    //     let c_output = Some(vec!["pathA/api_1.rs".into(),"pathA/api_2.rs".into()]);
    //     let rust_input = vec!["api_1.rs".into()];
    //     let extra_c_output_path = None::<Vec<String>>;
    //     let _ = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);
    //     unreachable !()
    // }

    #[test]
    fn test_single_block_with_no_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }

    #[test]
    fn test_single_block_with_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);

        // check 1st path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
        // check 2ed path
        assert_eq!(&result[0][1], "./extra_path_1/c_output.h");
        // check 3ed path
        assert_eq!(&result[0][2], "./extra_path_2/c_output.h");
    }

    #[test]
    fn test_single_block_with_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);

        // check path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
    }

    #[test]
    fn test_single_block_with_no_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);

        // get essential info from 1st path which has an arbitrary name
        let (_, file) = get_dir_and_file_str(&result[0][0]);
        // check 2ed path
        assert_eq!(&result[0][1], &format!("./extra_path_1/{}", file));
        // check 3ed path
        assert_eq!(&result[0][2], &format!("./extra_path_2/{}", file));
    }

    #[test]
    fn test_multi_blocks_with_no_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });

        result.iter().enumerate().for_each(|(i, each_block)| {
            // check 1st path
            let (dir, file) = get_dir_and_file_str(&each_block[0]);
            assert_eq!(dir, env::current_dir().unwrap().display().to_string());
            assert_eq!(&file, &format!("c_output_{}.h", i + 1));
            // check 2ed path
            assert_eq!(
                &each_block[1],
                &format!("./extra_path_1/c_output_{}.h", i + 1)
            );
            // check 3ed path
            assert_eq!(
                &each_block[2],
                &format!("./extra_path_2/c_output_{}.h", i + 1)
            );
        });
    }

    #[test]
    fn test_multi_blocks_with_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 1);
        });

        // check path
        result.iter().enumerate().for_each(|(i, each_block)| {
            let (dir, file) = get_dir_and_file_str(&each_block[0]);
            assert_eq!(dir, env::current_dir().unwrap().display().to_string());
            assert_eq!(&file, &format!("c_output_{}.h", i + 1));
        });
    }

    #[test]
    fn test_multi_blocks_with_no_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });

        result.iter().for_each(|each_block| {
            // get essential info from 1st path which has an arbitrary name
            let (_, file) = get_dir_and_file_str(&each_block[0]);
            // check 2ed path
            assert_eq!(&each_block[1], &format!("./extra_path_1/{}", file));
            // check 3ed path
            assert_eq!(&each_block[2], &format!("./extra_path_2/{}", file));
        });
    }
}
