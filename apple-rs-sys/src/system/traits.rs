use crate::utils::UnsafeId;

pub trait IsType {
    type Error;

    /// Returns true if the object is of the type.
    fn is_type(obj: UnsafeId) -> Result<bool, Self::Error>;
}
