use crate::util::variant_eq;

#[derive(Debug)]
pub enum DeltaVariant {
    Add,
    Remove,
}

#[derive(Debug)]
pub struct Delta {
    pub delta: String,
    pub line: usize,
    pub variant: DeltaVariant,
}

impl Delta {
    pub fn added(delta: &str, line: usize) -> Self {
        Self {
            delta: String::from(delta),
            line,
            variant: DeltaVariant::Add,
        }
    }

    pub fn removed(delta: &str, line: usize) -> Self {
        Self {
            delta: String::from(delta),
            line,
            variant: DeltaVariant::Remove,
        }
    }
}

impl PartialEq for Delta {
    fn eq(&self, other: &Self) -> bool {
        self.delta == other.delta
            && self.line == other.line
            && variant_eq(&self.variant, &other.variant)
    }
}
