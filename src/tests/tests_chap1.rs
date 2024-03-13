    use crate::*;

    #[test]
    fn test_chap1_1() {
        assert_stdout_eq!(chap1::hello_world::main(), "Hello World!\nI\'m a Rustacean!\n");
    }

    #[test]
    fn test_chap1_2() {
        assert_stdout_eq!(chap1::format1::main(), "31 days\nAlice, this is Bob. Bob, this is Alice\nthe quick brown fox jumps over the lazy dog\nBase 10:               69420\nBase 2 (binary):       10000111100101100\nBase 8 (octal):        207454\nBase 16 (hexadecimal): 10f2c\nBase 16 (hexadecimal): 10F2C\n    1\n00001\n10000\n00001\nMy name is Bond, James Bond\n    1\n");
    }
