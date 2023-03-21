use std::borrow::Cow;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use clap::CommandFactory;
use clap::Parser;
use clap::ValueEnum;
use convert_case::{Case, Casing};
use serde::Deserialize;
use toml::Value;

use crate::ir::IrFile;
use crate::parser;
use crate::utils::misc::{find_all_duplicates, BlockIndex};

#[derive(Parser, Debug, PartialEq, Eq, Deserialize, Default)]
#[command(version)]
#[command(override_usage(
    "flutter_rust_bridge_codegen [OPTIONS] --rust-input <RUST_INPUT>... --dart-output <DART_OUTPUT>...
       flutter_rust_bridge_codegen [CONFIG_FILE]"
))]
pub struct RawOpts {
    /// Path of input Rust code
    #[arg(short, long, required_unless_present = "config_file", num_args = 1..)]
    pub rust_input: Vec<String>,

    /// Path of output generated Dart code
    #[arg(short, long, required_unless_present = "config_file", num_args = 1..)]
    pub dart_output: Vec<String>,

    /// Path to a YAML config file.
    ///
    /// If present, other options and flags will be ignored.
    /// Accepts the same options as the CLI, but uses snake_case keys.
    #[serde(skip)]
    pub config_file: Option<String>,

    /// If provided, generated Dart declaration code to this separate file
    #[arg(long)]
    pub dart_decl_output: Option<String>,

    /// Output path (including file name) of generated C header, each field corresponding to that of --rust-input.
    #[arg(short, long)]
    pub c_output: Option<Vec<String>>,

    /// Extra output path (excluding file name) of generated C header
    #[arg(short, long)]
    pub extra_c_output_path: Option<Vec<String>>,

    /// Crate directory for your Rust project
    #[arg(long, num_args = 1..)]
    pub rust_crate_dir: Option<Vec<String>>,

    /// Output path of generated Rust code
    #[arg(long, num_args = 1..)]
    pub rust_output: Option<Vec<String>>,

    /// Generated class name
    #[arg(long, num_args = 1..)]
    pub class_name: Option<Vec<String>>,

    /// Line length for Dart formatting
    #[arg(long, default_value = "80")]
    pub dart_format_line_length: u32,

    /// The generated Dart enums will have their variant names camelCased.
    #[arg(long)]
    #[serde(default)]
    pub dart_enums_style: bool,

    /// Skip automatically adding `mod bridge_generated;` to `lib.rs`
    #[arg(long)]
    #[serde(default)]
    pub skip_add_mod_to_lib: bool,

    /// Path to the installed LLVM
    #[arg(long, num_args = 1..)]
    pub llvm_path: Option<Vec<String>>,

    /// LLVM compiler opts
    #[arg(long)]
    pub llvm_compiler_opts: Option<String>,

    /// Path to root of Dart project, otherwise inferred from --dart-output
    #[arg(long, num_args = 1..)]
    pub dart_root: Option<Vec<String>>,

    /// Skip running build_runner even when codegen-required code is detected
    #[arg(long)]
    #[serde(default)]
    pub no_build_runner: bool,

    /// Show debug messages.
    #[arg(short, long)]
    #[serde(default)]
    pub verbose: bool,

    /// Enable WASM module generation.
    /// Requires: --dart-decl-output
    #[arg(long)]
    #[serde(default)]
    pub wasm: bool,

    /// Inline declaration of Rust bridge modules
    #[arg(long)]
    #[serde(default)]
    pub inline_rust: bool,

    /// Skip dependencies check.
    #[arg(long)]
    #[serde(default)]
    pub skip_deps_check: bool,

