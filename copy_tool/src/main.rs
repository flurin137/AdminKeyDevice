fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let mut correct_device = api
        .device_list()
        .filter(|d| d.manufacturer_string() == Some("Fx137"));

    if let Some(device) = correct_device.next() {
        print!("{:?}", device);

        let vendor_id = 49374;
        let product_id = 51966;

        let device = api.open(vendor_id, product_id).unwrap();
        let buf = [0u8, 1, 2, 3, 4];
        //let res = device.write(&buf).unwrap();
        //println!("Wrote: {:?} byte(s)", res);

        device.write(&[0x11, 0xff, 0x05, 0x1c, 0]).unwrap();
    }

    println!("Hello, world!");
}
