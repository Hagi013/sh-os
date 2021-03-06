use super::super::super::spin::once::Once;

pub struct Lazy<T: Sync>(Once<T>);

impl<T: Sync> Lazy<T> {
    pub const INIT: Self = Lazy(Once::INIT);

    #[inline(always)]
    pub fn get<F>(&'static self, builder: F) -> &T
        where F: FnOnce() -> T
    {
        self.0.call_once(builder)
    }
}

#[macro_export]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T: ty) => {
        static $NAME: $crate::lazy::Lazy<$T> = $crate::lazy::Lazy::INIT;
    }
}
