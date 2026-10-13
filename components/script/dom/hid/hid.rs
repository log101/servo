use std::collections::HashSet;

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

type HIDDeviceVector = Vec<Root<Dom<HIDDevice>>>;

#[dom_struct]
pub struct HID {
    eventtarget: EventTarget,
    devices: HIDDeviceVector,
}

#[derive(Hash, PartialEq, Eq)]
struct DeviceID {
    vendor_id: u16,
    product_id: u16
}

impl HID {
    fn new_inherited(devices: HIDDeviceVector) -> Self {
        Self {
            eventtarget: EventTarget::new_inherited(),
            devices,
        }
    }

    pub(crate) fn new(cx: &mut JSContext, global: &GlobalScope) -> DomRoot<HID> {
        let mut hid_devices = Vec::new();
        let mut devices: HashSet<DeviceID> = HashSet::new();


        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    let device_id = DeviceID {
                        vendor_id: device.vendor_id(),
                        product_id: device.product_id()
                    };

                    if !devices.contains(&device_id) {
                        let hid_device = HIDDevice::new(
                            cx,
                            global,
                            device.vendor_id(),
                            device.product_id(),
                            DOMString::from(
                                device.product_string().unwrap_or("default device string"),
                            ),
                        );
                        devices.insert(device_id);

                        hid_devices.push(hid_device);
                    }
                }
            },
            Err(_e) => {
                log::info!("Error accesing HID devices!");
            },
        }

        log::info!("Accessed HID devices {}", hid_devices.len());

        reflect_dom_object(cx, Box::new(HID::new_inherited(hid_devices)), global)
    }
}

impl HIDMethods<crate::DomTypeHolder> for HID {
    event_handler!(connect, GetOnconnect, SetOnconnect);
    event_handler!(disconnect, GetOndisconnect, SetOndisconnect);

    fn GetDevices(&self, cx: &mut JSContext) -> RootedPromise {
        let promise = Promise::new(cx, &self.global());

        promise.resolve_native(cx, &self.devices.clone());

        promise
    }
}
