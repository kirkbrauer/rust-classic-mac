//! Minimal Rust runtime for PowerPC Classic Mac OS (System 7-9)
//!
//! This crate provides the entry point and panic handler so user programs
//! can define `main()` without boilerplate.
//!
//! # Usage
//!
//! ```rust,ignore
//! #![no_std]
//! #![no_main]
//!
//! extern crate classic_macos_rt;
//!
//! extern "C" {
//!     fn SysBeep(duration: i16);
//! }
//!
//! #[no_mangle]
//! pub fn macos_main() {
//!     unsafe { SysBeep(30); }
//! }
//! ```

#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

// Mac OS Toolbox imports from InterfaceLib
extern "C" {
    fn ExitToShell() -> !;
}

// User's entry function (void return - Classic Mac OS ignores exit codes)
// Named `macos_main` to avoid conflict with Rust's special `main` function
extern "Rust" {
    fn macos_main();
}

/// Entry point called by Code Fragment Manager (CFM)
///
/// CFM has already:
/// - Initialized TOC (r2) register
/// - Performed relocations
/// - Set up .data/.bss sections
#[no_mangle]
pub extern "C" fn __start() -> ! {
    unsafe {
        macos_main();
        ExitToShell()
    }
}

/// Panic handler - required for no_std crates
///
/// On panic, we simply exit to shell. A future version could
/// display an error dialog or beep to indicate the error.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { ExitToShell() }
}

/// Required lang item for exception handling personality
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
