
// https://hid.spec.whatwg.org/#extensions-to-the-navigator-interface
partial interface Navigator {
    [SameObject] readonly attribute HID hid;
};

[Exposed=(DedicatedWorker,ServiceWorker,Window), SecureContext]
interface HID {
    // attribute EventHandler onconnect;
    // attribute EventHandler ondisconnect;
    //Promise<sequence<HIDDevice>> getDevices();
    // [Exposed=Window] Promise<sequence<HIDDevice>> requestDevice(
    //   HIDDeviceRequestOptions options);
    boolean getAvailability();
};