use dom_struct::dom_struct;
use script_bindings::reflector::Reflector;
use crate::dom::bindings::codegen::Bindings::HIDBinding::HIDMethods;

#[dom_struct]
pub struct HID {
    reflector_: Reflector,
}

impl HIDMethods<crate::DomTypeHolder> for HID {
    fn GetAvailability(&self) -> bool { todo!() }
}