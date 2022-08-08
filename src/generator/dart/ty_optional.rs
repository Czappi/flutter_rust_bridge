use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "return raw == null ? ffi.nullptr : api2wire_{}(raw);",
                self.ir.inner.safe_ident()
            )),
            wasm: self.context.wasm().then(|| {
                format!(
                    "return raw == null ? null : api2wire_{}(raw);",
                    self.ir.inner.safe_ident()
                )
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        if !self.ir.needs_initialization() || self.ir.is_list() || self.ir.is_boxed_primitive() {
            return None;
        }
        Some(format!(
            "if (apiObj != null) _api_fill_to_wire_{}(apiObj, wireObj);",
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return raw == null ? null : _wire2api_{}(raw);",
            self.ir.inner.safe_ident()
        )
    }
}
