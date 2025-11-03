use std::path::PathBuf;

use crate::item::Item;

mod markdown;
pub trait Format: Sized {
    fn serialise(item: &Item<_>) -> Self;
    fn deserialise(self: &Self) -> Item<_>;
}
