//@ignore-target-windows: No libc on Windows
//@compile-flags: -Zmiri-disable-isolation

// FIXME: standard handles cannot be closed (https://github.com/rust-lang/rust/issues/40032)

#![feature(rustc_private)]

fn main() {
    unsafe {
        libc::close(1); //~ ERROR: cannot close stdout
    }
}
