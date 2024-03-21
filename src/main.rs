#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{AnyPin, Input, Pin, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::Builder;
use heapless::String;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

static WRITE: Signal<ThreadModeRawMutex, bool> = Signal::new();
static MESSAGE: Mutex<ThreadModeRawMutex, String<256>> = Mutex::new(String::new());

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let driver = Driver::new(peripherals.USB, Irqs);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Fx137");
    config.product = Some("Wiggle wiggle wiggle");
    config.serial_number = Some("13371337");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let request_handler = MyRequestHandler {};

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [],
        &mut control_buf,
    );

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: Some(&request_handler),
        poll_ms: 60,
        max_packet_size: 8,
    };

    unwrap!(spawner.spawn(io_task(peripherals.PIN_13.degrade())));

    let writer_reader = HidReaderWriter::<_, 20, 20>::new(&mut builder, &mut state, config);

    let (reader, mut writer) = writer_reader.split();

    let mut usb = builder.build();

    let usb_future = usb.run();

    let hid_in_future = async {
        loop {
            WRITE.wait().await;

            let asdf = MESSAGE.lock().await;

            for char in asdf.as_bytes() {
                let report = map_key(&char);

                match writer.write_serialize(&report).await {
                    Ok(()) => {}
                    Err(error) => {
                        info!("{}", error);
                    }
                }

                let report = KeyboardReport {
                    keycodes: [0, 0, 0, 0, 0, 0],
                    leds: 0,
                    modifier: 0,
                    reserved: 0,
                };

                match writer.write_serialize(&report).await {
                    Ok(()) => {}
                    Err(error) => {
                        info!("{}", error);
                    }
                }
            }
        }
    };

    let hid_out_future = async {
        reader.run(false, &request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join3(usb_future, hid_in_future, hid_out_future).await;
}

#[embassy_executor::task]
async fn io_task(button_pin: AnyPin) {
    let mut button = Input::new(button_pin, Pull::Up);

    loop {
        button.wait_for_falling_edge().await;

        info!("Button Pressed");
        WRITE.signal(true);


        Timer::after_millis(10).await;
        button.wait_for_high().await;
    }
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);

        // let mut asdf = MESSAGE.lock().await;
        // *asdf = String::try_from("value").unwrap_or(String::new());
        OutResponse::Accepted
    }

    fn set_idle_ms(&self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

fn map_key(key: &u8) -> KeyboardReport {
    let key = match key {
        b'a' => 0x04,
        b'b' => 0x05,
        b'c' => 0x06,
        b'd' => 0x07,
        b'e' => 0x08,
        b'f' => 0x09,
        b'g' => 0x0A,
        b'h' => 0x0B,
        b'i' => 0x0C,
        b'j' => 0x0D,
        b'k' => 0x0E,
        b'l' => 0x0F,
        b'm' => 0x10,
        b'n' => 0x11,
        b'o' => 0x12,
        b'p' => 0x13,
        b'q' => 0x14,
        b'r' => 0x15,
        b's' => 0x16,
        b't' => 0x17,
        b'u' => 0x18,
        b'v' => 0x19,
        b'w' => 0x1A,
        b'x' => 0x1B,
        b'y' => 0x1C,
        b'z' => 0x1D,
        b'1' => 0x1E,
        b'2' => 0x1F,
        b'3' => 0x20,
        b'4' => 0x21,
        b'5' => 0x22,
        b'6' => 0x23,
        b'7' => 0x24,
        b'8' => 0x25,
        b'9' => 0x26,
        b'0' => 0x27,
        _ => 0,
    };

    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [key, 0, 0, 0, 0, 0],
    }
}
