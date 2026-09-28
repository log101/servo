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
        HID {
            event_target: EventTarget::new_inherited(),
        }
    }

    pub(crate) fn new(cx: &mut JSContext, global: &GlobalScope) -> DomRoot<HID> {
        reflect_dom_object(cx, Box::new(HID::new_inherited()), global)
    }
}

impl HIDMethods<crate::DomTypeHolder> for HID {
    fn GetOnconnect(&self, r#cx: &mut JSContext) -> Option<script_bindings::callback::RootedCallback<script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull<crate::DomTypeHolder>>> {
        todo!()
    }

    fn SetOnconnect(&self, r#cx: &mut JSContext, r#value: Option<script_bindings::callback::RootedCallback<script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull<crate::DomTypeHolder>>>) {
        todo!()
    }

    fn GetOndisconnect(&self, r#cx: &mut JSContext) -> Option<script_bindings::callback::RootedCallback<script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull<crate::DomTypeHolder>>> {
        todo!()
    }

    fn SetOndisconnect(&self, r#cx: &mut JSContext, r#value: Option<script_bindings::callback::RootedCallback<script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull<crate::DomTypeHolder>>>) {
        todo!()
    }
}