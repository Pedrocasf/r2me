use crate::BaseTypes::JavaStrRef::JStrRef;
use crate::Class;
pub struct VirtualMachine{
    main_class:Class
}
impl VirtualMachine{
    pub fn new(main_class:Class) -> VirtualMachine{
        VirtualMachine{
            main_class
        }
    }
}