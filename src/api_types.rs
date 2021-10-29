use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};

use ApiType::*;

pub type ApiStructPool = HashMap<String, ApiStruct>;

#[derive(Debug, Clone)]
pub struct ApiFile {
    pub funcs: Vec<ApiFunc>,
    pub struct_pool: ApiStructPool,
    pub has_executor: bool,
}

impl ApiFile {
    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
            if include_func_inputs {
                for field in &func.inputs {
                    field.ty.visit_types(f, self);
                }
            }
            if include_func_output {
                func.output.visit_types(f, self);
            }
        }
    }

    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> Vec<ApiType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(ty.clone());
                }
                contains
            },
            include_func_inputs,
            include_func_output,
        );
        ans
    }
}

#[derive(Debug, Clone)]
pub struct ApiFunc {
    pub name: String,
    pub inputs: Vec<ApiField>,
    pub output: ApiType,
    pub mode: ApiFuncMode,
}

impl ApiFunc {
    pub fn wire_func_name(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum ApiFuncMode {
    Normal,
    Stream,
}

impl ApiFuncMode {
    pub fn dart_return_type(&self) -> &'static str {
        match self {
            Self::Normal => "Future",
            Self::Stream => "Stream",
        }
    }

    pub fn ffi_call_mode(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Stream => "Stream",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiIdent {
    pub raw: String,
}

impl ApiIdent {
    pub fn new(raw: String) -> ApiIdent {
        ApiIdent { raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.raw
    }

    pub fn dart_style(&self) -> String {
        self.raw.to_case(Case::Camel)
    }
}

#[derive(Debug, Clone)]
pub enum ApiType {
    Primitive(ApiTypePrimitive),
    Delegate(ApiTypeDelegate),
    PrimitiveList(ApiTypePrimitiveList),
    Optional(ApiTypeOptional),
    GeneralList(Box<ApiTypeGeneralList>),
    StructRef(ApiTypeStructRef),
    Boxed(Box<ApiTypeBoxed>),
}

macro_rules! api_type_call_child {
    ($func:ident, $ret:ty) => {
        pub fn $func(&self) -> $ret {
            match self {
                Primitive(inner) => inner.$func(),
                Delegate(inner) => inner.$func(),
                PrimitiveList(inner) => inner.$func(),
                GeneralList(inner) => inner.$func(),
                StructRef(inner) => inner.$func(),
                Boxed(inner) => inner.$func(),
                Optional(inner) => inner.$func(),
            }
        }
    };
}

impl ApiType {
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F, api_file: &ApiFile) {
        if f(self) {
            return;
        }

        match &self {
            PrimitiveList(inner) => {
                f(&ApiType::Primitive(inner.primitive.clone()));
            }
            GeneralList(inner) => inner.inner.visit_types(f, api_file),
            StructRef(struct_ref) => {
                for field in &struct_ref.get(api_file).fields {
                    field.ty.visit_types(f, api_file);
                }
            }
            Boxed(inner) => inner.inner.visit_types(f, api_file),
            Delegate(d) => d.get_delegate().visit_types(f, api_file),
            Optional(inner) => inner.inner.visit_types(f, api_file),
            Primitive(_) => {}
        }
    }

    api_type_call_child!(safe_ident, String);
    api_type_call_child!(dart_api_type, String);
    api_type_call_child!(dart_wire_type, String);
    api_type_call_child!(rust_api_type, String);
    api_type_call_child!(rust_wire_type, String);
    api_type_call_child!(rust_wire_modifier, String);
    api_type_call_child!(rust_wire_is_pointer, bool);

    #[inline]
    pub fn required_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "",
            _ => "required ",
        }
    }

    // api_fill functions target this type instead of the delegate.
    #[inline]
    pub fn optional_inner(&self) -> &ApiType {
        match self {
            Optional(inner) => &inner.inner,
            _ => self,
        }
    }

    #[inline]
    pub fn optional_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "*mut ",
            _ => "",
        }
    }
}

