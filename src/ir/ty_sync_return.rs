use crate::ir::*;

/// Types that have synchronized return
/// NOTE for maintainer: Please make sure all the types here
/// can be parsed by `executeSync` function in basic.dart.
#[derive(Debug, Clone)]
pub enum IrTypeSyncReturn {
    Primitive(IrTypePrimitive),
    String,
    VecU8,
}

impl IrTypeTrait for IrTypeSyncReturn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_inner().visit_types(f, ir_file)
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeSyncReturn::Primitive(_) => {
                "SyncReturn_".to_owned() + &self.get_inner().rust_api_type()
            }
            _ => "SyncReturn_".to_owned() + &self.get_inner().dart_api_type(),
        }
    }

    fn dart_api_type(&self) -> String {
        self.get_inner().dart_api_type()
    }

    fn dart_wire_type(&self) -> String {
        self.get_inner().dart_wire_type()
    }

    fn rust_api_type(&self) -> String {
        format!("SyncReturn<{}>", self.get_inner().rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        self.get_inner().rust_wire_type()
    }

    fn rust_wire_is_pointer(&self) -> bool {
        self.get_inner().rust_wire_is_pointer()
    }
}

impl IrTypeSyncReturn {
    pub fn get_inner(&self) -> IrType {
        match self {
            IrTypeSyncReturn::Primitive(primitive) => IrType::Primitive(primitive.clone()),
            IrTypeSyncReturn::String => IrType::Delegate(IrTypeDelegate::String),
            IrTypeSyncReturn::VecU8 => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
        }
    }
}
