#[cfg(test)]
mod tests {
    use crate::code::Color::*;
    use crate::code::bar_code;

    #[test]
    fn test_bar_code() {
        let input = 348165;
        let expected = [
            Black, White, White, Black,
            White, Black, Black, White,
            Black, Black, White, Black,
            Space, Black, Black, White,
            Black, Black, White, White,
            Black, White, Black, White,
            Black, Space, Black, White,
            Black, White, White, Black,
            Black, White, Black, White,
            Black, Black, Space, Black,
            Black, White, Black, White,
            White, Black, White, Black,
            Black, White, Black, Space,
            Black, Black, White, Black,
            White, White, Black, White,
            Black, White, Black, Black,
            Space, Black, White, Black,
            Black, White, White, Black,
            Black, White, Black, White,
            Black, Space, Black, Black,
            White, Black, White, White,
            Black, Black, White, Black,
            White, Black, Space, Black,
            Black, White, Black, White,
            White, Black, Black, White,
            Black, White, Black, Space,
            Black, White, White, Black,
            White, Black, Black, White,
            Black, Black, White, Black,
        ];

        let result = bar_code(input).unwrap();

        assert_eq!(expected.len(), result.len());
        assert_eq!(*expected.get(0).unwrap(), *result.get(0).unwrap());
        assert_eq!(*expected.get(55).unwrap(), *result.get(55).unwrap());
        assert_eq!(*expected.last().unwrap(), *result.last().unwrap());
    }

    #[test]
    #[should_panic(expected = "CIP can be counted for 6 digit numbers only")]
    fn test_bar_code_incorrect_input() {
        bar_code(34816).unwrap();
    }
}
