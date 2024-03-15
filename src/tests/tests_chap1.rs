use serial_test::serial;
use crate::*;

#[test]
#[serial]
fn test_chap1() {
    assert_stdout_eq!(chap1::hello_world::hello_world(), "Hello World!\nI\'m a Rustacean!\n");

    assert_stdout_eq!(chap1::format1::format1(), "31 days\nAlice, this is Bob. Bob, this is Alice\nthe quick brown fox jumps over the lazy dog\nBase 10:               69420\nBase 2 (binary):       10000111100101100\nBase 8 (octal):        207454\nBase 16 (hexadecimal): 10f2c\nBase 16 (hexadecimal): 10F2C\n    1\n00001\n10000\n00001\nMy name is Bond, James Bond\n    1\n");

    assert_stdout_eq!(chap1::format122::format122(), "Compare structures:\nDisplay: (0, 14)\nDebug: MinMax(0, 14)\nThe big range is (-300, 300) and the small is (-3, 3)\nCompare points:\nDisplay: x: 3.3, y: 7.2\nDebug: Point2D { x: 3.3, y: 7.2 }\nCompare complex:\nDisplay: 3.3 + 7.2i\nDebug: Complex { real: 3.3, imag: 7.2 }\n");

    assert_stdout_eq!(chap1::format1221::format1221(), "[0: 1, 1: 2, 2: 3]\n");

    assert_stdout_eq!(chap1::format123::format123(), "RGB (128, 255, 90) 0x80FF5A\nRGB (0, 3, 254) 0x0003FE\nRGB (0, 0, 0) 0x000000\n");
}