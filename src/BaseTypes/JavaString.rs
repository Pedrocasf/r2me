use std::any::Any;
use super::JBaseType;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JString{
    size:u16,
    string:String
}
impl JString{
    pub fn new(d:&[u8])->JString{
        let sz = u16::from_be_bytes(d[0..2].try_into().expect("Not enough bytes"));
        //let string:Vec<u16> = d.to_vec().chunks_exact(2).into_iter().map(|a| u16::from_le_bytes([a[0], a[1]])).collect();
        JString{
            size:sz,
            string:String::from_utf8(d[2..2+sz as usize].to_vec()).unwrap()
        }
    }
}
impl JBaseType for JString{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}