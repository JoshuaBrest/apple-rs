use crate::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A NSNotification
pub struct NSNotification(pub UnsafeId);

impl NSNotification {
    
}

impl IsType for NSNotification {
    type Error = ();

    fn is_type(obj: UnsafeId) -> Result<bool, Self::Error> {
        let raw: UnsafeBool = unsafe { msg_send![obj, isKindOfClass: class!(NSNotification)] };
        let is: Boolean = raw.try_into()?;

        Ok(is.into())
    }
}
