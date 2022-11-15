use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOpaqueGenerator, IrTypeOpaque);

impl TypeDartGeneratorTrait for TypeOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "final ptr = inner.new_{0}();
                _api_fill_to_wire_{0}(raw, ptr);
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some("return raw.share();".to_owned()),
            ..Default::default()
        }
    }

    fn api_validate(&self) -> Option<String> {
        Some(
            "if (raw.isStale()) {
            throw StateError('Use after dispose.');
            }"
            .to_owned(),
        )
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("wireObj.ptr = apiObj.share();".into())
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return {0}.fromRaw(raw[0], raw[1], this);",
            self.ir.dart_api_type()
        )
    }

    fn structs(&self) -> String {
        let field_bridge = format!(
            "final {} bridge;",
            self.context.config.dart_api_class_name(),
        );
        format!(
            "@sealed class {0} extends FrbOpaque {{
                {field_bridge}
                    {0}.fromRaw(int ptr, int size, this.bridge) : super.unsafe(ptr, size);
                    @override
                    get dropFn => bridge.drop_opaque_{0};
                    
                    @override
                    get shareFn => bridge.share_opaque_{0};

                    @override
                    get staticFinalizer => bridge.{0}Finalizer;
            }}",
            self.ir.dart_api_type()
        )
    }
}
