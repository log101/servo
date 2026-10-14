use dom_struct::dom_struct;
use js::context::JSContext;
use crate::dom::bindings::error::Error;
use crate::dom::bindings::reflector::DomGlobal;
use script_bindings::codegen::GenericBindings::HIDBinding::HIDDeviceMethods;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;
use script_bindings::cell::DomRefCell;
use hidapi::HidApi;

use crate::dom::promise::{Promise, RootedPromise};
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HIDDevice {
    event_target: EventTarget,
    opened: bool,
    vendor_id: u16,
    product_id: u16,
    product_name: DOMString,
    state: DomRefCell<String>
}

impl HIDDevice {
    fn new_inherited(vendor_id: u16, product_id: u16, product_name: DOMString) -> HIDDevice {
        Self {
            event_target: EventTarget::new_inherited(),
            opened: false,
            vendor_id,
            product_id,
            product_name,
            state: DomRefCell::new(String::from("closed"))
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

    /// https://hid.spec.whatwg.org/#open-method
    fn Open(&self, cx: &mut JSContext) -> RootedPromise {
        let promise = Promise::new(cx, &self.global());

        if self.state.borrow().clone() != "closed" {
            promise.reject_error(cx, Error::InvalidState(None));
        }

        "opening".clone_into(&mut self.state.borrow_mut());

        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    if device.vendor_id() == self.vendor_id && device.product_id() == self.product_id {
                        if let Ok(val) = api.open(device.vendor_id(), device.product_id()) {
                            log::info!("OPENED CONNECTION");
                            promise.resolve_native(cx, &())
                        }
                    }
                }
            },
            Err(_e) => {
                log::info!("Error accesing HID devices!");
                promise.reject_error(cx, Error::InvalidState(None))
            },
        }

        promise

    }
}
