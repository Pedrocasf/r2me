use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, PartialEq)]
pub struct JDouble{
    data:f64
}
impl JDouble{
    pub fn new(d:[u8;8])->JDouble{
        JDouble{
            data:f64::from_be_bytes(d)
        }
    }
}
impl JBaseType for JDouble{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}