use crate::config::Acc;
use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let wasm = self.context.config.wasm_enabled;
        match self.ir {
            IrTypeDelegate::String => Acc {
                io: Some("return _api2wire_uint_8_list(utf8.encoder.convert(raw));".into()),
                wasm: wasm.then(|| "return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                // In this case, even though the body is the same, their types are different
                // and must be split.
                let body = format!(
                    "return _api2wire_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                );
                Acc::distribute(Some(body))
            }
            IrTypeDelegate::StringList => Acc {
                io: Some(format!(
                    "final ans = inner.new_StringList_{}(raw.length);
                    for (var i = 0; i < raw.length; i++){{
                        ans.ref.ptr[i] = _api2wire_String(raw[i]);
                    }}
                    return ans;",
                    self.context.config.block_index
                )),
                wasm: wasm.then(|| "return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum { ref repr, .. } => {
                format!("return _api2wire_{}(raw.index);", repr.safe_ident()).into()
            }
        }
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::String
            | IrTypeDelegate::SyncReturnVecU8
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&self.ir.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                format!("return {}.values[raw];", ir.dart_api_type())
            }
        }
    }

    fn structs(&self) -> String {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &self.ir {
            super::TypeEnumRefGenerator {
                ir: ir.clone(),
                context: TypeGeneratorContext {
                    dart_api_class_name: None,
                    ..self.context.clone()
                },
            }
            .structs()
        } else {
            "".into()
        }
    }
}
