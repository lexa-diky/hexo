pub(crate) struct Deferred<F: FnMut()> {
    f: F,
}

impl<F: FnMut()> Deferred<F> {
    pub(crate) fn new(f: F) -> Self {
        Deferred { f }
    }
}

impl<F: FnMut()> Drop for Deferred<F> {
    fn drop(&mut self) {
        (self.f)();
    }
}

macro_rules! defer {
    ($e:stmt) => (
        use crate::util::Deferred;
        let _deferred = Deferred::new(|| -> () { $e });
    )
}

pub(crate) use defer;