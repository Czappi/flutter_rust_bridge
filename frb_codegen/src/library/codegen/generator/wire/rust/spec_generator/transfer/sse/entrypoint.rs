use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::transfer::base::WireRustTransferEntrypointTrait;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct SseWireRustTransferEntrypoint {}

impl WireRustTransferEntrypointTrait for SseWireRustTransferEntrypoint {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        todo!()
    }

    fn generate_func_call_decode(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        todo!()
    }
}
