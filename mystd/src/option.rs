pub mod mystd {
    pub enum Option<T> {
        None,
        Some(T),
    }
}
impl<T: Clone> mystd::Option<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, mystd::Option::None)
    }

    pub fn is_none_or(&self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            mystd::Option::None => true,
            mystd::Option::Some(x) => f(x.clone()),
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    pub fn is_some_and(&self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            mystd::Option::None => false,
            mystd::Option::Some(x) => f(x.clone()),
        }
    }
}
