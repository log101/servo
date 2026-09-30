use std::borrow::BorrowMut;

use dom_struct::dom_struct;
use hidapi::HidApi;
use js::context::JSContext;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::{Dom, DomRoot, Root};
use script_bindings::str::DOMString;

use crate::dom::bindings::codegen::Bindings::HIDBinding::{HIDDevice, HIDMethods};
use crate::dom::bindings::reflector::DomGlobal;
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::{Promise, RootedPromise};

#[dom_struct]
pub struct HID {
    event_target: EventTarget,
    devices: Vec<Root<Dom<HIDDevice>>>,
}

impl HID {
    fn new_inherited() -> HID {
        Self {
            event_target: EventTarget::new_inherited(),
            devices: Vec::new(),
        }
    }

    pub(crate) fn new(cx: &mut JSContext, global: &GlobalScope) -> DomRoot<HID> {
        reflect_dom_object(cx, Box::new(HID::new_inherited()), global)
    }
}

impl HIDMethods<crate::DomTypeHolder> for HID {
    event_handler!(connect, GetOnconnect, SetOnconnect);
    event_handler!(disconnect, GetOndisconnect, SetOndisconnect);

    fn GetDevices(&self, cx: &mut JSContext) -> RootedPromise {
        match HidApi::new() {
            Ok(api) => {
                let mut jeffsdevices = Vec::new();

                for device in api.device_list() {
                    log::info!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
                    let device = HIDDevice::new(
                        cx,
                        &self.global(),
                        device.vendor_id(),
                        device.product_id(),
                        DOMString::from(device.product_string().unwrap_or("default device string")),
                    );

                    jeffsdevices.push(device);
                }

                let promise = Promise::new(cx, &self.global());

                promise.resolve_native(cx, &jeffsdevices);
                return promise;
            },
            Err(e) => {
                log::info!("Error accesing HID devices!");
                let promise = Promise::new(cx, &self.global());

                promise.resolve_native(cx, &self.devices);
                return promise;
            },
        }
    }
}
