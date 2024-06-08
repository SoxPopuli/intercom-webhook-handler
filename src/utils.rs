pub(crate) trait Pipe: Sized {
    fn pipe<T: Sized>(self, func: impl FnOnce(Self) -> T) -> T {
        func(self)
    }
}

impl<T> Pipe for T { }
