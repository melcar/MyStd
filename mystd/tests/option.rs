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
    fn map() {
        let none_case = mystd::Option::<i32>::None;
        assert!(none_case.map(|x| x * 2).is_none());

        let some_case = mystd::Option::<i32>::Some(1);
        assert!(some_case.map(|x| x * 2).is_some_and(|x| x == 2));
    }

    #[test]
    fn as_ref() {
        //https://doc.rust-lang.org/std/option/enum.Option.html#examples-4
        let hello_world = "Hello world!".to_string();
        let text: Option<String> = Some(hello_world.clone());
        let text_length: Option<usize> = text.as_ref().map(|s| s.len());

        assert!(text.is_some_and(|s| s == hello_world));
        assert!(text_length.is_some_and(|l| l == hello_world.len()));
    }
}
