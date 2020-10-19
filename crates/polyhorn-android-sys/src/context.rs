use super::{Object, Reference};

pub struct Context {
    reference: Reference,
}

impl Context {
    pub fn from_ref(reference: Reference) -> Context {
        Context { reference }
    }
}

impl Object for Context {
    fn from_reference(reference: Reference) -> Self {
        Context { reference }
    }

    fn as_reference(&self) -> &Reference {
        &self.reference
    }
}
