#[cfg(test)]
mod tests {
    use mystd::option::mystd;

    #[test]
    fn is_none() {
        assert!((mystd::Option::<i32>::None).is_none());
        assert!(!(mystd::Option::<i32>::Some(1)).is_none());
    }

    #[test]
    fn is_some() {
        assert!(!(mystd::Option::<i32>::None).is_some());
        assert!((mystd::Option::<i32>::Some(1)).is_some());
    }

    #[test]
    fn is_none_or() {
        assert!((mystd::Option::<i32>::None).is_none_or(|_| true));
        assert!(!(mystd::Option::<i32>::Some(1)).is_none_or(|x| x == 0));
        assert!((mystd::Option::<i32>::Some(1)).is_none_or(|x| x == 1));
    }

    #[test]
    fn is_some_and() {
        assert!(!(mystd::Option::<i32>::None).is_some_and(|_| true));
        assert!(!(mystd::Option::<i32>::Some(1)).is_some_and(|x| x == 0));
        assert!((mystd::Option::<i32>::Some(1)).is_some_and(|x| x == 1));
    }
}
