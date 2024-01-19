#[derive(Debug, Clone)]
pub struct InterfacePool{
    size:u16,
    pool:Vec<u16>
}
impl InterfacePool{
    pub fn new(data:&[u8])->(InterfacePool, usize){
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut pool  = Vec::<u16>::with_capacity(size as usize);
        for i in 0..size as usize{
            pool.push(u16::from_be_bytes(data[i*2..(i*2)+2].try_into().unwrap()));
        }
        println!("{:#X?}", size);
        (InterfacePool {
            size: size,
            pool: pool
        },((size as usize+1)*2))

    }
}