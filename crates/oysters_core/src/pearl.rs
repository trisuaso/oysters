pub const EPOCH_YEAR: u16 = 2025;
#[cfg(feature = "lru")]
use crate::time::epoch_timestamp;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResourceDescriptor {
    /// The timestamp of the last time this resource was used.
    #[cfg(feature = "lru")]
    pub used: usize,
}

impl Default for ResourceDescriptor {
    fn default() -> Self {
        Self {
            #[cfg(feature = "lru")]
            used: epoch_timestamp(EPOCH_YEAR),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pearl<V: Clone>(pub V, pub ResourceDescriptor);

impl<V: Clone> Pearl<V> {
    pub fn new(value: V) -> Self {
        Self(value, ResourceDescriptor::default())
    }
}
