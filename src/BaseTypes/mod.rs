use std::fmt::Debug;

pub mod JString;
pub mod JInteger;
pub mod JFloat;
pub mod JLong;
pub mod JDouble;
pub mod JClassRef;
pub mod JStrRef;
pub mod JFieldRef;
pub mod JMethodRef;
pub mod JInterfaceRef;
pub mod JNameTypeDescriptor;
pub trait JBaseType:Debug + JBaseTypeClone{}
pub trait JBaseTypeClone{
    fn clone_box(&self)-> Box<dyn JBaseType>;
}
impl<T> JBaseTypeClone for T
where 
    T: 'static + JBaseType + Clone{
        fn clone_box(&self)-> Box<dyn JBaseType> {
            Box::new(self.clone())
        }
}
impl Clone for Box<dyn JBaseType>{
    fn clone(&self) -> Box<dyn JBaseType>{
        self.clone_box()
    }
}
/*
String=1,
    Integer=3,
    Float=4,
    Long=5,
    Double=6,
    ClassRef=7,
    StrRef=8,
    FieldRef=9,
    MethodRef=10,
    InterfaceRef=11,
    NameTypeDescriptor=12,
     */
