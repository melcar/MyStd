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

    pub fn as_ref(&self) -> mystd::Option<&T> {
        match self {
            mystd::Option::None => mystd::Option::None,
            mystd::Option::Some(x) => mystd::Option::Some(x),
        }
    }

    pub fn as_mut(&mut self) -> mystd::Option<&mut T> {
        match self {
            mystd::Option::None => mystd::Option::None,
            mystd::Option::Some(x) => mystd::Option::Some(x),
        }
    }

    pub fn insert(&mut self, t: T) -> &T {
        *self = mystd::Option::Some(t);
        match self {
            mystd::Option::Some(x) => x,
            mystd::Option::None => unreachable!(),
        }
    }
}
