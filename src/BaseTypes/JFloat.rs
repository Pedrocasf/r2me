use super::JBaseType;
#[derive(Debug, Clone)]
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
impl JBaseType for JFloat{}