pub trait ApiTypeChild {
    fn safe_ident(&self) -> String;

    fn dart_api_type(&self) -> String;

    fn dart_wire_type(&self) -> String;

    fn rust_api_type(&self) -> String;

    fn rust_wire_type(&self) -> String;

    fn rust_wire_modifier(&self) -> String {
        if self.rust_wire_is_pointer() {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum ApiTypePrimitive {
    U8,
    I8,
    I32,
    I64,
    F64,
    Bool,
}

impl ApiTypeChild for ApiTypePrimitive {
    fn safe_ident(&self) -> String {
        self.rust_api_type()
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8 => "int",
            ApiTypePrimitive::I8 => "int",
            ApiTypePrimitive::I32 => "int",
            ApiTypePrimitive::I64 => "int",
            ApiTypePrimitive::F64 => "double",
            ApiTypePrimitive::Bool => "bool",
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.dart_api_type()
    }

    fn rust_api_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_wire_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8 => "u8",
            ApiTypePrimitive::I8 => "i8",
            ApiTypePrimitive::I32 => "i32",
            ApiTypePrimitive::I64 => "i64",
            ApiTypePrimitive::F64 => "f64",
            ApiTypePrimitive::Bool => "bool",
        }
        .to_string()
    }
}

impl ApiTypePrimitive {
    /// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
    /// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
    /// whenever primitives are put behind a pointer.
    pub fn dart_native_type(&self) -> &'static str {
        match self {
            ApiTypePrimitive::U8 | ApiTypePrimitive::Bool => "ffi.Uint8",
            ApiTypePrimitive::I8 => "ffi.Int8",
            ApiTypePrimitive::I32 => "ffi.Int32",
            ApiTypePrimitive::I64 => "ffi.Int64",
            ApiTypePrimitive::F64 => "ffi.Double",
        }
    }
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(ApiTypePrimitive::U8),
            "i8" => Some(ApiTypePrimitive::I8),
            "i32" => Some(ApiTypePrimitive::I32),
            "i64" => Some(ApiTypePrimitive::I64),
            "f64" => Some(ApiTypePrimitive::F64),
            "bool" => Some(ApiTypePrimitive::Bool),
            _ => None,
        }
    }
}

// types that delegate to another type
#[derive(Debug, Clone)]
pub enum ApiTypeDelegate {
    String,
    // upstream (allo-isolate) only supports ZeroCopyBuffer for Vec<u8> and Vec<i8>.
    ZeroCopyBufferVecU8,
}

impl ApiTypeDelegate {
    pub fn get_delegate(&self) -> ApiType {
        match self {
            ApiTypeDelegate::String => ApiType::PrimitiveList(ApiTypePrimitiveList {
                primitive: ApiTypePrimitive::U8,
            }),
            ApiTypeDelegate::ZeroCopyBufferVecU8 => ApiType::PrimitiveList(ApiTypePrimitiveList {
                primitive: ApiTypePrimitive::U8,
            }),
        }
    }
}

impl ApiTypeChild for ApiTypeDelegate {
    fn safe_ident(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_string(),
            ApiTypeDelegate::ZeroCopyBufferVecU8 => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_string(),
            ApiTypeDelegate::ZeroCopyBufferVecU8 => self.get_delegate().dart_api_type(),
        }
    }

    fn dart_wire_type(&self) -> String {
        self.get_delegate().dart_wire_type()
    }

    fn rust_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_owned(),
            ApiTypeDelegate::ZeroCopyBufferVecU8 => "ZeroCopyBuffer<Vec<u8>>".to_owned(),
        }
    }

    fn rust_wire_type(&self) -> String {
        self.get_delegate().rust_wire_type()
    }

    fn rust_wire_is_pointer(&self) -> bool {
        self.get_delegate().rust_wire_is_pointer()
    }
}

