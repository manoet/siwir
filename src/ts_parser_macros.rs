use crate::basic_parser::State;
use crate::basic_parser::Transition;

#[test]
fn chr_match() {
    let matcher = chr!('a');
    let result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek_many(3), "bcd");
}

#[test]
fn chr_no_match() {
    let matcher = chr!('a');
    let result = matcher.matches(&State::from_string("ABCDabcd"));
    assert!(result.is_none());
}

#[test]
fn range_chr_match() {
    let matcher = chr!('0', '9');
    let result = matcher.matches(&State::from_string("0"));
    assert!(result.is_some());
    let result = matcher.matches(&State::from_string("5"));
    assert!(result.is_some());
    let result = matcher.matches(&State::from_string("9"));
    assert!(result.is_some());
}

#[test]
fn range_chr_no_match() {
    let matcher = chr!('0', '9');
    let result = matcher.matches(&State::from_string("ABCD1234"));
    assert!(result.is_none());
}

#[test]
fn seq() {
    let matcher = seq!(chr!('a'), chr!('b'), chr!('c'));
    let result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'd');
}

#[test]
fn alt_match() {
    let state = State::from_string("abc");
    let matcher1 = alt!(chr!('a'), chr!('b'));
    let matcher2 = alt!(chr!('b'), chr!('a'));
    let result1 = matcher1.matches(&state);
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().peek(), 'b');
    let result2 = matcher2.matches(&state);
    assert!(result2.is_some());
    assert_eq!(result2.unwrap().peek(), 'b');
}

#[test]
fn alt_no_match() {
    let state = State::from_string("cba");
    let matcher = alt!(chr!('a'), chr!('b'));
    let result = matcher.matches(&state);
    assert!(result.is_none());
}

#[test]
fn opt() {
    let matcher = opt!(chr!('b'));
    // For opt being false
    let result= matcher.matches(&State::from_string("abc"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'a');
    // for opt being true
    let result = matcher.matches(&State::from_string("bcd"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'c');
}

#[test]
fn rep() {
    // Testing ?
    let matcher = rep!(chr!('a'), '?');
    let mut result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_some());
    // Testing +
    let matcher = rep!(chr!('a'), '+');
    result = matcher.matches(&State::from_string("aabcd"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'b');
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_none());
    // Testing *
    let matcher = rep!(chr!('a'), '*');
    result = matcher.matches(&State::from_string("aaaaaaabcd"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'b');
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().peek(), 'b');
}

