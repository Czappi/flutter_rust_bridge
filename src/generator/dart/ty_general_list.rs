use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self) -> Option<String> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        if self.context.config.wasm {
            Some(format!(
                "return raw.map(_api2wire_{}).toList();",
                self.ir.inner.safe_ident()
            ))
        } else {
            Some(format!(
                "final ans = inner.new_{}_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
                self.ir.safe_ident(),
                self.context.config.block_index,
                self.ir.inner.safe_ident()
            ))
        }
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
