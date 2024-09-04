use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JStrRef{
    idx:u16,
}
impl JStrRef{
    pub fn new(d:[u8;2])->JStrRef{
        JStrRef { 
            idx:u16::from_be_bytes(d)
        }
    }
    pub fn get_idx(&self)->u16{
        self.idx
    }
}
impl JBaseType for JStrRef{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}