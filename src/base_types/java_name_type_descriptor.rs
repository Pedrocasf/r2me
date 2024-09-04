use super::JBaseType;
use std::any::Any;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JNameTypeDescriptor {
    name_idx: u16,
    descriptor_idx: u16,
}
impl JNameTypeDescriptor {
    pub fn new(d: [u8; 4]) -> JNameTypeDescriptor {
        JNameTypeDescriptor {
            name_idx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            descriptor_idx: u16::from_be_bytes(d[2..4].try_into().unwrap()),
        }
    }
}
impl JBaseType for JNameTypeDescriptor {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
