use crate::format::Format;

// TODO: Figure out how to construct and form a syntax tree from Markdown
pub struct Markdown {
    frontmatter: Option<String>,
    body: String,
}

impl TryFrom<String> for Markdown {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!("Implement validation and parsing logic for Markdown")
    }
}

impl Format for Markdown {
    fn serialise(item: &crate::item::Item) -> Self {
        todo!()
    }

    fn deserialise(self: &Self) -> crate::item::Item {
        todo!()
    }
}
