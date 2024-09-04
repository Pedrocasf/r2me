use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JInteger{
    data:u32
}
impl JInteger{
    pub fn new(d:[u8;4])->JInteger{
        JInteger{
            data:u32::from_be_bytes(d)
        }
    }
}
impl JBaseType for JInteger{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}