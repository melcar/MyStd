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

    #[test]
    fn insert() {
        let mut insert_into = mystd::Option::<i32>::None;
        insert_into.insert(1);
        assert!(insert_into.is_some_and(|x| x == 1));
        insert_into.insert(2);
        assert!(insert_into.is_some_and(|x| x == 2));
    }

    #[test]
    fn as_ref() {
        let mut option = mystd::Option::<i32>::None;
        let option_ref = option.as_ref();
        //option.insert(1);
        assert!(option_ref.is_some_and(|&x| x == 1));
    }
}
