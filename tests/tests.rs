#[test]
fn test_messages() {
    let messages = makai::utils::messages::Messages::new();

    messages.push(String::from("hello"));
    messages.push(0usize);
    messages.push(String::from("world"));
    messages.push(1usize);

    assert!(messages.check_activity());
    assert!(!messages.check_activity());

    assert_eq!(
        messages.get::<String>(),
        vec![String::from("hello"), String::from("world")]
    );
    assert!(messages.get::<String>().is_empty());

    assert_eq!(messages.get::<usize>(), vec![0usize, 1usize]);
    assert!(messages.get::<usize>().is_empty());

    assert!(messages.check_activity());
    assert!(!messages.check_activity());
}