impl ApiTypeDelegate {
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "String" => Some(ApiTypeDelegate::String),
            "ZeroCopyBuffer<Vec<u8>>" => Some(ApiTypeDelegate::ZeroCopyBufferVecU8),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypePrimitiveList {
    pub primitive: ApiTypePrimitive,
}

impl ApiTypeChild for ApiTypePrimitiveList {
    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        match &self.primitive {
            ApiTypePrimitive::U8 => "Uint8List",
            ApiTypePrimitive::I8 => "Int8List",
            ApiTypePrimitive::I64 => "Int64List",
            ApiTypePrimitive::F64 => "Float64List",
            _ => panic!("does not support {:?} yet", &self.primitive),
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeGeneralList {
    pub inner: ApiType,
}

impl ApiTypeChild for ApiTypeGeneralList {
    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn dart_api_type(&self) -> String {
        format!("List<{}>", self.inner.dart_api_type())
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeStructRef {
    pub name: String,
}

impl ApiTypeStructRef {
    pub fn get<'a>(&self, f: &'a ApiFile) -> &'a ApiStruct {
        &f.struct_pool[&self.name]
    }
}

impl ApiTypeChild for ApiTypeStructRef {
    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct ApiStruct {
    pub name: String,
    pub fields: Vec<ApiField>,
    pub is_fields_named: bool,
}

#[derive(Debug, Clone)]
pub struct ApiField {
    pub ty: ApiType,
    pub name: ApiIdent,
}

impl ApiField {
    pub fn name_rust_style(&self, is_fields_named: bool) -> String {
        if is_fields_named {
            self.name.rust_style().to_string()
        } else {
            // TO DO this is so hacky...
            self.name.rust_style().replace("field", "")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: ApiType,
}

impl ApiTypeChild for ApiTypeBoxed {
    fn safe_ident(&self) -> String {
        format!(
            "box_{}{}",
            if self.exist_in_real_api {
                ""
            } else {
                "autoadd_"
            },
            self.inner.safe_ident()
        )
    }

    fn dart_api_type(&self) -> String {
        self.inner.dart_api_type()
    }

    fn dart_wire_type(&self) -> String {
        let wire_type = if let Primitive(prim) = &self.inner {
            prim.dart_native_type().to_owned()
        } else {
            self.inner.dart_wire_type()
        };
        format!("ffi.Pointer<{}>", wire_type)
    }

    fn rust_api_type(&self) -> String {
        if self.exist_in_real_api {
            format!("Box<{}>", self.inner.rust_api_type())
        } else {
            self.inner.rust_api_type()
        }
    }

    fn rust_wire_type(&self) -> String {
        self.inner.rust_wire_type()
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeOptional {
    pub inner: Box<ApiType>,
}

impl ApiTypeOptional {
    pub fn new_prim(prim: ApiTypePrimitive) -> Self {
        Self {
            inner: Box::new(Boxed(Box::new(ApiTypeBoxed {
                inner: Primitive(prim),
                exist_in_real_api: false,
            }))),
        }
    }

    pub fn new_ptr(ptr: ApiType) -> Self {
        Self {
            inner: Box::new(ptr),
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if matches!(boxed.inner, ApiType::Primitive(_)))
    }

    pub fn is_list(&self) -> bool {
        matches!(&*self.inner, GeneralList(_) | PrimitiveList(_))
    }

    pub fn is_delegate(&self) -> bool {
        matches!(&*self.inner, Delegate(_))
    }

    pub fn needs_initialization(&self) -> bool {
        !(self.is_primitive() || self.is_delegate())
    }
}

impl ApiTypeChild for ApiTypeOptional {
    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }
    fn rust_wire_type(&self) -> String {
        self.inner.rust_wire_type()
    }
    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
    }
    fn dart_wire_type(&self) -> String {
        self.inner.dart_wire_type()
    }
    fn dart_api_type(&self) -> String {
        format!("{}?", self.inner.dart_api_type())
    }
    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}
