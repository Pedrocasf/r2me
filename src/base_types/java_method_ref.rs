use super::JBaseType;
use std::any::Any;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JMethodRef {
    class_ref_idx: u16,
    name_type_descriptor_idx: u16,
}
impl JMethodRef {
    pub fn new(d: [u8; 4]) -> JMethodRef {
        JMethodRef {
            class_ref_idx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            name_type_descriptor_idx: u16::from_be_bytes(d[2..4].try_into().unwrap()),
        }
    }
}
impl JBaseType for JMethodRef {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
