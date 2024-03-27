#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]

mod flash_wrapper;

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join4;
use embassy_rp::bind_interrupts;
use embassy_rp::flash::{Async, Flash};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::hid::{HidWriter, ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use flash_wrapper::{print_error, FlashWrapper};
use heapless::String;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

static WRITE: Signal<ThreadModeRawMutex, bool> = Signal::new();
static MESSAGE: Mutex<ThreadModeRawMutex, String<256>> = Mutex::new(String::new());

const VENDOR_ID: u16 = 0x72F3;
const PRODUCT_ID: u16 = 0x1337;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let driver = Driver::new(peripherals.USB, Irqs);

    let flash = Flash::<_, Async, FLASH_SIZE>::new(peripherals.FLASH, peripherals.DMA_CH0);
    let mut wrapper = FlashWrapper::new(flash);

    let buffer = [0; 64];

    if let Err(error) = wrapper.write_bytes(&buffer) {
        print_error(error)
    };

    match wrapper.read().await {
        Ok(asdf) => {
            info!("asdf: {:?}", asdf)
        }
        Err(error) => print_error(error),
    }

    let mut config = embassy_usb::Config::new(VENDOR_ID, PRODUCT_ID);
    config.manufacturer = Some("Fx137");
    config.product = Some("Wiggle wiggle wiggle");
    config.serial_number = Some("13371337");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let request_handler = MyRequestHandler {};

    let mut state = embassy_usb::class::hid::State::new();
    let mut cdc_state = embassy_usb::class::cdc_acm::State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [],
        &mut control_buf,
    );

    let mut cdc_class = CdcAcmClass::new(&mut builder, &mut cdc_state, 64);

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: Some(&request_handler),
        poll_ms: 60,
        max_packet_size: 8,
    };

    let button_pin = peripherals.PIN_13;

    let mut writer = HidWriter::<_, 20>::new(&mut builder, &mut state, config);

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

    let io_future = async {
        let mut button = Input::new(button_pin, Pull::Up);

        loop {
            button.wait_for_falling_edge().await;

            info!("Button Pressed");
            WRITE.signal(true);

            Timer::after_millis(50).await;
            button.wait_for_high().await;
        }
    };

    let echo_future = async {
        loop {
            cdc_class.wait_connection().await;
            info!("Connected");
            let _ = listen_and_cho(&mut cdc_class).await;
            info!("Disconnected");
        }
    };

    join4(usb_future, hid_in_future, io_future, echo_future).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn listen_and_cho<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("Received data: {:x}", data);

        if data == [0x57, 0x68, 0x61, 0x61, 0x61, 0x74] {
            let response = [0x46, 0x75, 0x63, 0x6b, 0x20, 0x59, 0x4f, 0x55];

            info!("Match :-D");
            class.write_packet(&response).await?;
        } else {
        }
    }
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&self, _id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        None
    }
    fn set_report(&self, _id: ReportId, _data: &[u8]) -> OutResponse {
        OutResponse::Accepted
    }
    fn set_idle_ms(&self, _id: Option<ReportId>, _dur: u32) {}
    fn get_idle_ms(&self, _id: Option<ReportId>) -> Option<u32> {
        None
    }
}

fn map_key(key: &u8) -> KeyboardReport {
    let modifier = match key.is_ascii_uppercase() {
        true => 0x02,
        false => 0,
    };

    let key = match key.to_ascii_lowercase() {
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
        modifier,
        reserved: 0,
        leds: 0,
        keycodes: [key, 0, 0, 0, 0, 0],
    }
}
