use std::ffi::CString;

use dom_struct::dom_struct;
use js::context::JSContext;
use tendril::fmt::Slice;
use crate::dom::TracedPromise;
use crate::dom::bindings::error::Error;
use crate::dom::bindings::reflector::DomGlobal;
use script_bindings::codegen::GenericBindings::HIDBinding::HIDDeviceMethods;
use script_bindings::codegen::GenericUnionTypes::ArrayBufferViewOrArrayBuffer;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;
use script_bindings::cell::RefCell;
use crate::dom::bindings::buffer_source::get_buffer_source_copy;
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
    state: RefCell<String>,
    pending_send_report_promises: RefCell<Vec<TracedPromise>>
}

impl HIDDevice {
    fn new_inherited(vendor_id: u16, product_id: u16, product_name: DOMString) -> HIDDevice {
        Self {
            event_target: EventTarget::new_inherited(),
            opened: false,
            vendor_id,
            product_id,
            product_name,
            state: RefCell::new(String::from("closed")),
            pending_send_report_promises: RefCell::new(Vec::new()),
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

        *self.state.borrow_mut() = String::from("opening");

        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    if device.vendor_id() == self.vendor_id && device.product_id() == self.product_id {
                        let res = api.open(device.vendor_id(), device.product_id());
                        match res {
                            Ok(_) => {
                                log::info!("Opened connection to device");

                                *self.state.borrow_mut() = String::from("opened");

                                promise.resolve_native(cx, &())
                            },
                            Err(err) => {
                                log::info!("Could not open connection {}", err);

                                promise.reject_error(cx, Error::Network(None));
                            },
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

    // https://hid.spec.whatwg.org/#dom-hiddevice-sendreport
    fn SendReport(&self, cx: &mut JSContext, report_id: u8, data: ArrayBufferViewOrArrayBuffer) -> RootedPromise {
        let promise = Promise::new(cx, &self.global());

        if self.state.borrow().clone() != "opened" {
            promise.reject_error(cx, Error::InvalidState(None));
        }

        if report_id == 0 {
            promise.reject_error(cx, Error::Type(CString::new("").unwrap()));
        }

        self.pending_send_report_promises.borrow_mut().push(promise.to_traced());

        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    if device.vendor_id() == self.vendor_id && device.product_id() == self.product_id {
                        let res = api.open(device.vendor_id(), device.product_id());
                        match res {
                            Ok(hid_device) => {
                                log::info!("OPENED CONNECTION");

                                let mut report_bytes = vec![report_id];

                                log::info!("Sending with report id {}", report_id);

                                let mut report_data = get_buffer_source_copy((&data).into());

                                log::info!("Sending with data {:?}", report_data);

                                report_bytes.append(&mut report_data);

                                let report_result = hid_device.send_output_report(report_bytes.as_bytes());

                                match report_result {
                                    Ok(_) => {
                                        promise.resolve_native(cx, &())
                                    },
                                    Err(err) => {
                                        log::info!("Could not send output report {}", err);

                                        promise.reject_error(cx, Error::Network(None));
                                    }
                                }

                                promise.resolve_native(cx, &())
                            },
                            Err(err) => {
                                log::info!("Could not open connection {}", err);

                                promise.reject_error(cx, Error::Network(None));
                            },
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
