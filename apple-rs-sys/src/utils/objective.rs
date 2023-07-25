#[cfg(feature = "objc2")]
pub use objc2::{msg_send, sel};

#[cfg(feature = "objc")]
pub use objc::*;

#[cfg(feature = "objc")]
pub use objc_id::*;

#[cfg(feature = "objc")]
pub type UnsafeId = *mut objc::runtime::Object;
#[cfg(feature = "objc")]
pub type UnsafeBool = objc::runtime::BOOL;
