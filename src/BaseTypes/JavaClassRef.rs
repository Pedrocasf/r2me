use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JClassRef{
    idx:u16,
}
impl JClassRef{
    pub fn new(d:[u8;2])->JClassRef{
        JClassRef { 
            idx:u16::from_be_bytes(d)
        }
    }
}
impl JBaseType for JClassRef{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}