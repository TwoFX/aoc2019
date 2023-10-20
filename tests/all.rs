use paste::paste;
use std::fs;

macro_rules! test {
    ($day:ident) => {
        paste! {
            use aoc19::days::$day;

            #[test]
            fn [<test_ $day _part_a>]() {
                let input = fs::read_to_string(concat!("inputs/", stringify!($day), ".in")).unwrap();
                let expected_output = fs::read_to_string(concat!("inputs/", stringify!($day), "a.out")).unwrap();
                let actual_output = $day::part_a(input.as_str()).unwrap();
                assert_eq!(expected_output.trim(), actual_output.trim());
            }

            #[test]
            fn [<test_ $day _part_b>]() {
                let input = fs::read_to_string(concat!("inputs/", stringify!($day), ".in")).unwrap();
                let expected_output = fs::read_to_string(concat!("inputs/", stringify!($day), "b.out")).unwrap();
                let actual_output = $day::part_b(input.as_str()).unwrap();
                assert_eq!(expected_output.trim(), actual_output.trim());
            }
        }
    };
}

test! {day1}
test! {day2}
test! {day4}
test! {day5}
