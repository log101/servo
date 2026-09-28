use dom_struct::dom_struct;
use hidapi::HidApi;
use js::context::JSContext;
use log::warn;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;

use crate::dom::RootedPromise;
use crate::dom::bindings::codegen::Bindings::HIDBinding::{HIDDevice, HIDMethods};
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HID {
    event_target: EventTarget,
    devices: Vec<HIDDevice>,
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

    fn GetDevices(&self) -> RootedPromise {
        log::info!("HERE!!!!!!!!!!!");

        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    log::info!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
                }

                todo!()
            },
            Err(e) => {
                log::info!("Error accesing HID devices!");

                todo!()
            },
        }

        todo!()
    }
}
