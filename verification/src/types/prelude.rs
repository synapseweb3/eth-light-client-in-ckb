pub use molecule::prelude::{Builder, Entity, Reader};

/// A syntactic sugar to convert a rust type into binary data.
pub trait Pack<T: Entity> {
    /// Packs a rust type into binary data.
    fn pack(&self) -> T;
}

/// A syntactic sugar to convert binary data into rust types.
pub trait Unpack<T> {
    /// Unpack binary data into rust types.
    fn unpack(&self) -> T;
}
