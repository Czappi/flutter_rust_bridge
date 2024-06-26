use crate::codegen::generator::api_dart;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::misc::wire_func::wire_func_name;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) fn generate_api_impl_normal_function(
    func: &IrFunc,
    context: WireDartGeneratorContext,
) -> anyhow::Result<WireDartOutputCode> {
    let dart2rust_codec = WireDartCodecEntrypoint::from(func.codec_mode_pack.dart2rust);

    let api_dart_func =
        api_dart::spec_generator::function::generate(func, context.as_api_dart_context())?;

    let const_meta_field_name = format!("k{}ConstMeta", func.name.name.to_case(Case::Pascal));

    let wire_func_name = wire_func_name(func);
    let inner_func_stmt = dart2rust_codec.generate_dart2rust_inner_func_stmt(func, &wire_func_name);
    let execute_func_name = generate_execute_func_name(func);

    let codec = generate_rust2dart_codec_object(func);
    let call_ffi_args = generate_call_ffi_args(func);
    let arg_values = generate_arg_values(func);

    let task_class = generate_task_class(func);

    let func_expr = api_dart_func.func_expr;

    let call_handler = format!(
        "handler.{execute_func_name}({task_class}(
            callFfi: ({call_ffi_args}) {{
              {inner_func_stmt}
            }},
            codec: {codec},
            constMeta: {const_meta_field_name},
            argValues: [{arg_values}],
            apiImpl: this,
            hint: hint,
        ))",
    );
    let function_implementation_body = if let Some(return_stream) = &api_dart_func.return_stream {
        format!(
            "
            final {return_stream_name} = {return_stream_type}();
            {maybe_await}{call_handler};
            return {return_stream_name}.stream;
            ",
            return_stream_name = return_stream.field.name.dart_style(),
            return_stream_type = ApiDartGenerator::new(
                return_stream.field.ty.clone(),
                context.as_api_dart_context()
            )
            .dart_api_type(),
            maybe_await = if func.mode != IrFuncMode::Sync {
                "await "
            } else {
                ""
            },
        )
    } else {
        format!("return {call_handler};")
    };
    let function_implementation = format!(
        "@override {func_expr} {maybe_async} {{ {function_implementation_body} }}",
        maybe_async = if func.mode != IrFuncMode::Sync && api_dart_func.return_stream.is_some() {
            "async "
        } else {
            ""
        },
    );

    let companion_field_implementation = generate_companion_field(func, &const_meta_field_name);

    Ok(WireDartOutputCode {
        api_class_body: format!("{func_expr};\n\n"),
        api_impl_class_body: format!(
            "{function_implementation}\n\n{companion_field_implementation}\n\n"
        ),
        ..Default::default()
    })
}

fn generate_execute_func_name(func: &IrFunc) -> &str {
    match func.mode {
        IrFuncMode::Normal => "executeNormal",
        IrFuncMode::Sync => "executeSync",
    }
}

fn generate_task_class(func: &IrFunc) -> &str {
    match func.mode {
        IrFuncMode::Normal => "NormalTask",
        IrFuncMode::Sync => "SyncTask",
    }
}

fn generate_companion_field(func: &IrFunc, const_meta_field_name: &str) -> String {
    format!(
        r#"
        TaskConstMeta get {const_meta_field_name} => const TaskConstMeta(
            debugName: "{}",
            argNames: [{}],
        );
        "#,
        func.name.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect_vec()
            .join(", "),
    )
}

fn generate_call_ffi_args(func: &IrFunc) -> &str {
    if func.mode == IrFuncMode::Sync {
        ""
    } else {
        "port_"
    }
}

fn generate_arg_values(func: &IrFunc) -> String {
    (func.inputs.iter())
        .map(|input| input.name.dart_style())
        .join(", ")
}

fn generate_rust2dart_codec_object(func: &IrFunc) -> String {
    let codec_mode = func.codec_mode_pack.rust2dart;
    let codec_name_pascal = codec_mode.delegate_or_self().to_string();
    let codec_name_snake = codec_name_pascal.to_case(Case::Snake);

    let parse_success_data = format!("{codec_name_snake}_decode_{}", func.output.safe_ident());
    let parse_error_data = if let Some(error_output) = &func.error_output {
        format!("{codec_name_snake}_decode_{}", error_output.safe_ident())
    } else {
        "null".to_string()
    };

    format!(
        "
        {codec_name_pascal}Codec(
          decodeSuccessData: {parse_success_data},
          decodeErrorData: {parse_error_data},
        )
        "
    )
}
