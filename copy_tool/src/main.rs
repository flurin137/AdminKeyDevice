mod device;
mod serial_wrapper;

use crate::device::AdminKey;

fn main() -> Result<(), String> {
    let device = AdminKey::connect()?;
    device.write("FUCK YOU")
}
