#![no_std]
#![feature(naked_functions)]

extern crate alloc;
#[macro_use]
extern crate log;

mod context_frame;
#[macro_use]
mod exception_utils;
mod pcpu;
mod sync;
mod vcpu;

use spin::once::Once;

use axerrno::AxResult;
use axhal::arch::register_lower_aarch64_synchronous_handler;

pub use self::pcpu::Aarch64PerCpu;
pub use self::vcpu::Aarch64VCpu;
pub use vcpu::AxArchVCpuConfig;

/// context frame for aarch64
pub type ContextFrame = context_frame::Aarch64ContextFrame;

pub fn has_hardware_support() -> bool {
    true
}

static INIT: Once = Once::new();

pub fn do_register_lower_aarch64_synchronous_handler() -> AxResult {
    unsafe {
        INIT.call_once(|| {
            register_lower_aarch64_synchronous_handler(self::vcpu::vmexit_aarch64_handler)
        });
    }
    return Ok(());
}

pub fn do_register_lower_aarch64_irq_handler() -> AxResult {
    // TODO
    Ok(())
}
