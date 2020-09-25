use std::marker::PhantomData;

pub struct Auto(PhantomData<()>);

pub fn auto() -> Auto {
    Auto(Default::default())
}
