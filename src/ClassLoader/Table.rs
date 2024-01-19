pub trait NewTable<T>
    where T:Sized
{
    fn newTable(data:Vec<u8>)->T;
}
pub struct Table<T>
    where T: NewTable<T>
{
    table:Box<T>
}
impl<T:NewTable<T>> Table<T>{
    pub fn new(data:Vec<u8>)->Table<T>{
        Table{
            table:Box::new(T::newTable(data))
        }
    }
}