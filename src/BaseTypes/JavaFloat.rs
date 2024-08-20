use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, PartialEq)]
pub struct JFloat{
    data:f32
}
impl JFloat{
    pub fn new(d:[u8;4])->JFloat{
        JFloat{
            data:f32::from_be_bytes(d)
        }
    }
}
impl JBaseType for JFloat{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}