use uefi::prelude::*;

use uefi::proto::loaded_image::LoadedImage;
use uefi::{proto, Identify};

pub fn test(image: Handle, st: &mut SystemTable<Boot>) {
    info!("Testing various protocols");

    console::test(image, st);

    let bt = st.boot_services();
    find_protocol(bt);
    test_protocols_per_handle(image, bt);

    debug::test(bt);
    device_path::test(image, bt);
    driver::test(bt);
    loaded_image::test(image, bt);
    media::test(bt);
    network::test(bt);
    pi::test(bt);
    rng::test(bt);
    string::test(bt);

    #[cfg(any(
        target_arch = "i386",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64"
    ))]
    shim::test(bt);
    tcg::test(bt);
}

fn find_protocol(bt: &BootServices) {
    let handles = bt
        .find_handles::<proto::console::text::Output>()
        .expect("Failed to retrieve list of handles");

    assert!(
        !handles.is_empty(),
        "There should be at least one implementation of Simple Text Output (stdout)"
    );
}

fn test_protocols_per_handle(image: Handle, bt: &BootServices) {
    let pph = bt
        .protocols_per_handle(image)
        .expect("Failed to get protocols for image handle");

    info!("Image handle has {} protocols", pph.len());

    // Check that one of the image's protocols is `LoadedImage`.
    assert!(pph.iter().any(|guid| **guid == LoadedImage::GUID));
}

mod console;
mod debug;
mod device_path;
mod driver;
mod loaded_image;
mod media;
mod network;
mod pi;
mod rng;
#[cfg(any(
    target_arch = "i386",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
mod shim;
mod string;
mod tcg;
