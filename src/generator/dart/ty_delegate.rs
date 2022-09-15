use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        match self.ir {
            IrTypeDelegate::String => Acc {
                io: Some("return api2wire_uint_8_list(utf8.encoder.convert(raw));".into()),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                // In this case, even though the body is the same, their types are different
                // and must be split.
                let body = format!(
                    "return api2wire_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                );
                Acc::distribute(Some(body))
            }
            IrTypeDelegate::StringList => Acc {
                io: Some(format!(
                    "final ans = inner.new_StringList_{}(raw.length);
                    for (var i = 0; i < raw.length; i++){{
                        ans.ref.ptr[i] = api2wire_String(raw[i]);
                    }}
                    return ans;",
                    self.context.config.block_index
                )),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum { ref repr, .. } => {
                format!("return api2wire_{}(raw.index);", repr.safe_ident()).into()
            }
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ref ir) => match ir {
                IrTypeTime::Utc => Acc {
                    io: Some("return raw.microsecondsSinceEpoch;".into()),
                    wasm: Some("return raw.millisecondsSinceEpoch;".into()),
                    ..Default::default()
                },
                IrTypeTime::Local | IrTypeTime::Naive => Acc {
                    io: Some("return raw.toUtc().microsecondsSinceEpoch;".into()),
                    wasm: Some("return raw.toUtc().millisecondsSinceEpoch;".into()),
                    ..Default::default()
                },
                IrTypeTime::Duration => Acc {
                    io: Some("return raw.inMicroseconds;".into()),
                    wasm: Some("return raw.inMilliseconds;".into()),
                    ..Default::default()
                },
            },
        }
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                IrTypePrimitive::I64 | IrTypePrimitive::U64,
            ) => {
                format!(
                    "return _wire2api_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                )
            }
            IrTypeDelegate::String | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&self.ir.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                format!("return {}.values[raw];", ir.dart_api_type())
            }
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                // TODO: alternate body based on platform
                IrTypeTime::Naive | IrTypeTime::Local | IrTypeTime::Utc => "
                return DateTime.fromMillisecondsSinceEpoch(_wire2api_i64(raw), isUtc: true);"
                    .to_owned(),
                IrTypeTime::Duration => "
                return Duration(milliseconds: _wire2api_i64(raw));"
                    .to_owned(),
            },
        }
    }

    fn structs(&self) -> String {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &self.ir {
            super::TypeEnumRefGenerator {
                ir: ir.clone(),
                context: self.context.clone(),
            }
            .structs()
        } else {
            "".into()
        }
    }
}
