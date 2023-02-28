use crate::{ir::*, parser::DefaultValues};

#[derive(Debug, Clone)]
pub struct IrField {
    pub ty: IrType,
    pub name: IrIdent,
    pub is_final: bool,
    pub comments: Vec<IrComment>,
    pub default: Option<DefaultValues>,
}

impl IrField {
    #[inline]
    pub fn required_modifier(&self) -> &str {
        if self.default.is_some() {
            ""
        } else {
            self.ty.dart_required_modifier()
        }
    }
    pub fn field_default(&self, freezed: bool) -> String {
        self.default
            .as_ref()
            .map(|default| {
                let default = match default {
                    DefaultValues::Str(lit)
                        if !matches!(&self.ty, IrType::Delegate(IrTypeDelegate::String)) =>
                    {
                        lit.value().into()
                    }
                    _ => default.to_dart(),
                };
                if freezed {
                    format!("@Default({default})")
                } else {
                    format!("= {default}")
                }
            })
            .unwrap_or_default()
    }
}
