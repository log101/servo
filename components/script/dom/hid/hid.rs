use dom_struct::dom_struct;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;
use crate::dom::eventtarget::EventTarget;
use crate::dom::bindings::codegen::Bindings::HIDBinding::HIDMethods;
use js::context::JSContext;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HID {
    event_target: EventTarget,
}

impl HID {
    fn new_inherited() -> HID {
        Self {
            event_target: EventTarget::new_inherited(),
        }
    }

    pub(crate) fn new(cx: &mut JSContext, global: &GlobalScope) -> DomRoot<HID> {
        reflect_dom_object(cx, Box::new(HID::new_inherited()), global)
    }
}

impl HIDMethods<crate::DomTypeHolder> for HID {
    event_handler!(connect, GetOnconnect, SetOnconnect);
    event_handler!(disconnect, GetOndisconnect, SetOndisconnect);
}