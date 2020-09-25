pub struct Disposable(Box<dyn Drop>);

impl Disposable {
    pub fn new<T>(value: T) -> Disposable
    where
        T: Drop + 'static,
    {
        Disposable(Box::new(value))
    }
}
