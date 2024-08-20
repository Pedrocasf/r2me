use core::fmt::Debug;
use std::any::Any;

pub mod JavaString;
pub use JavaString::JString;
pub mod JavaInteger;
pub use JavaInteger::JInteger;
pub mod JavaFloat;
pub use JavaFloat::JFloat;
pub mod JavaLong;
pub use JavaLong::JLong;
pub mod JavaDouble;
pub use JavaDouble::JDouble;
pub mod JavaClassRef;
pub use JavaClassRef::JClassRef;
pub mod JavaStrRef;
pub use JavaStrRef::JStrRef;
pub mod JavaFieldRef;
pub use JavaFieldRef::JFieldRef;
pub mod JavaMethodRef;
pub use JavaMethodRef::JMethodRef;
pub mod JavaInterfaceRef;
pub use JavaInterfaceRef::JInterfaceRef;
pub mod JavaNameTypeDescriptor;
pub use JavaNameTypeDescriptor::JNameTypeDescriptor;
pub trait JBaseType:Debug + JBaseTypeClone{
    fn as_any(&mut self) -> &mut dyn Any;
}
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
