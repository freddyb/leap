#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


extern crate leap;

#[test]
fn test_strip_bold() {
    assert_eq!(leap::strip("l\x02ol"), "lol");
}

#[test]
fn test_strip_fg_color() {
    assert_eq!(leap::strip("l\x033ol"), "lol");
}

#[test]
fn test_strip_fg_color2() {
    assert_eq!(leap::strip("l\x0312ol"), "lol");
}

#[test]
fn test_strip_fg_bg_11() {
    assert_eq!(leap::strip("l\x031,2ol"), "lol");
}
#[test]
fn test_strip_fg_bg_21() {
    assert_eq!(leap::strip("l\x0312,3ol"), "lol");
}
#[test]
fn test_strip_fg_bg_12() {
    assert_eq!(leap::strip("l\x031,12ol"), "lol");
}
#[test]
fn test_strip_fg_bg_22() {
    assert_eq!(leap::strip("l\x0312,13ol"), "lol");
}