    /// A list of data to be dumped. If specified without a value, defaults to all.
    #[cfg(feature = "serde")]
    #[arg(long, value_enum, num_args(0..))]
    pub dump: Option<Vec<Dump>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Opts {
    pub rust_input_path: String,
    pub dart_output_path: String,
    pub dart_decl_output_path: Option<String>,
    pub c_output_path: Vec<String>,
    pub rust_crate_dir: String,
    pub rust_output_path: String,
    pub class_name: String,
    pub dart_format_line_length: u32,
    pub dart_enums_style: bool,
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum Dump {
    Config,
    Ir,
}

#[inline(never)]
fn bail(err: clap::error::ErrorKind, message: Cow<str>) -> ! {
    RawOpts::command().error(err, message).exit()
}

impl RawOpts {
    /// Parses options from arguments, or from a config file if no arguments are given.
    /// Terminates the program if argument validation fails.
    #[cfg(feature = "serde")]
    pub fn try_parse_args_or_yaml() -> anyhow::Result<Self> {
        const CONFIG_LOCATIONS: [&str; 3] = [
            ".flutter_rust_bridge.yml",
            ".flutter_rust_bridge.yaml",
            ".flutter_rust_bridge.json",
        ];
        const PUBSPEC_LOCATIONS: [&str; 2] = ["pubspec.yaml", "pubspec.yml"];

        let has_args = std::env::args_os().len() > 1;
        match Self::try_parse() {
            Ok(opts) => Ok(opts),
            Err(err) if has_args => err.exit(),
            // Try to parse a command file, if exists
            Err(err) => {
                for location in CONFIG_LOCATIONS {
                    if let Ok(file) = fs::File::open(location) {
                        eprintln!("Found config file {location}");
                        return serde_yaml::from_reader(file)
                            .with_context(|| format!("Could not parse {location}"));
                    }
                }

                let mut hint = "fill in .flutter_rust_bridge.yml with your config.".to_owned();
                for location in PUBSPEC_LOCATIONS {
                    if let Ok(pubspec) = fs::File::open(location) {
                        #[derive(serde::Deserialize)]
                        struct Needle {
                            #[serde(rename = "flutter_rust_bridge")]
                            data: Option<RawOpts>,
                        }
                        match serde_yaml::from_reader(pubspec) {
                            Ok(Needle { data: Some(data) }) => return Ok(data),
                            Ok(Needle { data: None }) => {
                                hint = format!("create an entry called 'flutter_rust_bridge' in {location} with your config.");
                            }
                            Err(err) => {
                                return Err(anyhow::Error::new(err).context(format!(
                                    "Could not parse the 'flutter_rust_bridge' entry in {location}"
                                )));
                            }
                        }
                    }
                }

                _ = err.print();
                eprintln!("Hint: To call flutter_rust_bridge_codegen with no arguments, {hint}");
                std::process::exit(1)
            }
        }
    }
}

pub fn parse(mut raw: RawOpts) -> Vec<Opts> {
    if let Some(config) = raw.config_file {
        raw = parse_yaml(&config);
    }

    use clap::error::ErrorKind;
    // rust input path(s)
    let rust_input_paths = get_valid_canon_paths(&raw.rust_input);

    // dart output path(s)
    let dart_output_paths = get_valid_canon_paths(&raw.dart_output);
    if dart_output_paths.len() != rust_input_paths.len() {
        bail(
            ErrorKind::WrongNumberOfValues,
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
            ErrorKind::WrongNumberOfValues,
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
            ErrorKind::WrongNumberOfValues,
            "--rust-output's inputs should match --rust-input's length".into(),
        );
    }

    // class name(s)
    let class_names = get_outputs_for_flag_requires_full_data(
        &raw.class_name,
        &rust_crate_dirs,
        &fallback_class_name,
        "class_name",
    );
    if !find_all_duplicates(&class_names).is_empty() {
        bail(
            ErrorKind::ValueValidation,
            "there should be no duplication in --class-name's inputs".into(),
        );
    };
    if class_names.len() != rust_input_paths.len() {
        bail(
            ErrorKind::WrongNumberOfValues,
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
    let dart_enums_style = raw.dart_enums_style;
    let llvm_paths = get_llvm_paths(&raw.llvm_path);
    let llvm_compiler_opts = raw.llvm_compiler_opts.clone().unwrap_or_default();
    let skip_add_mod_to_lib = raw.skip_add_mod_to_lib;
    let build_runner = !raw.no_build_runner;
    let wasm = raw.wasm;
    let inline_rust = raw.inline_rust;

    (0..rust_input_paths.len())
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
                dart_enums_style,
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
            }
        })
        .collect()
}

/// Terminates the program if either the file doesn't exist, or is invalid.
fn parse_yaml(path: &str) -> RawOpts {
    use clap::error::ErrorKind;

    let file = File::open(path)
        .map_err(|err| {
            bail(
                ErrorKind::Io,
                format!("Could not open {path}: {err}").into(),
            )
        })
        .unwrap();
    let config = serde_yaml::from_reader(file)
        .map_err(|err| {
            bail(
                ErrorKind::InvalidValue,
                format!("Could not parse config from {path}: {err}").into(),
            )
        })
        .unwrap();
    anchor_config(config, path)
}

fn anchor_config(config: RawOpts, config_path: &str) -> RawOpts {
    let config_path = canon_pathbuf(config_path);
    let cwd = config_path.parent().unwrap();
    let anchor = |path: String| {
        if Path::new(&path).is_absolute() {
            path
        } else {
            cwd.join(&path).to_str().unwrap().to_owned()
        }
    };
    let anchor_many = |paths: Vec<String>| paths.into_iter().map(anchor).collect();

    // Don't collapse reassignments into a spread here, as future configs may need to be
    // correctly re-anchored.
    RawOpts {
        rust_input: anchor_many(config.rust_input),
        dart_output: anchor_many(config.dart_output),
        rust_output: config.rust_output.map(anchor_many),
        dart_decl_output: config.dart_decl_output.map(anchor),
        c_output: config.c_output.map(anchor_many),
        extra_c_output_path: config.extra_c_output_path.map(anchor_many),
        rust_crate_dir: config.rust_crate_dir.map(anchor_many),
        dart_root: config.dart_root.map(anchor_many),
        config_file: config.config_file,
        class_name: config.class_name,
        dart_format_line_length: config.dart_format_line_length,
        dart_enums_style: config.dart_enums_style,
        skip_add_mod_to_lib: config.skip_add_mod_to_lib,
        llvm_path: config.llvm_path,
        llvm_compiler_opts: config.llvm_compiler_opts,
        no_build_runner: config.no_build_runner,
        verbose: config.verbose,
        wasm: config.wasm,
        inline_rust: config.inline_rust,
        skip_deps_check: config.skip_deps_check,
        dump: config.dump,
    }
}

fn get_refined_c_output(
    c_output: &Option<Vec<String>>,
    extra_c_output_path: &Option<Vec<String>>,
    rust_input_paths: &Vec<String>,
) -> Vec<Vec<String>> {
    use clap::error::ErrorKind;

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
            ErrorKind::WrongNumberOfValues,
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
            bail(
                clap::error::ErrorKind::ValueValidation,
                format!("for more than 1 rust blocks, please specify each {raw_str} clearly with flag \"{flag_str}\"").into()
            )
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
    pub fn get_ir_file(&self) -> Result<IrFile> {
        // info!("Phase: Parse source code to AST");
        let source_rust_content = fs::read_to_string(&self.rust_input_path).with_context(|| {
            format!(
                "Failed to read rust input file \"{}\"",
                self.rust_input_path
            )
        })?;
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        // info!("Phase: Parse AST to IR");

        Ok(parser::parse(
            &source_rust_content,
            file_ast,
            &self.manifest_path,
        ))
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
