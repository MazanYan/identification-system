const CODE_DIGITS_COUNT: usize = 9;
const CODE_DISPLAY_LENGTH: usize = 12;
const BAR_CODE_LENGTH: usize = 116;

type Code = u16;

const ZERO:  Code = 0b000110100;
const ONE:   Code = 0b100100001;
const TWO:   Code = 0b001100001;
const THREE: Code = 0b101100000;
const FOUR:  Code = 0b000110001;
const FIVE:  Code = 0b100110000;
const SIX:   Code = 0b001110000;
const SEVEN: Code = 0b000100101;
const EIGHT: Code = 0b100100100;
const NINE:  Code = 0b001100100;

const ASTERISK: Code = 0b010010100;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
    Space,
}

pub fn bar_code(num: u32) -> Result<Vec<Color>, String> {
    let digs = digits(num);
    let sum = checksum(&digs)?;

    let mut code = Vec::with_capacity(BAR_CODE_LENGTH);
    code.extend(code_to_lines(ASTERISK));
    code.extend([Color::Space]);

    for &d in digs.iter() {
        code.extend(code_to_lines(code_from_number(d)));
        code.extend([Color::Space]);
    }

    code.extend(code_to_lines(code_from_number(sum)));
    code.extend([Color::Space]);
    code.extend(code_to_lines(ASTERISK));

    Ok(code)
}

fn code_from_number(num: u8) -> Code {
    match num {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        _ => ZERO,
    }
}

fn code_to_lines(code: Code) -> Vec<Color> {
    let mut lines = Vec::with_capacity(CODE_DISPLAY_LENGTH);

    let mut color = Color::Black;

    for i in (0..CODE_DIGITS_COUNT).rev() {
        let mask = 1 << i;
        match (code & mask) >> i {
            1 => {
                lines.push(color);
                lines.push(color);
            },
            0 => lines.push(color),
            _ => (),
        }

        color = match color {
            Color::Black => Color::White,
            Color::White => Color::Black,
            other       => other,
        }
    }

    lines
}

fn checksum(digits: &Vec<u8>) -> Result<u8, String> {
    if digits.len() != 6 {
        return Err(String::from("CIP can be counted for 6 digit numbers only"));
    }

    let mut checksum = 0;

    for (i, &d) in digits.iter().enumerate() {
        let coef = (i as u8) + 2;
        checksum += d * coef;
    }

    Ok(checksum % 11)
}

fn digits(num: u32) -> Vec<u8> {
    let digits = count_digits(num);
    let mut vec = Vec::with_capacity(digits as usize);

    for i in 0..digits {
        let digit = (num / 10_u32.pow(i) % 10) as u8;
        vec.push(digit);
    }

    vec.reverse();
    vec
}

fn count_digits(num: u32) -> u32 {
    let mut result = 0;
    let mut num = num;
    
    loop {
        num /= 10;
        result += 1;

        if num == 0 {
            return result;
        }
    }
}
