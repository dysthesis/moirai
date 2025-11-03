use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::format::Format;

#[derive(Debug)]
struct Description(String);

#[derive(Debug)]
struct Tag(String);

#[derive(Debug)]
pub struct Item<F: Format> {
    /// An immutable unique identifier for this item
    id: Uuid,
    /// The item's description.
    description: Description,
    /// THe set of tags marking this item
    tags: Vec<Tag>,
    /// The time when this item was created.
    /// NOTE: This must be immutable.
    ctime: DateTime<Utc>,
    /// The time this item was last modified.
    /// NOTE: This time MUST be no earlier than `atime`; it does not make sense
    /// to have an item modified before it was accessed.
    mtime: DateTime<Utc>,
    /// The time this item was last accessed.
    atime: DateTime<Utc>,
    format: F,
}
