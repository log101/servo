
// https://hid.spec.whatwg.org/#extensions-to-the-navigator-interface
partial interface Navigator {
    [SameObject] readonly attribute HID hid;
};

[Exposed=(DedicatedWorker,ServiceWorker,Window), SecureContext]
interface HID : EventTarget {
    attribute EventHandler onconnect;
    attribute EventHandler ondisconnect;
    Promise<sequence<HIDDevice>> getDevices();
    // [Exposed=Window] Promise<sequence<HIDDevice>> requestDevice(
    //   HIDDeviceRequestOptions options);
};

[Exposed=(DedicatedWorker,ServiceWorker,Window), SecureContext]
interface HIDDevice : EventTarget {
    // attribute EventHandler oninputreport;
    readonly attribute boolean opened;
    readonly attribute unsigned short vendorId;
    readonly attribute unsigned short productId;
    readonly attribute DOMString productName;
    // readonly attribute FrozenArray<HIDCollectionInfo> collections;
    Promise<undefined> open();
    // Promise<undefined> close();
    // Promise<undefined> forget();
    // Promise<undefined> sendReport([EnforceRange] octet reportId, BufferSource data);
    // Promise<undefined> sendFeatureReport(
    //     [EnforceRange] octet reportId,
    //     BufferSource data);
    // Promise<DataView> receiveFeatureReport([EnforceRange] octet reportId);
};