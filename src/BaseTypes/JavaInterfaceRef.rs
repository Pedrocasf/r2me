use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JInterfaceRef{
    classRefIdx:u16,
    nameTypeDescriptorIdx:u16
}
impl JInterfaceRef{
    pub fn new(d:[u8;4])->JInterfaceRef{
        JInterfaceRef { 
            classRefIdx: u16::from_le_bytes(d[0..2].try_into().unwrap()),
            nameTypeDescriptorIdx: u16::from_le_bytes(d[2..4].try_into().unwrap())
        }
    }
}
impl JBaseType for JInterfaceRef{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}