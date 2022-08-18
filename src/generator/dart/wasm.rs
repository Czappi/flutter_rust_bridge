use super::*;

const VOID: &str = "void";
const WIRE_SYNC_RETURN_STRUCT: &str = "WireSyncReturnStruct";

pub fn generate_wasm_module<'a>(
    funcs: impl IntoIterator<Item = &'a IrFuncLike>,
    dart_wire_class_name: &str,
    dart_wasm_module_name: &str,
) -> String {
    format!(
        "class {cls} extends FlutterRustBridgeWasmWireBase<{wasm}> {{
            {cls}(FutureOr<WasmModule> module) : super(WasmModule.cast<{wasm}>(module));
            
            {}
        }}
        ",
        funcs
            .into_iter()
            .map(generate_wasm_wire_func_method)
            .collect::<Vec<_>>()
            .join("\n\n"),
        cls = dart_wire_class_name,
        wasm = dart_wasm_module_name,
    )
}

pub fn push_wasm_module(
    lines: &mut Acc<Vec<String>>,
    dart_wasm_funcs: &[String],
    dart_wasm_module: Option<&str>,
    config: &Opts,
) {
    let dart_wasm_module_name = config.dart_wasm_module();
    lines.wasm.push(section_header("WASM wire module"));
    lines.wasm.push(format!(
        "@JS('wasm_bindgen') external {} get wasmModule;",
        dart_wasm_module_name,
    ));
    lines.wasm.push(format!(
        "@JS() @anonymous class {wasm} implements WasmModule {{
            external Object /* Promise */ call([String? moduleName]);
            external {wasm} bind(dynamic thisArg, String moduleName);

            // external void __start_streamsink(String name);
        ",
        wasm = dart_wasm_module_name,
    ));
    lines.wasm.push(dart_wasm_funcs.join("\n\n"));
    lines.wasm.push("}\n".into());

    lines.wasm.push(section_header("WASM wire connector"));
    (lines.wasm).push(dart_wasm_module.unwrap_or_default().to_string());
}

fn is_rust_pointer(ty: &str) -> bool {
    ty.starts_with("*mut") || ty.starts_with("*const")
}

/// Since there exists no toolchain that can generate Dart bindings
/// for JS code, we have to supply our own stubs. The external function
/// generator however converts an [IrType] (sometimes a raw string)
/// into its string representation, so these heuristics parse those representations
/// into their appropriate Dart wire types. Note that only a subset of types
/// is supported, outside of which `dynamic` is returned.
///
/// In practice however this is optional as unlike Rust, Dart values are
/// aware of their own types (via the `runtimeType` property) and can
/// safely assume the `dynamic` or `Object` type instead.
pub fn reconstruct_dart_wire_from_rust_wire(ty: &str) -> Cow<str> {
    let ty = ty.trim();
    if matches!(ty, VOID | WIRE_SYNC_RETURN_STRUCT) {
        return ty.into();
    }
    if is_rust_pointer(ty) {
        return format!("int /* {} */", ty).into();
    }
    warn!("unknown string repr of ty={}", ty);
    format!("dynamic /* {} */", ty).into()
}

pub fn generate_wasm_wire_func_decl(func: &IrFuncLike) -> String {
    format!(
        "external {} {name}({});",
        reconstruct_dart_wire_from_rust_wire(&func.output),
        func.inputs
            .iter()
            .map(|(key, ty)| format!("{} {}", ty, key))
            .join(","),
        name = func.name,
    )
}

pub fn generate_wasm_wire_func_method(func: &IrFuncLike) -> String {
    format!(
        "{out} {name}({}) => wasmModule.{name}({});",
        func.inputs
            .iter()
            .map(|(key, ty)| format!("{} {}", ty, key))
            .join(","),
        func.inputs.iter().map(|(key, _)| key.to_string()).join(","),
        name = func.name,
        out = if func.has_port_argument {
            "void".into()
        } else {
            reconstruct_dart_wire_from_rust_wire(&func.output)
        },
    )
}
