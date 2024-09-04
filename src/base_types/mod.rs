use core::fmt::Debug;
use std::any::Any;

pub mod java_string;
pub use java_string::JString;
pub mod java_integer;
pub use java_integer::JInteger;
pub mod java_float;
pub use java_float::JFloat;
pub mod java_long;
pub use java_long::JLong;
pub mod java_double;
pub use java_double::JDouble;
pub mod java_class_ref;
pub use java_class_ref::JClassRef;
pub mod java_str_ref;
pub use java_str_ref::JStrRef;
pub mod java_field_ref;
pub use java_field_ref::JFieldRef;
pub mod java_method_ref;
pub use java_method_ref::JMethodRef;
pub mod java_interface_ref;
pub use java_interface_ref::JInterfaceRef;
pub mod java_name_type_descriptor;
pub use java_name_type_descriptor::JNameTypeDescriptor;
pub trait JBaseType: Debug + JBaseTypeClone {
    fn as_any(&mut self) -> &mut dyn Any;
}
pub trait JBaseTypeClone {
    fn clone_box(&self) -> Box<dyn JBaseType>;
}
impl<T> JBaseTypeClone for T
where
    T: 'static + JBaseType + Clone,
{
    fn clone_box(&self) -> Box<dyn JBaseType> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn JBaseType> {
    fn clone(&self) -> Box<dyn JBaseType> {
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
