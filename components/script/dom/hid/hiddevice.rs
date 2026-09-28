use dom_struct::dom_struct;
use js::context::JSContext;
use script_bindings::codegen::GenericBindings::HIDBinding::HIDDeviceMethods;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;

use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HIDDevice {
    event_target: EventTarget,
}

impl HIDDevice {
    fn new_inherited() -> HIDDevice {
        Self {
            event_target: EventTarget::new_inherited(),
        }
    }

    pub(crate) fn new(cx: &mut JSContext, global: &GlobalScope) -> DomRoot<HIDDevice> {
        reflect_dom_object(cx, Box::new(HIDDevice::new_inherited()), global)
    }
}

impl HIDDeviceMethods<crate::DomTypeHolder> for HIDDevice {
    fn Opened(&self) -> bool {
        todo!()
    }

    fn VendorId(&self) -> u16 {
        todo!()
    }

    fn ProductId(&self) -> u16 {
        todo!()
    }

    fn ProductName(&self) -> script_bindings::str::DOMString {
        todo!()
    }
}
