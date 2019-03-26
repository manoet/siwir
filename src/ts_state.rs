use crate::basic_parser::State;

#[test]
fn peek_one() {
    let s = State::from_string("Hello, World!");
    assert_eq!(s.peek(), 'H');
}

#[test]
fn peek_one_exceed() {
    let s = State::from_string("");
    assert_eq!(s.peek(), '\0');
}

#[test]
fn peek_many() {
    let s = State::from_string("Hello, World!");
    assert_eq!(s.peek_many(7), "Hello, ");
}

#[test]
fn peek_many_exceed() {
    let s = State::from_string("Hi");
    assert_eq!(s.peek_many(3), "");
}

#[test]
fn complete_true() {
    let s = State::from_string("");
    assert!(s.complete());
}

#[test]
fn complete_false() {
    let s = State::from_string("A");
    assert_eq!(s.complete(), false);
}

#[test]
fn read() {
    let s1 = State::from_string("Hello, World!");
    let s2 = s1.read(7);
    assert_eq!(s2.peek_many(6), "World!");
}

