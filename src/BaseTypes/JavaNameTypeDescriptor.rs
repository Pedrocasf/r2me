use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JNameTypeDescriptor{
    nameIdx:u16,
    descriptorIdx:u16
}
impl JNameTypeDescriptor{
    pub fn new(d:[u8;4])->JNameTypeDescriptor{
        JNameTypeDescriptor { 
            nameIdx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            descriptorIdx: u16::from_be_bytes(d[2..4].try_into().unwrap())
        }
    }
}
impl JBaseType for JNameTypeDescriptor{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}