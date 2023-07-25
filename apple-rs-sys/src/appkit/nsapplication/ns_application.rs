use crate::utils::*;

#[inline]
/// Returns the shared application instance.
pub fn shared_application() -> Id<runtime::Object> {
    unsafe { msg_send![class!(NSApplication), sharedApplication] }
}
