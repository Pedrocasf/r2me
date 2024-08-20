use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JFieldRef{
    classRefIdx:u16,
    nameTypeDescriptorIdx:u16
}
impl JFieldRef{
    pub fn new(d:[u8;4])->JFieldRef{
        JFieldRef { 
            classRefIdx: u16::from_be_bytes(d[0..2].try_into().unwrap()),
            nameTypeDescriptorIdx: u16::from_be_bytes(d[2..4].try_into().unwrap())
        }
    }
}
impl JBaseType for JFieldRef{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}