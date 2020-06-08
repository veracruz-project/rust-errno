//! Implementation for OP-TEE.
//!
//! Code from `unix.rs` used as a base starting point.
//!
//! This is a dummy implementation: the Rust TrustZone SDK libc does not
//! provide any mechanisms for manipulating errno.  We emulate it here with
//! a global variable stored in a Mutex.

use std::sync::Mutex;

use Errno;

lazy_static! {
    static ref ERR_NO: Mutex<Errno> = Mutex::new(Errno(0));
}

pub const STRERROR_NAME: &'static str = "strerror_r";

pub fn with_description<F, T>(_err: Errno, callback: F) -> T where
    F: FnOnce(Result<&str, Errno>) -> T
{
    callback(Ok("dummy description"))
}

pub fn errno() -> Errno {
    *(ERR_NO.lock().expect("Failed to obtain lock on errno."))    
}

pub fn set_errno(errno: Errno) {
    let mut err = ERR_NO.lock().expect("Failed to obtain lock on errno.");
    *err = errno;
}
