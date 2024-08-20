
use super::MethodInfo::MethodInfo;
#[derive(Debug, Clone)]
pub struct MethodPool{
    size:u16,
    methods:Vec<MethodInfo>
}
impl MethodPool{
    pub fn new(data:&[u8])->(MethodPool,usize){
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut methods = Vec::<MethodInfo>::with_capacity(size as usize);
        let mut acc = 2;
        for _ in 0..size as usize{
            let(info,size)= MethodInfo::new(&data[acc..]);
            acc += size;
            methods.push(info);
        }
        (MethodPool{
            size:size,
            methods:methods
        },acc)
    }
}