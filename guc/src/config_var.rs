use crate::Opaque;

#[derive(Debug)]
pub enum ConfigVar {
    BoolVal(bool, /*Option<Opaque>*/),
    IntVal(i32, /*Option<Opaque>*/),
    RealVal(f64, /*Option<Opaque>*/),
    StringVal(String, /*Option<Opaque>*/),
    EnumVal(i32, /*Option<Opaque>*/),
}
