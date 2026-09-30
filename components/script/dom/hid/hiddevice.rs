use dom_struct::dom_struct;
use js::context::JSContext;
use script_bindings::codegen::GenericBindings::HIDBinding::HIDDeviceMethods;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;

use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HIDDevice {
    event_target: EventTarget,
    opened: bool,
    vendor_id: u16,
    product_id: u16,
    product_name: DOMString,
}

impl HIDDevice {
    fn new_inherited(vendor_id: u16, product_id: u16, product_name: DOMString) -> HIDDevice {
        Self {
            event_target: EventTarget::new_inherited(),
            opened: false,
            vendor_id,
            product_id,
            product_name,
        }
    }

    pub(crate) fn new(
        cx: &mut JSContext,
        global: &GlobalScope,
        vendor_id: u16,
        product_id: u16,
        product_name: DOMString,
    ) -> DomRoot<HIDDevice> {
        reflect_dom_object(
            cx,
            Box::new(HIDDevice::new_inherited(
                vendor_id,
                product_id,
                product_name,
            )),
            global,
        )
    }
}

impl HIDDeviceMethods<crate::DomTypeHolder> for HIDDevice {
    fn Opened(&self) -> bool {
        self.opened
    }

    fn VendorId(&self) -> u16 {
        self.vendor_id
    }

    fn ProductId(&self) -> u16 {
        self.product_id
    }

    fn ProductName(&self) -> DOMString {
        self.product_name.clone()
    }
}
