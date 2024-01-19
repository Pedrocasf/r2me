use super::JBaseType;
#[derive(Debug, Clone)]
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
impl JBaseType for JDouble{}