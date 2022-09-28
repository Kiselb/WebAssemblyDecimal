#![feature(bigint_helper_methods)]
#![feature(const_bigint_helper_methods)]

mod utils;

use core::panic;
use std::convert::TryFrom;
use std::{fmt, result};
use std::ops::Add;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-decimal!");
}

#[wasm_bindgen]
pub fn calculation(operand1: String, operand2: String, operation: i32) -> String {
    let decimal_operand1 = Decimal::parse(&operand1).unwrap();
    let decimal_operand2 = Decimal::parse(&operand2).unwrap();
    match operation {
        0 => {
            let result = decimal_operand1.add(&decimal_operand2);
            result.to_string()
        },
        1 => {
            let result = decimal_operand1.sub(&decimal_operand2);
            result.to_string()
        },
        2 => {
            let result = decimal_operand1.mul(&decimal_operand2).unwrap();
            result.to_string()
        },
        4 => {
            let result = decimal_operand1.div(&decimal_operand2);
            result.to_string()
        },
        _ => String::from("0"),
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalSign {
    Positive = 0,
    Negative = 1,
}

//#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalPrecision {
    Precision01 = 1,
    Precision02 = 2,
    Precision03 = 3,
    Precision04 = 4,
    Precision05 = 5,
    Precision06 = 6,
    Precision07 = 7,
    Precision08 = 8,
    Precision09 = 9,
    Precision10 = 10,
    Precision11 = 11,
    Precision12 = 12,
    Precision13 = 13,
    Precision14 = 14,
    Precision15 = 15,
    Precision16 = 16,
    Precision17 = 17,
    Precision18 = 18,
    Precision19 = 19,
    Precision20 = 20,
    Precision21 = 21,
    Precision22 = 22,
    Precision23 = 23,
    Precision24 = 24,
    Precision25 = 25,
    Precision26 = 26,
    Precision27 = 27,
    Precision28 = 28,
    Precision29 = 29,
    Precision30 = 30,
    Precision31 = 31,
    Precision32 = 32,
    Precision33 = 33,
    Precision34 = 34,
    Precision35 = 35,
    Precision36 = 36,
    Precision37 = 37,
    Precision38 = 38,
}

impl DecimalPrecision {
    pub fn inc(precision: DecimalPrecision) -> Result<DecimalPrecision, DecimalError> {
        // Change to Result with Specific Decimal Error
        match DecimalPrecision::try_from_usize(precision as usize + 1) {
            Ok(value) => Ok(value),
            Err(_) => Err(DecimalError),
        }
    }
    pub fn dec(precision: DecimalPrecision) -> Result<DecimalPrecision, DecimalError> {
        // Change to Result with Specific Decimal Error
        match DecimalPrecision::try_from_usize(precision as usize - 1) {
            Ok(value) => Ok(value),
            Err(_) => Err(DecimalError),
        }
    }
    fn try_from_usize(value: usize) -> Result<DecimalPrecision, ()> {
        match value {
            1 => Ok(DecimalPrecision::Precision01),
            2 => Ok(DecimalPrecision::Precision02),
            3 => Ok(DecimalPrecision::Precision03),
            4 => Ok(DecimalPrecision::Precision04),
            5 => Ok(DecimalPrecision::Precision05),
            6 => Ok(DecimalPrecision::Precision06),
            7 => Ok(DecimalPrecision::Precision07),
            8 => Ok(DecimalPrecision::Precision08),
            9 => Ok(DecimalPrecision::Precision09),
            10 => Ok(DecimalPrecision::Precision10),
            11 => Ok(DecimalPrecision::Precision11),
            12 => Ok(DecimalPrecision::Precision12),
            13 => Ok(DecimalPrecision::Precision13),
            14 => Ok(DecimalPrecision::Precision14),
            15 => Ok(DecimalPrecision::Precision15),
            16 => Ok(DecimalPrecision::Precision16),
            17 => Ok(DecimalPrecision::Precision17),
            18 => Ok(DecimalPrecision::Precision18),
            19 => Ok(DecimalPrecision::Precision19),
            20 => Ok(DecimalPrecision::Precision20),
            21 => Ok(DecimalPrecision::Precision21),
            22 => Ok(DecimalPrecision::Precision22),
            23 => Ok(DecimalPrecision::Precision23),
            24 => Ok(DecimalPrecision::Precision24),
            25 => Ok(DecimalPrecision::Precision25),
            26 => Ok(DecimalPrecision::Precision26),
            27 => Ok(DecimalPrecision::Precision27),
            28 => Ok(DecimalPrecision::Precision28),
            29 => Ok(DecimalPrecision::Precision29),
            30 => Ok(DecimalPrecision::Precision30),
            31 => Ok(DecimalPrecision::Precision31),
            32 => Ok(DecimalPrecision::Precision32),
            33 => Ok(DecimalPrecision::Precision33),
            34 => Ok(DecimalPrecision::Precision34),
            35 => Ok(DecimalPrecision::Precision35),
            36 => Ok(DecimalPrecision::Precision36),
            37 => Ok(DecimalPrecision::Precision37),
            38 => Ok(DecimalPrecision::Precision38),
            _ => Err(()), // Result: Err("Invalid Decimal Scale")
        }
    }
    pub fn try_to_usize(value: DecimalPrecision) -> Result<usize, ()> {
        match value {
            DecimalPrecision::Precision01 => Ok(1),
            DecimalPrecision::Precision02 => Ok(2),
            DecimalPrecision::Precision03 => Ok(3),
            DecimalPrecision::Precision04 => Ok(4),
            DecimalPrecision::Precision05 => Ok(5),
            DecimalPrecision::Precision06 => Ok(6),
            DecimalPrecision::Precision07 => Ok(7),
            DecimalPrecision::Precision08 => Ok(8),
            DecimalPrecision::Precision09 => Ok(9),
            DecimalPrecision::Precision10 => Ok(10),
            DecimalPrecision::Precision11 => Ok(11),
            DecimalPrecision::Precision12 => Ok(12),
            DecimalPrecision::Precision13 => Ok(13),
            DecimalPrecision::Precision14 => Ok(14),
            DecimalPrecision::Precision15 => Ok(15),
            DecimalPrecision::Precision16 => Ok(16),
            DecimalPrecision::Precision17 => Ok(17),
            DecimalPrecision::Precision18 => Ok(18),
            DecimalPrecision::Precision19 => Ok(19),
            DecimalPrecision::Precision20 => Ok(20),
            DecimalPrecision::Precision21 => Ok(21),
            DecimalPrecision::Precision22 => Ok(22),
            DecimalPrecision::Precision23 => Ok(23),
            DecimalPrecision::Precision24 => Ok(24),
            DecimalPrecision::Precision25 => Ok(25),
            DecimalPrecision::Precision26 => Ok(26),
            DecimalPrecision::Precision27 => Ok(27),
            DecimalPrecision::Precision28 => Ok(28),
            DecimalPrecision::Precision29 => Ok(29),
            DecimalPrecision::Precision30 => Ok(30),
            DecimalPrecision::Precision31 => Ok(31),
            DecimalPrecision::Precision32 => Ok(32),
            DecimalPrecision::Precision33 => Ok(33),
            DecimalPrecision::Precision34 => Ok(34),
            DecimalPrecision::Precision35 => Ok(35),
            DecimalPrecision::Precision36 => Ok(36),
            DecimalPrecision::Precision37 => Ok(37),
            DecimalPrecision::Precision38 => Ok(38),
        }
    }
}

// impl TryFrom<usize> for DecimalPrecision {
//     type Error = (); // Change to DecimalError
//
//     fn try_from(value: usize) -> Result<DecimalPrecision, ()> {
//         match value {
//             1 => Ok(DecimalPrecision::Precision01),
//             2 => Ok(DecimalPrecision::Precision02),
//             3 => Ok(DecimalPrecision::Precision03),
//             4 => Ok(DecimalPrecision::Precision04),
//             5 => Ok(DecimalPrecision::Precision05),
//             6 => Ok(DecimalPrecision::Precision06),
//             7 => Ok(DecimalPrecision::Precision07),
//             8 => Ok(DecimalPrecision::Precision08),
//             9 => Ok(DecimalPrecision::Precision09),
//             10 => Ok(DecimalPrecision::Precision10),
//             11 => Ok(DecimalPrecision::Precision11),
//             12 => Ok(DecimalPrecision::Precision12),
//             13 => Ok(DecimalPrecision::Precision13),
//             14 => Ok(DecimalPrecision::Precision14),
//             15 => Ok(DecimalPrecision::Precision15),
//             16 => Ok(DecimalPrecision::Precision16),
//             17 => Ok(DecimalPrecision::Precision17),
//             18 => Ok(DecimalPrecision::Precision18),
//             19 => Ok(DecimalPrecision::Precision19),
//             20 => Ok(DecimalPrecision::Precision20),
//             21 => Ok(DecimalPrecision::Precision21),
//             22 => Ok(DecimalPrecision::Precision22),
//             23 => Ok(DecimalPrecision::Precision23),
//             24 => Ok(DecimalPrecision::Precision24),
//             25 => Ok(DecimalPrecision::Precision25),
//             26 => Ok(DecimalPrecision::Precision26),
//             27 => Ok(DecimalPrecision::Precision27),
//             28 => Ok(DecimalPrecision::Precision28),
//             29 => Ok(DecimalPrecision::Precision29),
//             30 => Ok(DecimalPrecision::Precision30),
//             31 => Ok(DecimalPrecision::Precision31),
//             32 => Ok(DecimalPrecision::Precision32),
//             33 => Ok(DecimalPrecision::Precision33),
//             34 => Ok(DecimalPrecision::Precision34),
//             35 => Ok(DecimalPrecision::Precision35),
//             36 => Ok(DecimalPrecision::Precision36),
//             37 => Ok(DecimalPrecision::Precision37),
//             38 => Ok(DecimalPrecision::Precision38),
//             _ => Err(()), // Result: Err("Invalid Decimal Scale")
//         }
//     }
// }

// impl TryFrom<DecimalPrecision> for usize {
//     type Error = ();
//
//     fn try_from(value: DecimalPrecision) -> Result<usize, ()> {
//         match value {
//             DecimalPrecision::Precision01 => Ok(1),
//             DecimalPrecision::Precision02 => Ok(2),
//             DecimalPrecision::Precision03 => Ok(3),
//             DecimalPrecision::Precision04 => Ok(4),
//             DecimalPrecision::Precision05 => Ok(5),
//             DecimalPrecision::Precision06 => Ok(6),
//             DecimalPrecision::Precision07 => Ok(7),
//             DecimalPrecision::Precision08 => Ok(8),
//             DecimalPrecision::Precision09 => Ok(9),
//             DecimalPrecision::Precision10 => Ok(10),
//             DecimalPrecision::Precision11 => Ok(11),
//             DecimalPrecision::Precision12 => Ok(12),
//             DecimalPrecision::Precision13 => Ok(13),
//             DecimalPrecision::Precision14 => Ok(14),
//             DecimalPrecision::Precision15 => Ok(15),
//             DecimalPrecision::Precision16 => Ok(16),
//             DecimalPrecision::Precision17 => Ok(17),
//             DecimalPrecision::Precision18 => Ok(18),
//             DecimalPrecision::Precision19 => Ok(19),
//             DecimalPrecision::Precision20 => Ok(20),
//             DecimalPrecision::Precision21 => Ok(21),
//             DecimalPrecision::Precision22 => Ok(22),
//             DecimalPrecision::Precision23 => Ok(23),
//             DecimalPrecision::Precision24 => Ok(24),
//             DecimalPrecision::Precision25 => Ok(25),
//             DecimalPrecision::Precision26 => Ok(26),
//             DecimalPrecision::Precision27 => Ok(27),
//             DecimalPrecision::Precision28 => Ok(28),
//             DecimalPrecision::Precision29 => Ok(29),
//             DecimalPrecision::Precision30 => Ok(30),
//             DecimalPrecision::Precision31 => Ok(31),
//             DecimalPrecision::Precision32 => Ok(32),
//             DecimalPrecision::Precision33 => Ok(33),
//             DecimalPrecision::Precision34 => Ok(34),
//             DecimalPrecision::Precision35 => Ok(35),
//             DecimalPrecision::Precision36 => Ok(36),
//             DecimalPrecision::Precision37 => Ok(37),
//             DecimalPrecision::Precision38 => Ok(38),
//         }
//     }
// }

//#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalScale {
    Scale00 = 0,
    Scale01 = 1,
    Scale02 = 2,
    Scale03 = 3,
    Scale04 = 4,
    Scale05 = 5,
    Scale06 = 6,
    Scale07 = 7,
    Scale08 = 8,
    Scale09 = 9,
    Scale10 = 10,
    Scale11 = 11,
    Scale12 = 12,
    Scale13 = 13,
    Scale14 = 14,
    Scale15 = 15,
    Scale16 = 16,
    Scale17 = 17,
    Scale18 = 18,
    Scale19 = 19,
    Scale20 = 20,
    Scale21 = 21,
    Scale22 = 22,
    Scale23 = 23,
    Scale24 = 24,
    Scale25 = 25,
    Scale26 = 26,
    Scale27 = 27,
    Scale28 = 28,
    Scale29 = 29,
    Scale30 = 30,
    Scale31 = 31,
    Scale32 = 32,
    Scale33 = 33,
    Scale34 = 34,
    Scale35 = 35,
    Scale36 = 36,
    Scale37 = 37,
    Scale38 = 38,
}

//#[wasm_bindgen]
impl DecimalScale {
    pub fn inc(scale: DecimalScale) -> Result<DecimalScale, DecimalError> {
        // Change to Result with Specific Decimal Error: Scale Overflow
        match DecimalScale::try_from_usize(scale as usize + 1) {
            Ok(value) => Ok(value),
            Err(_) => Err(DecimalError),
        }
    }
    pub fn dec(scale: DecimalScale) -> Result<DecimalScale, DecimalError> {
        // Change to Result with Specific Decimal Error: Scale Underflow
        match DecimalScale::try_from_usize(scale as usize - 1) {
            Ok(value) => Ok(value),
            Err(_) => Err(DecimalError),
        }
    }
    fn add(self, rhs: DecimalScale) -> Result<DecimalScale, DecimalError> {
        // Change to Result with Specific Decimal Error: Scale Overflow
        match DecimalScale::try_from_usize(rhs as usize + self as usize) {
            Ok(value) => Ok(value),
            Err(_) => Err(DecimalError),
        }
    }
    fn try_from_usize(value: usize) -> Result<DecimalScale, ()> {
        match value {
            0 => Ok(DecimalScale::Scale00),
            1 => Ok(DecimalScale::Scale01),
            2 => Ok(DecimalScale::Scale02),
            3 => Ok(DecimalScale::Scale03),
            4 => Ok(DecimalScale::Scale04),
            5 => Ok(DecimalScale::Scale05),
            6 => Ok(DecimalScale::Scale06),
            7 => Ok(DecimalScale::Scale07),
            8 => Ok(DecimalScale::Scale08),
            9 => Ok(DecimalScale::Scale09),
            10 => Ok(DecimalScale::Scale10),
            11 => Ok(DecimalScale::Scale11),
            12 => Ok(DecimalScale::Scale12),
            13 => Ok(DecimalScale::Scale13),
            14 => Ok(DecimalScale::Scale14),
            15 => Ok(DecimalScale::Scale15),
            16 => Ok(DecimalScale::Scale16),
            17 => Ok(DecimalScale::Scale17),
            18 => Ok(DecimalScale::Scale18),
            19 => Ok(DecimalScale::Scale19),
            20 => Ok(DecimalScale::Scale20),
            21 => Ok(DecimalScale::Scale21),
            22 => Ok(DecimalScale::Scale22),
            23 => Ok(DecimalScale::Scale23),
            24 => Ok(DecimalScale::Scale24),
            25 => Ok(DecimalScale::Scale25),
            26 => Ok(DecimalScale::Scale26),
            27 => Ok(DecimalScale::Scale27),
            28 => Ok(DecimalScale::Scale28),
            29 => Ok(DecimalScale::Scale29),
            30 => Ok(DecimalScale::Scale30),
            31 => Ok(DecimalScale::Scale31),
            32 => Ok(DecimalScale::Scale32),
            33 => Ok(DecimalScale::Scale33),
            34 => Ok(DecimalScale::Scale34),
            35 => Ok(DecimalScale::Scale35),
            36 => Ok(DecimalScale::Scale36),
            37 => Ok(DecimalScale::Scale37),
            38 => Ok(DecimalScale::Scale38),
            _ => panic!("Invalid scale"), // Result: Err("Invalid Decimal Scale")
        }
    }
}

// impl Add for DecimalScale {
//     type Output = Result<DecimalScale, DecimalError>;

//     fn add(self, rhs: DecimalScale) -> Result<DecimalScale, DecimalError> {
//         // Change to Result with Specific Decimal Error: Scale Overflow
//         match DecimalScale::try_from(rhs as usize + self as usize) {
//             Ok(value) => Ok(value),
//             Err(_) => Err(DecimalError),
//         }
//     }
// }

// impl TryFrom<usize> for DecimalScale {
//     type Error = ();

//     fn try_from(value: usize) -> Result<DecimalScale, ()> {
//         match value {
//             0 => Ok(DecimalScale::Scale00),
//             1 => Ok(DecimalScale::Scale01),
//             2 => Ok(DecimalScale::Scale02),
//             3 => Ok(DecimalScale::Scale03),
//             4 => Ok(DecimalScale::Scale04),
//             5 => Ok(DecimalScale::Scale05),
//             6 => Ok(DecimalScale::Scale06),
//             7 => Ok(DecimalScale::Scale07),
//             8 => Ok(DecimalScale::Scale08),
//             9 => Ok(DecimalScale::Scale09),
//             10 => Ok(DecimalScale::Scale10),
//             11 => Ok(DecimalScale::Scale11),
//             12 => Ok(DecimalScale::Scale12),
//             13 => Ok(DecimalScale::Scale13),
//             14 => Ok(DecimalScale::Scale14),
//             15 => Ok(DecimalScale::Scale15),
//             16 => Ok(DecimalScale::Scale16),
//             17 => Ok(DecimalScale::Scale17),
//             18 => Ok(DecimalScale::Scale18),
//             19 => Ok(DecimalScale::Scale19),
//             20 => Ok(DecimalScale::Scale20),
//             21 => Ok(DecimalScale::Scale21),
//             22 => Ok(DecimalScale::Scale22),
//             23 => Ok(DecimalScale::Scale23),
//             24 => Ok(DecimalScale::Scale24),
//             25 => Ok(DecimalScale::Scale25),
//             26 => Ok(DecimalScale::Scale26),
//             27 => Ok(DecimalScale::Scale27),
//             28 => Ok(DecimalScale::Scale28),
//             29 => Ok(DecimalScale::Scale29),
//             30 => Ok(DecimalScale::Scale30),
//             31 => Ok(DecimalScale::Scale31),
//             32 => Ok(DecimalScale::Scale32),
//             33 => Ok(DecimalScale::Scale33),
//             34 => Ok(DecimalScale::Scale34),
//             35 => Ok(DecimalScale::Scale35),
//             36 => Ok(DecimalScale::Scale36),
//             37 => Ok(DecimalScale::Scale37),
//             38 => Ok(DecimalScale::Scale38),
//             _ => panic!("Invalid scale"), // Result: Err("Invalid Decimal Scale")
//         }
//     }
// }

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decimal {
    sign: DecimalSign,
    precision: DecimalPrecision,
    scale: DecimalScale,
    part_0: u32,
    part_1: u32,
    part_2: u32,
    part_3: u32,
}

pub type DecimalParts = (
    DecimalSign,
    DecimalPrecision,
    DecimalScale,
    u32,
    u32,
    u32,
    u32,
);

pub const DECIMAL_ZERO: Decimal = Decimal {
    sign: DecimalSign::Positive,
    precision: DecimalPrecision::Precision38,
    scale: DecimalScale::Scale00,
    part_0: 0,
    part_1: 0,
    part_2: 0,
    part_3: 0,
};

pub const DECIMAL_ZERO_STR: &str = "00000000000000000000000000000000000000";

pub const DECIMAL_MIN: Decimal = Decimal {
    // (-) FF FF FF FF : 3F 22 8A 09 : 7A C4 86 5A : A8 4C 3B 4B
    sign: DecimalSign::Negative,
    precision: DecimalPrecision::Precision38,
    scale: DecimalScale::Scale38,
    part_0: 0xFFFFFFFF,
    part_1: 0x098A223F,
    part_2: 0x5A86C47A,
    part_3: 0x4B3B4CA8,
};
pub const DECIMAL_MIN_STR: &str = "-99999999999999999999999999999999999999";

pub const DECIMAL_MAX: Decimal = Decimal {
    // (+) FF FF FF FF : 3F 22 8A 09 : 7A C4 86 5A : A8 4C 3B 4B
    sign: DecimalSign::Positive,
    precision: DecimalPrecision::Precision38,
    scale: DecimalScale::Scale00,
    part_0: 0xFFFFFFFF,
    part_1: 0x098A223F,
    part_2: 0x5A86C47A,
    part_3: 0x4B3B4CA8,
};

pub const DECIMAL_MAX_STR: &str = "99999999999999999999999999999999999999";

pub const DECIMAL_01: &[Decimal] = &[
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale01, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale02, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale03, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale04, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale05, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale06, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale07, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale08, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale09, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale10, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale11, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale12, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale13, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale14, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale15, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale16, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale17, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale18, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale19, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale20, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale21, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale22, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale23, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale24, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale25, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale26, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale27, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale28, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale29, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale30, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale31, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale32, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale33, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale34, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale35, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale36, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale37, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,},      
];

pub const DECIMAL_10: &[Decimal] = &[
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x0000000A,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale01, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000064,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale02, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000003E8,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale03, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00002710,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale04, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000186A0,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale05, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000F4240,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale06, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00989680,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale07, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x05F5E100,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale08, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x3B9ACA00,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale09, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000002, part_0: 0x540BE400,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale10, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000017, part_0: 0x4876E800,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale11, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x000000E8, part_0: 0xD4A51000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale12, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000918, part_0: 0x4E72A000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale13, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00005AF3, part_0: 0x107A4000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale14, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00038D7E, part_0: 0xA4C68000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale15, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x002386F2, part_0: 0x6FC10000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale16, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x01634578, part_0: 0x5D8A0000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale17, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x0DE0B6B3, part_0: 0xA7640000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale18, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x8AC72304, part_0: 0x89E80000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale19, part_3: 0x00000000, part_2: 0x00000005, part_1: 0x6BC75E2D, part_0: 0x63100000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale20, part_3: 0x00000000, part_2: 0x00000036, part_1: 0x35C9ADC5, part_0: 0xDEA00000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale21, part_3: 0x00000000, part_2: 0x0000021E, part_1: 0x19E0C9BA, part_0: 0xB2400000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale22, part_3: 0x00000000, part_2: 0x0000152D, part_1: 0x02C7E14A, part_0: 0xF6800000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale23, part_3: 0x00000000, part_2: 0x0000D3C2, part_1: 0x1BCECCED, part_0: 0xA1000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale24, part_3: 0x00000000, part_2: 0x00084595, part_1: 0x16140148, part_0: 0x4A000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale25, part_3: 0x00000000, part_2: 0x0052B7D2, part_1: 0xDCC80CD2, part_0: 0xE4000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale26, part_3: 0x00000000, part_2: 0x033B2E3C, part_1: 0x9FD0803C, part_0: 0xE8000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale27, part_3: 0x00000000, part_2: 0x204FCE5E, part_1: 0x3E250261, part_0: 0x10000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale28, part_3: 0x00000001, part_2: 0x431E0FAE, part_1: 0x6D7217CA, part_0: 0xA0000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale29, part_3: 0x0000000C, part_2: 0x9F2C9CD0, part_1: 0x4674EDEA, part_0: 0x40000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale30, part_3: 0x0000007E, part_2: 0x37BE2022, part_1: 0xC0914B26, part_0: 0x80000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale31, part_3: 0x000004EE, part_2: 0x2D6D415B, part_1: 0x85ACEF81, part_0: 0x00000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale32, part_3: 0x0000314D, part_2: 0xC6448D93, part_1: 0x38C15B0A, part_0: 0x00000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale33, part_3: 0x0001ED09, part_2: 0xBEAD87C0, part_1: 0x378D8E64, part_0: 0x00000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale34, part_3: 0x00134261, part_2: 0x72C74D82, part_1: 0x2B878FE8, part_0: 0x00000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale35, part_3: 0x00C097CE, part_2: 0x7BC90715, part_1: 0xB34B9F10, part_0: 0x00000000,},
    Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale36, part_3: 0x0785EE10, part_2: 0xD5DA46D9, part_1: 0x00F436A0, part_0: 0x00000000,},
];

pub const DECIMAL_RUN_ONE: &[(&str, Decimal)] = &[
    ("1", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000001,}),
    ("10", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x0000000A,}),
    ("100", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00000064,}),
    ("1000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000003E8,}),
    ("10000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00002710,}),
    ("100000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000186A0,}),
    ("1000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x000F4240,}),
    ("10000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x00989680,}),
    ("100000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x05F5E100,}),
    ("1000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000000, part_0: 0x3B9ACA00,}),
    ("10000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000002, part_0: 0x540BE400,}),
    ("100000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000017, part_0: 0x4876E800,}),
    ("1000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x000000E8, part_0: 0xD4A51000,}),
    ("10000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00000918, part_0: 0x4E72A000,}),
    ("100000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00005AF3, part_0: 0x107A4000,}),
    ("1000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x00038D7E, part_0: 0xA4C68000,}),
    ("10000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x002386F2, part_0: 0x6FC10000,}),
    ("100000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x01634578, part_0: 0x5D8A0000,}),
    ("1000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x0DE0B6B3, part_0: 0xA7640000,}),
    ("10000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000000, part_1: 0x8AC72304, part_0: 0x89E80000,}),
    ("100000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000005, part_1: 0x6BC75E2D, part_0: 0x63100000,}),
    ("1000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00000036, part_1: 0x35C9ADC5, part_0: 0xDEA00000,}),
    ("10000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x0000021E, part_1: 0x19E0C9BA, part_0: 0xB2400000,}),
    ("100000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x0000152D, part_1: 0x02C7E14A, part_0: 0xF6800000,}),
    ("1000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x0000D3C2, part_1: 0x1BCECCED, part_0: 0xA1000000,}),
    ("10000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x00084595, part_1: 0x16140148, part_0: 0x4A000000,}),
    ("100000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x0052B7D2, part_1: 0xDCC80CD2, part_0: 0xE4000000,}),
    ("1000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x033B2E3C, part_1: 0x9FD0803C, part_0: 0xE8000000,}),
    ("10000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000000, part_2: 0x204FCE5E, part_1: 0x3E250261, part_0: 0x10000000,}),
    ("100000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00000001, part_2: 0x431E0FAE, part_1: 0x6D7217CA, part_0: 0xA0000000,}),  
    ("1000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x0000000C, part_2: 0x9F2C9CD0, part_1: 0x4674EDEA, part_0: 0x40000000,}),
    ("10000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x0000007E, part_2: 0x37BE2022, part_1: 0xC0914B26, part_0: 0x80000000,}),
    ("100000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x000004EE, part_2: 0x2D6D415B, part_1: 0x85ACEF81, part_0: 0x00000000,}),
    ("1000000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x0000314D, part_2: 0xC6448D93, part_1: 0x38C15B0A, part_0: 0x00000000,}),
    ("10000000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x0001ED09, part_2: 0xBEAD87C0, part_1: 0x378D8E64, part_0: 0x00000000,}),
    ("100000000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00134261, part_2: 0x72C74D82, part_1: 0x2B878FE8, part_0: 0x00000000,}),
    ("1000000000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x00C097CE, part_2: 0x7BC90715, part_1: 0xB34B9F10, part_0: 0x00000000,}),
    ("10000000000000000000000000000000000000", Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, part_3: 0x0785EE10, part_2: 0xD5DA46D9, part_1: 0x00F436A0, part_0: 0x00000000,}),
];

pub const DECIMAL_DIGITS: &[&[Decimal]] = &[
    &[
        // 0
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000001,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000002,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000003,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000004,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000005,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000006,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000007,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000008,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000009,
        },
    ],
    &[
        // 1
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000000A,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000014,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000001E,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000028,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000032,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000003C,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000046,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000050,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000005A,
        },
    ],
    &[
        // 2
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000064,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000000C8,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000012C,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000190,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000001F4,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000258,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000002BC,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000320,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000384,
        },
    ],
    &[
        // 3
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000003E8,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000007D0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000BB8,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000FA0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00001388,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00001770,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00001B58,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00001F40,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00002328,
        },
    ],
    &[
        // 4
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00002710,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00004E20,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00007530,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00009C40,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000C350,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0000EA60,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00011170,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00013880,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00015F90,
        },
    ],
    &[
        // 5
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000186A0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00030D40,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000493E0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00061A80,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0007A120,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000927C0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000AAE60,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000C3500,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000DBBA0,
        },
    ],
    &[
        // 6
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x000F4240,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x001E8480,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x002DC6C0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x003D0900,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x004C4B40,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x005B8D80,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x006ACFC0,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x007A1200,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00895440,
        },
    ],
    &[
        // 7
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00989680,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x01312D00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x01C9C380,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x02625A00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x02FAF080,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x03938700,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x042C1D80,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x04C4B400,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x055D4A80,
        },
    ],
    &[
        // 8
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x05F5E100,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x0BEBC200,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x11E1A300,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x17D78400,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x1DCD6500,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x23C34600,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x29B92700,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x2FAF0800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x35A4E900,
        },
    ],
    &[
        // 9
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x3B9ACA00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x77359400,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0xB2D05E00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0xEE6B2800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000001,
            part_0: 0x2A05F200,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000001,
            part_0: 0x65A0BC00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000001,
            part_0: 0xA13B8600,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000001,
            part_0: 0xDCD65000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000002,
            part_0: 0x18711A00,
        },
    ],
    &[
        // 10
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000002,
            part_0: 0x540BE400,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000004,
            part_0: 0xA817C800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000006,
            part_0: 0xFC23AC00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000009,
            part_0: 0x502F9000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000000B,
            part_0: 0xA43B7400,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000000D,
            part_0: 0xF8475800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000010,
            part_0: 0x4C533C00,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000012,
            part_0: 0xA05F2000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000014,
            part_0: 0xF46B0400,
        },
    ],
    &[
        // 11
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000017,
            part_0: 0x4876E800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000002E,
            part_0: 0x90EDD000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000045,
            part_0: 0xD964B800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000005D,
            part_0: 0x21DBA000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000074,
            part_0: 0x6A528800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000008B,
            part_0: 0xB2C97000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000000A2,
            part_0: 0xFB405800,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000000BA,
            part_0: 0x43B74000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000000D1,
            part_0: 0x8C2E2800,
        },
    ],
    &[
        // 12
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000000E8,
            part_0: 0xD4A51000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000001D1,
            part_0: 0xA94A2000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000002BA,
            part_0: 0x7DEF3000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000003A3,
            part_0: 0x52944000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000048C,
            part_0: 0x27395000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000574,
            part_0: 0xFBDE6000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000065D,
            part_0: 0xD0837000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000746,
            part_0: 0xA5288000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000082F,
            part_0: 0x79CD9000,
        },
    ],
    &[
        // 13
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000918,
            part_0: 0x4E72A000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00001230,
            part_0: 0x9CE54000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00001B48,
            part_0: 0xEB57E000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00002461,
            part_0: 0x39CA8000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00002D79,
            part_0: 0x883D2000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00003691,
            part_0: 0xD6AFC000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00003FAA,
            part_0: 0x25226000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000048C2,
            part_0: 0x73950000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000051DA,
            part_0: 0xC207A000,
        },
    ],
    &[
        // 14
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00005AF3,
            part_0: 0x107A4000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0000B5E6,
            part_0: 0x20F48000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000110D9,
            part_0: 0x316EC000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00016BCC,
            part_0: 0x41E90000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0001C6BF,
            part_0: 0x52634000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000221B2,
            part_0: 0x62DD8000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00027CA5,
            part_0: 0x7357C000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0002D798,
            part_0: 0x83D20000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0003328B,
            part_0: 0x944C4000,
        },
    ],
    &[
        // 15
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00038D7E,
            part_0: 0xA4C68000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00071AFD,
            part_0: 0x498D0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000AA87B,
            part_0: 0xEE538000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x000E35FA,
            part_0: 0x931A0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0011C379,
            part_0: 0x37E08000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x001550F7,
            part_0: 0xDCA70000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0018DE76,
            part_0: 0x816D8000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x001C6BF5,
            part_0: 0x26340000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x001FF973,
            part_0: 0xCAFA8000,
        },
    ],
    &[
        // 16
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x002386F2,
            part_0: 0x6FC10000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00470DE4,
            part_0: 0xDF820000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x006A94D7,
            part_0: 0x4F430000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x008E1BC9,
            part_0: 0xBF040000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00B1A2BC,
            part_0: 0x2EC50000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00D529AE,
            part_0: 0x9E860000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00F8B0A1,
            part_0: 0x0E470000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x011C3793,
            part_0: 0x7E080000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x013FBE85,
            part_0: 0xEDC90000,
        },
    ],
    &[
        // 17
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x01634578,
            part_0: 0x5D8A0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x02C68AF0,
            part_0: 0xBB140000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0429D069,
            part_0: 0x189E0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x058D15E1,
            part_0: 0x76280000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x06F05B59,
            part_0: 0xD3B20000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0853A0D2,
            part_0: 0x313C0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x09B6E64A,
            part_0: 0x8EC60000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0B1A2BC2,
            part_0: 0xEC500000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0C7D713B,
            part_0: 0x49DA0000,
        },
    ],
    &[
        // 18
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x0DE0B6B3,
            part_0: 0xA7640000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x1BC16D67,
            part_0: 0x4EC80000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x29A2241A,
            part_0: 0xF62C0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x3782DACE,
            part_0: 0x9D900000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x45639182,
            part_0: 0x44F40000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x53444835,
            part_0: 0xEC580000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x6124FEE9,
            part_0: 0x93BC0000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x6F05B59D,
            part_0: 0x3B200000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x7CE66C50,
            part_0: 0xE2840000,
        },
    ],
    &[
        // 19
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x8AC72304,
            part_0: 0x89E80000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000001,
            part_1: 0x158E4609,
            part_0: 0x13D00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000001,
            part_1: 0xA055690D,
            part_0: 0x9DB80000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000002,
            part_1: 0x2B1C8C12,
            part_0: 0x27A00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000002,
            part_1: 0xB5E3AF16,
            part_0: 0xB1880000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000003,
            part_1: 0x40AAD21B,
            part_0: 0x3B700000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000003,
            part_1: 0xCB71F51F,
            part_0: 0xC5580000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000004,
            part_1: 0x56391824,
            part_0: 0x4F400000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000004,
            part_1: 0xE1003B28,
            part_0: 0xD9280000,
        },
    ],
    &[
        // 20
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000005,
            part_1: 0x6BC75E2D,
            part_0: 0x63100000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000000A,
            part_1: 0xD78EBC5A,
            part_0: 0xC6200000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000010,
            part_1: 0x43561A88,
            part_0: 0x29300000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000015,
            part_1: 0xAF1D78B5,
            part_0: 0x8C400000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000001B,
            part_1: 0x1AE4D6E2,
            part_0: 0xEF500000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000020,
            part_1: 0x86AC3510,
            part_0: 0x52600000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000025,
            part_1: 0xF273933D,
            part_0: 0xB5700000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000002B,
            part_1: 0x5E3AF16B,
            part_0: 0x18800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000030,
            part_1: 0xCA024F98,
            part_0: 0x7B900000,
        },
    ],
    &[
        // 21
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000036,
            part_1: 0x35C9ADC5,
            part_0: 0xDEA00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000006C,
            part_1: 0x6B935B8B,
            part_0: 0xBD400000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000000A2,
            part_1: 0xA15D0951,
            part_0: 0x9BE00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000000D8,
            part_1: 0xD726B717,
            part_0: 0x7A800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000010F,
            part_1: 0x0CF064DD,
            part_0: 0x59200000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000145,
            part_1: 0x42BA12A3,
            part_0: 0x37C00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000017B,
            part_1: 0x7883C069,
            part_0: 0x16600000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000001B1,
            part_1: 0xAE4D6E2E,
            part_0: 0xF5000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000001E7,
            part_1: 0xE4171BF4,
            part_0: 0xD3A00000,
        },
    ],
    &[
        // 22
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000021E,
            part_1: 0x19E0C9BA,
            part_0: 0xB2400000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000043C,
            part_1: 0x33C19375,
            part_0: 0x64800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000065A,
            part_1: 0x4DA25D30,
            part_0: 0x16C00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000878,
            part_1: 0x678326EA,
            part_0: 0xC9000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000A96,
            part_1: 0x8163F0A5,
            part_0: 0x7B400000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000CB4,
            part_1: 0x9B44BA60,
            part_0: 0x2D800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000ED2,
            part_1: 0xB525841A,
            part_0: 0xDFC00000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000010F0,
            part_1: 0xCF064DD5,
            part_0: 0x92000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000130E,
            part_1: 0xE8E71790,
            part_0: 0x44400000,
        },
    ],
    &[
        // 23
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000152D,
            part_1: 0x02C7E14A,
            part_0: 0xF6800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00002A5A,
            part_1: 0x058FC295,
            part_0: 0xED000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00003F87,
            part_1: 0x0857A3E0,
            part_0: 0xE3800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000054B4,
            part_1: 0x0B1F852B,
            part_0: 0xDA000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000069E1,
            part_1: 0x0DE76676,
            part_0: 0xD0800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00007F0E,
            part_1: 0x10AF47C1,
            part_0: 0xC7000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000943B,
            part_1: 0x1377290C,
            part_0: 0xBD800000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000A968,
            part_1: 0x163F0A57,
            part_0: 0xB4000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000BE95,
            part_1: 0x1906EBA2,
            part_0: 0xAA800000,
        },
    ],
    &[
        // 24
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0000D3C2,
            part_1: 0x1BCECCED,
            part_0: 0xA1000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0001A784,
            part_1: 0x379D99DB,
            part_0: 0x42000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00027B46,
            part_1: 0x536C66C8,
            part_0: 0xE3000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00034F08,
            part_1: 0x6F3B33B6,
            part_0: 0x84000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000422CA,
            part_1: 0x8B0A00A4,
            part_0: 0x25000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0004F68C,
            part_1: 0xA6D8CD91,
            part_0: 0xC6000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0005CA4E,
            part_1: 0xC2A79A7F,
            part_0: 0x67000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00069E10,
            part_1: 0xDE76676D,
            part_0: 0x08000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x000771D2,
            part_1: 0xFA45345A,
            part_0: 0xA9000000,
        },
    ],
    &[
        // 25
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00084595,
            part_1: 0x16140148,
            part_0: 0x4A000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00108B2A,
            part_1: 0x2C280290,
            part_0: 0x94000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0018D0BF,
            part_1: 0x423C03D8,
            part_0: 0xDE000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00211654,
            part_1: 0x58500521,
            part_0: 0x28000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00295BE9,
            part_1: 0x6E640669,
            part_0: 0x72000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0031A17E,
            part_1: 0x847807B1,
            part_0: 0xBC000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0039E713,
            part_1: 0x9A8C08FA,
            part_0: 0x06000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00422CA8,
            part_1: 0xB0A00A42,
            part_0: 0x50000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x004A723D,
            part_1: 0xC6B40B8A,
            part_0: 0x9A000000,
        },
    ],
    &[
        // 26
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0052B7D2,
            part_1: 0xDCC80CD2,
            part_0: 0xE4000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00A56FA5,
            part_1: 0xB99019A5,
            part_0: 0xC8000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00F82778,
            part_1: 0x96582678,
            part_0: 0xAC000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x014ADF4B,
            part_1: 0x7320334B,
            part_0: 0x90000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x019D971E,
            part_1: 0x4FE8401E,
            part_0: 0x74000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x01F04EF1,
            part_1: 0x2CB04CF1,
            part_0: 0x58000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x024306C4,
            part_1: 0x097859C4,
            part_0: 0x3C000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0295BE96,
            part_1: 0xE6406697,
            part_0: 0x20000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x02E87669,
            part_1: 0xC308736A,
            part_0: 0x04000000,
        },
    ],
    &[
        // 27
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x033B2E3C,
            part_1: 0x9FD0803C,
            part_0: 0xE8000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x06765C79,
            part_1: 0x3FA10079,
            part_0: 0xD0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x09B18AB5,
            part_1: 0xDF7180B6,
            part_0: 0xB8000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x0CECB8F2,
            part_1: 0x7F4200F3,
            part_0: 0xA0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x1027E72F,
            part_1: 0x1F128130,
            part_0: 0x88000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x1363156B,
            part_1: 0xBEE3016D,
            part_0: 0x70000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x169E43A8,
            part_1: 0x5EB381AA,
            part_0: 0x58000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x19D971E4,
            part_1: 0xFE8401E7,
            part_0: 0x40000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x1D14A021,
            part_1: 0x9E548224,
            part_0: 0x28000000,
        },
    ],
    &[
        // 28
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x204FCE5E,
            part_1: 0x3E250261,
            part_0: 0x10000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x409F9CBC,
            part_1: 0x7C4A04C2,
            part_0: 0x20000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x60EF6B1A,
            part_1: 0xBA6F0723,
            part_0: 0x30000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x813F3978,
            part_1: 0xF8940984,
            part_0: 0x40000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0xA18F07D7,
            part_1: 0x36B90BE5,
            part_0: 0x50000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0xC1DED635,
            part_1: 0x74DE0E46,
            part_0: 0x60000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0xE22EA493,
            part_1: 0xB30310A7,
            part_0: 0x70000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000001,
            part_2: 0x027E72F1,
            part_1: 0xF1281308,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000001,
            part_2: 0x22CE4150,
            part_1: 0x2F4D1569,
            part_0: 0x90000000,
        },
    ],
    &[
        // 29
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000001,
            part_2: 0x431E0FAE,
            part_1: 0x6D7217CA,
            part_0: 0xA0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000002,
            part_2: 0x863C1F5C,
            part_1: 0xDAE42F95,
            part_0: 0x40000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000003,
            part_2: 0xC95A2F0B,
            part_1: 0x4856475F,
            part_0: 0xE0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000005,
            part_2: 0x0C783EB9,
            part_1: 0xB5C85F2A,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000006,
            part_2: 0x4F964E68,
            part_1: 0x233A76F5,
            part_0: 0x20000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000007,
            part_2: 0x92B45E16,
            part_1: 0x90AC8EBF,
            part_0: 0xC0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000008,
            part_2: 0xD5D26DC4,
            part_1: 0xFE1EA68A,
            part_0: 0x60000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000000A,
            part_2: 0x18F07D73,
            part_1: 0x6B90BE55,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000000B,
            part_2: 0x5C0E8D21,
            part_1: 0xD902D61F,
            part_0: 0xA0000000,
        },
    ],
    &[
        // 30
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000000C,
            part_2: 0x9F2C9CD0,
            part_1: 0x4674EDEA,
            part_0: 0x40000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000019,
            part_2: 0x3E5939A0,
            part_1: 0x8CE9DBD4,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000025,
            part_2: 0xDD85D670,
            part_1: 0xD35EC9BE,
            part_0: 0xC0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000032,
            part_2: 0x7CB27341,
            part_1: 0x19D3B7A9,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000003F,
            part_2: 0x1BDF1011,
            part_1: 0x6048A593,
            part_0: 0x40000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000004B,
            part_2: 0xBB0BACE1,
            part_1: 0xA6BD937D,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000058,
            part_2: 0x5A3849B1,
            part_1: 0xED328167,
            part_0: 0xC0000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000064,
            part_2: 0xF964E682,
            part_1: 0x33A76F52,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000071,
            part_2: 0x98918352,
            part_1: 0x7A1C5D3C,
            part_0: 0x40000000,
        },
    ],
    &[
        // 31
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000007E,
            part_2: 0x37BE2022,
            part_1: 0xC0914B26,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000000FC,
            part_2: 0x6F7C4045,
            part_1: 0x8122964D,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000017A,
            part_2: 0xA73A6068,
            part_1: 0x41B3E173,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000001F8,
            part_2: 0xDEF8808B,
            part_1: 0x02452C9A,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000277,
            part_2: 0x16B6A0AD,
            part_1: 0xC2D677C0,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000002F5,
            part_2: 0x4E74C0D0,
            part_1: 0x8367C2E7,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000373,
            part_2: 0x8632E0F3,
            part_1: 0x43F90E0D,
            part_0: 0x80000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000003F1,
            part_2: 0xBDF10116,
            part_1: 0x048A5934,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000046F,
            part_2: 0xF5AF2138,
            part_1: 0xC51BA45A,
            part_0: 0x80000000,
        },
    ],
    &[
        // 32
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000004EE,
            part_2: 0x2D6D415B,
            part_1: 0x85ACEF81,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000009DC,
            part_2: 0x5ADA82B7,
            part_1: 0x0B59DF02,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000ECA,
            part_2: 0x8847C412,
            part_1: 0x9106CE83,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000013B8,
            part_2: 0xB5B5056E,
            part_1: 0x16B3BE04,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000018A6,
            part_2: 0xE32246C9,
            part_1: 0x9C60AD85,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00001D95,
            part_2: 0x108F8825,
            part_1: 0x220D9D06,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00002283,
            part_2: 0x3DFCC980,
            part_1: 0xA7BA8C87,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00002771,
            part_2: 0x6B6A0ADC,
            part_1: 0x2D677C08,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00002C5F,
            part_2: 0x98D74C37,
            part_1: 0xB3146B89,
            part_0: 0x00000000,
        },
    ],
    &[
        // 33
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000314D,
            part_2: 0xC6448D93,
            part_1: 0x38C15B0A,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000629B,
            part_2: 0x8C891B26,
            part_1: 0x7182B614,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000093E9,
            part_2: 0x52CDA8B9,
            part_1: 0xAA44111E,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000C537,
            part_2: 0x1912364C,
            part_1: 0xE3056C28,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0000F684,
            part_2: 0xDF56C3E0,
            part_1: 0x1BC6C732,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000127D2,
            part_2: 0xA59B5173,
            part_1: 0x5488223C,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00015920,
            part_2: 0x6BDFDF06,
            part_1: 0x8D497D46,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00018A6E,
            part_2: 0x32246C99,
            part_1: 0xC60AD850,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0001BBBB,
            part_2: 0xF868FA2C,
            part_1: 0xFECC335A,
            part_0: 0x00000000,
        },
    ],
    &[
        // 34
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0001ED09,
            part_2: 0xBEAD87C0,
            part_1: 0x378D8E64,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0003DA13,
            part_2: 0x7D5B0F80,
            part_1: 0x6F1B1CC8,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0005C71D,
            part_2: 0x3C089740,
            part_1: 0xA6A8AB2C,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0007B426,
            part_2: 0xFAB61F00,
            part_1: 0xDE363990,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0009A130,
            part_2: 0xB963A6C1,
            part_1: 0x15C3C7F4,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000B8E3A,
            part_2: 0x78112E81,
            part_1: 0x4D515658,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000D7B44,
            part_2: 0x36BEB641,
            part_1: 0x84DEE4BC,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x000F684D,
            part_2: 0xF56C3E01,
            part_1: 0xBC6C7320,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00115557,
            part_2: 0xB419C5C1,
            part_1: 0xF3FA0184,
            part_0: 0x00000000,
        },
    ],
    &[
        // 35
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00134261,
            part_2: 0x72C74D82,
            part_1: 0x2B878FE8,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x002684C2,
            part_2: 0xE58E9B04,
            part_1: 0x570F1FD0,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0039C724,
            part_2: 0x5855E886,
            part_1: 0x8296AFB8,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x004D0985,
            part_2: 0xCB1D3608,
            part_1: 0xAE1E3FA0,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00604BE7,
            part_2: 0x3DE4838A,
            part_1: 0xD9A5CF88,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00738E48,
            part_2: 0xB0ABD10D,
            part_1: 0x052D5F70,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0086D0AA,
            part_2: 0x23731E8F,
            part_1: 0x30B4EF58,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x009A130B,
            part_2: 0x963A6C11,
            part_1: 0x5C3C7F40,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00AD556D,
            part_2: 0x0901B993,
            part_1: 0x87C40F28,
            part_0: 0x00000000,
        },
    ],
    &[
        // 36
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00C097CE,
            part_2: 0x7BC90715,
            part_1: 0xB34B9F10,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x01812F9C,
            part_2: 0xF7920E2B,
            part_1: 0x66973E20,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0241C76B,
            part_2: 0x735B1541,
            part_1: 0x19E2DD30,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x03025F39,
            part_2: 0xEF241C56,
            part_1: 0xCD2E7C40,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x03C2F708,
            part_2: 0x6AED236C,
            part_1: 0x807A1B50,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x04838ED6,
            part_2: 0xE6B62A82,
            part_1: 0x33C5BA60,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x054426A5,
            part_2: 0x627F3197,
            part_1: 0xE7115970,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0604BE73,
            part_2: 0xDE4838AD,
            part_1: 0x9A5CF880,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x06C55642,
            part_2: 0x5A113FC3,
            part_1: 0x4DA89790,
            part_0: 0x00000000,
        },
    ],
    &[
        // 37
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x00000000,
            part_2: 0x00000000,
            part_1: 0x00000000,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0785EE10,
            part_2: 0xD5DA46D9,
            part_1: 0x00F436A0,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x0F0BDC21,
            part_2: 0xABB48DB2,
            part_1: 0x01E86D40,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x1691CA32,
            part_2: 0x818ED48B,
            part_1: 0x02DCA3E0,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x1E17B843,
            part_2: 0x57691B64,
            part_1: 0x03D0DA80,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x259DA654,
            part_2: 0x2D43623D,
            part_1: 0x04C51120,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x2D239465,
            part_2: 0x031DA916,
            part_1: 0x05B947C0,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x34A98275,
            part_2: 0xD8F7EFEF,
            part_1: 0x06AD7E60,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x3C2F7086,
            part_2: 0xAED236C8,
            part_1: 0x07A1B500,
            part_0: 0x00000000,
        },
        Decimal {
            sign: DecimalSign::Positive,
            precision: DecimalPrecision::Precision38,
            scale: DecimalScale::Scale00,
            part_3: 0x43B55E97,
            part_2: 0x84AC7DA1,
            part_1: 0x0895EBA0,
            part_0: 0x00000000,
        },
    ],
];

#[wasm_bindgen]
pub enum DecimalComparison {
    GT,
    EQ,
    LT,
}

//type DecimalResult<T> = std::result::Result<T, DecimalError>;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct DecimalError;

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid string")
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.part_0 == 0 && self.part_1 == 0 && self.part_2 == 0 && self.part_3 == 0 {
            write!(f, "0")
        } else {
            let mut origin = Decimal::new(DecimalSign::Positive, self.precision, DecimalScale::Scale00, self.part_0, self.part_1, self.part_2, self.part_3);
            let mut result = String::from("");
            let mut run_10_index = DECIMAL_RUN_ONE.len() - 1;
            match (origin.compare_modulo(&DECIMAL_MAX), origin.compare_modulo(&DECIMAL_RUN_ONE[run_10_index - 1].1)) {
                (_, DecimalComparison::LT) => (),
                (_, DecimalComparison::EQ) => {
                    let result0 = &DECIMAL_RUN_ONE[run_10_index - 1].0[0..DECIMAL_RUN_ONE[run_10_index - 1].0.len() - origin.scale as usize];
                    let result1 = &DECIMAL_RUN_ONE[run_10_index - 1].0[DECIMAL_RUN_ONE[run_10_index - 1].0.len() - origin.scale as usize..DECIMAL_RUN_ONE[run_10_index - 1].0.len()];
                    return write!(f, "{}.{}", result0, result1);
                }
                (DecimalComparison::LT, DecimalComparison::GT) => {
                    let mut digit_counter = 0;
                    loop {
                        let remainder = origin.sub(&DECIMAL_RUN_ONE[DECIMAL_RUN_ONE.len() - 1].1);
                        if remainder.sign == DecimalSign::Negative {
                            result.push_str(&digit_counter.to_string());
                            break;
                        } else {
                            origin = remainder;
                            digit_counter += 1;
                        }
                    }
                },
                (DecimalComparison::EQ, _) => {
                    let result0 = &DECIMAL_MAX_STR[0..DECIMAL_MAX_STR.len() - origin.scale as usize];
                    let result1 = &DECIMAL_MAX_STR[DECIMAL_MAX_STR.len() - origin.scale as usize..DECIMAL_MAX_STR.len()];
                    return write!(f, "{}.{}", result0, result1);
                },
                (DecimalComparison::GT, _) => panic!("Arithmetic overflow"),
            };
            let mut leading_zeros = true;
            loop {
                match DECIMAL_RUN_ONE[run_10_index].1.compare(&origin) {
                    DecimalComparison::LT => panic!("Ambiquous comparing"),
                    DecimalComparison::EQ => {
                        result.push_str(DECIMAL_RUN_ONE[run_10_index].0);
                        break;
                    }, //result.push_str(DECIMAL_RUN_ONE[run_10_index - 1].0),
                    DecimalComparison::GT => match DECIMAL_RUN_ONE[run_10_index - 1].1.compare(&origin) {
                        DecimalComparison::LT => {
                            let mut digit_counter = 0;
                            loop {
                                let remainder = origin.sub(&DECIMAL_RUN_ONE[run_10_index - 1].1);
                                if remainder.sign == DecimalSign::Negative {
                                    break;
                                } else {
                                    origin = remainder;
                                    digit_counter += 1;
                                }
                            }
                            result.push_str(&digit_counter.to_string());
                            leading_zeros = false;
                            run_10_index -= 1;
                        },
                        DecimalComparison::EQ => {
                            result.push_str(DECIMAL_RUN_ONE[run_10_index - 1].0);
                            break;
                        }, //panic!("Algorithm error"),
                        DecimalComparison::GT => {
                            if !leading_zeros {
                                result.push('0');
                            }
                            run_10_index -= 1;
                        },
                    }
                }
                if run_10_index == 0 {
                    break;
                }
            }
            let scale_diff = self.scale as i32 - result.len() as i32;
            match scale_diff.cmp(&0) {
                Ordering::Greater => {
                    let mut lead_zeros = scale_diff;
                    while lead_zeros > 0 {
                        result.insert(0, '0');
                        lead_zeros -= 1;
                    }
                    result.insert(0, '.');
                    result.insert(0, '0');
                },
                Ordering::Equal => {
                    result.insert(0, '.');
                    result.insert(0, '0');
                },
                Ordering::Less => {
                    result.insert(scale_diff.unsigned_abs() as usize, '.');
                    if self.scale == DecimalScale::Scale00 {
                        result.push('0');
                    }
                },
            };
            if self.sign == DecimalSign::Negative {
                result.insert(0, '-');
            }
            write!(f, "{}", result)
        }
    }
}

//#[test]
fn decimal_display_works() {
    let test = Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 77, 0, 0, 0);
    let test_string = test.to_string();
    assert_eq!(test_string, "77.0");

    let test = Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale01, 77, 0, 0, 0);
    let test_string = test.to_string();
    assert_eq!(test_string, "7.7");

    let test = Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale02, 77, 0, 0, 0);
    let test_string = test.to_string();
    assert_eq!(test_string, "0.77");

    let test = Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale03, 77, 0, 0, 0);
    let test_string = test.to_string();
    assert_eq!(test_string, "0.077");
}

//#[test]
fn decimal_parse_works() {
    let test = Decimal::parse("0.11").unwrap();
    println!("Decimal: {} {} {} {} {} {}", test.precision as u32, test.scale as u32, test.part_0, test.part_1, test.part_2, test.part_3);
    assert_eq!(test.precision as u32, 3);
    assert_eq!(test.scale as u32, 2);
    assert_eq!(test.part_0, 11);
    assert_eq!(test.part_1, 0);
    assert_eq!(test.part_2, 0);
    assert_eq!(test.part_3, 0);

    let test = Decimal::parse("1.1").unwrap();
    println!("Decimal: {} {} {} {} {} {}", test.precision as u32, test.scale as u32, test.part_0, test.part_1, test.part_2, test.part_3);
    assert_eq!(test.precision as u32, 2);
    assert_eq!(test.scale as u32, 1);
    assert_eq!(test.part_0, 11);
    assert_eq!(test.part_1, 0);
    assert_eq!(test.part_2, 0);
    assert_eq!(test.part_3, 0);

    let test = Decimal::parse("11").unwrap();
    println!("Decimal: {} {} {} {} {} {}", test.precision as u32, test.scale as u32, test.part_0, test.part_1, test.part_2, test.part_3);
    assert_eq!(test.precision as u32, 2);
    assert_eq!(test.scale as u32, 0);
    assert_eq!(test.part_0, 11);
    assert_eq!(test.part_1, 0);
    assert_eq!(test.part_2, 0);
    assert_eq!(test.part_3, 0);

    let test = Decimal::parse("11.0").unwrap();
    println!("Decimal: {} {} {} {} {} {}", test.precision as u32, test.scale as u32, test.part_0, test.part_1, test.part_2, test.part_3);
    assert_eq!(test.precision as u32, 3);
    assert_eq!(test.scale as u32, 1);
    assert_eq!(test.part_0, 110);
    assert_eq!(test.part_1, 0);
    assert_eq!(test.part_2, 0);
    assert_eq!(test.part_3, 0);
}

//#[test]
fn decimal_add_works() {
    let test1 = Decimal::parse("111").unwrap();
    let test2 = Decimal::parse("11").unwrap();

    let result = test1.add(&test2);
    println!("Sub result Decimal: {} {} {} {} {} {}", result.precision as u32, result.scale as u32, result.part_0, result.part_1, result.part_2, result.part_3);
    assert_eq!(result.precision as u32, 3);
    assert_eq!(result.scale as u32, 0);
    assert_eq!(result.part_0, 122);
    assert_eq!(result.part_1, 0);
    assert_eq!(result.part_2, 0);
    assert_eq!(result.part_3, 0);
}

//#[test]
fn decimal_sub_works() {
    let test1 = Decimal::parse("111").unwrap();
    let test2 = Decimal::parse("11").unwrap();

    let result = test1.sub(&test2);
    println!("Add result Decimal: {} {} {} {} {} {}", result.precision as u32, result.scale as u32, result.part_0, result.part_1, result.part_2, result.part_3);
    assert_eq!(result.precision as u32, 3);
    assert_eq!(result.scale as u32, 0);
    assert_eq!(result.part_0, 100);
    assert_eq!(result.part_1, 0);
    assert_eq!(result.part_2, 0);
    assert_eq!(result.part_3, 0);
}

#[test]
fn decimal_mul_works() {
    let test1 = Decimal::parse("11").unwrap();
    let test2 = Decimal::parse("11").unwrap();

    let result = test1.mul(&test2).unwrap();
    println!("Mul result Decimal: {} {} {} {} {} {}", result.precision as u32, result.scale as u32, result.part_0, result.part_1, result.part_2, result.part_3);
    assert_eq!(result.precision as u32, 38);
    assert_eq!(result.scale as u32, 0);
    assert_eq!(result.part_0, 121);
    assert_eq!(result.part_1, 0);
    assert_eq!(result.part_2, 0);
    assert_eq!(result.part_3, 0);

    let test1 = Decimal::parse("2222222222222").unwrap();
    let test2 = Decimal::parse("1111111111111").unwrap();
    let result = test1.mul(&test2).unwrap(); // 0x26000001 - 62 11 30 CF : 13 3F BC 1E : DC 0A 02 - 0000000000 - 2469135802468641975308642 / 5189426734590595426
    println!("Mul result Decimal: {} {} {} {} {} {}", result.precision as u32, result.scale as u32, result.part_0, result.part_1, result.part_2, result.part_3);
    
}
#[test]
fn decimal_div_works() {
    let test1 = Decimal::parse("2222222222222").unwrap();
    let test2 = Decimal::parse("1111111111111").unwrap();

    let result = test1.div(&test2);
    println!("Div result Decimal: {} {} {} {} {} {}", result.precision as u32, result.scale as u32, result.part_0, result.part_1, result.part_2, result.part_3);
    assert_eq!(result.precision as u32, 38);
    assert_eq!(result.scale as u32, 0);
    assert_eq!(result.part_0, 2);
    assert_eq!(result.part_1, 0);
    assert_eq!(result.part_2, 0);
    assert_eq!(result.part_3, 0);
}

//#[test]
fn decimal_mixed_works() {
    let decimal = Decimal::parse("2000000000002").unwrap(); // 2000000000002 : 0x26000001 - 02 20 4A A9 D1 01 - 00000000000000000000 - 2840207362 465
    println!("String -> Decimal -> String 2: {} {} {} {} {} {}", decimal.precision as u32, decimal.scale as u32, decimal.part_0, decimal.part_1, decimal.part_2, decimal.part_3);
    assert_eq!(decimal.precision as u32, 13);
    assert_eq!(decimal.scale as u32, 0);
    assert_eq!(decimal.part_0, 2840207362);
    assert_eq!(decimal.part_1, 465);
    assert_eq!(decimal.part_2, 0);
    assert_eq!(decimal.part_3, 0);

    let result = decimal.to_string();
    println!("String -> Decimal -> String 2: {}", result);
    assert_eq!(result, "2000000000002.0");

    let decimal = Decimal::parse("9999999999999").unwrap(); // 9999999999999 : 0x26000001 FF 9F 72 4E 18 09 - 00000000000000000000 - 1316134911 2328
    println!("String -> Decimal -> String 2: {} {} {} {} {} {}", decimal.precision as u32, decimal.scale as u32, decimal.part_0, decimal.part_1, decimal.part_2, decimal.part_3);
    assert_eq!(decimal.precision as u32, 13);
    assert_eq!(decimal.scale as u32, 0);
    assert_eq!(decimal.part_0, 1316134911);
    assert_eq!(decimal.part_1, 2328);
    assert_eq!(decimal.part_2, 0);
    assert_eq!(decimal.part_3, 0);

    let result = decimal.to_string();
    println!("String -> Decimal -> String 2: {}", result);
    assert_eq!(result, "9999999999999.0");

}

impl Decimal {
    pub fn new(
        sign: DecimalSign,
        precision: DecimalPrecision,
        scale: DecimalScale,
        part_0: u32,
        part_1: u32,
        part_2: u32,
        part_3: u32,
    ) -> Decimal {
        // Change to Result
        if part_0 == 0 && part_1 == 0 && part_2 == 0 && part_3 == 0 {
            Decimal {
                // fn normalize?
                sign: DecimalSign::Positive,
                precision: DecimalPrecision::Precision38,
                scale: DecimalScale::Scale00,
                part_0,
                part_1,
                part_2,
                part_3,
            }
        } else {
            Decimal {
                sign,
                precision,
                scale,
                part_0,
                part_1,
                part_2,
                part_3,
            }
        }
    }
    pub fn parse(text: &str) -> Result<Decimal, DecimalError> {
        let mut scale: Option<DecimalScale> = None;
        let mut precision: Option<DecimalPrecision> = None;
        let mut sign: Option<DecimalSign> = None;

        let mut result: (bool, u32, u32, u32, u32) = (true, 0, 0, 0, 0);

        for (index, char) in text.chars().rev().enumerate() {
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match precision {
                        None => precision = Some(DecimalPrecision::Precision01),
                        Some(value) => match DecimalPrecision::inc(value) {
                            Ok(value) => precision = Some(value),
                            Err(_) => return Err(DecimalError),
                        },
                    };
                    let digits_precision = DecimalPrecision::try_to_usize(precision.unwrap()).unwrap() - 1;
                    match char {
                        '0' => (),
                        '1' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][1].part_0,
                                    DECIMAL_DIGITS[digits_precision][1].part_1,
                                    DECIMAL_DIGITS[digits_precision][1].part_2,
                                    DECIMAL_DIGITS[digits_precision][1].part_3,
                                ),
                            )
                        },
                        '2' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][2].part_0,
                                    DECIMAL_DIGITS[digits_precision][2].part_1,
                                    DECIMAL_DIGITS[digits_precision][2].part_2,
                                    DECIMAL_DIGITS[digits_precision][2].part_3,
                                ),
                            )
                        },
                        '3' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][3].part_0,
                                    DECIMAL_DIGITS[digits_precision][3].part_1,
                                    DECIMAL_DIGITS[digits_precision][3].part_2,
                                    DECIMAL_DIGITS[digits_precision][3].part_3,
                                ),
                            )
                        },
                        '4' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][4].part_0,
                                    DECIMAL_DIGITS[digits_precision][4].part_1,
                                    DECIMAL_DIGITS[digits_precision][4].part_2,
                                    DECIMAL_DIGITS[digits_precision][4].part_3,
                                ),
                            )
                        },
                        '5' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][5].part_0,
                                    DECIMAL_DIGITS[digits_precision][5].part_1,
                                    DECIMAL_DIGITS[digits_precision][5].part_2,
                                    DECIMAL_DIGITS[digits_precision][5].part_3,
                                ),
                            )
                        },
                        '6' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][6].part_0,
                                    DECIMAL_DIGITS[digits_precision][6].part_1,
                                    DECIMAL_DIGITS[digits_precision][6].part_2,
                                    DECIMAL_DIGITS[digits_precision][6].part_3,
                                ),
                            )
                        },
                        '7' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][7].part_0,
                                    DECIMAL_DIGITS[digits_precision][7].part_1,
                                    DECIMAL_DIGITS[digits_precision][7].part_2,
                                    DECIMAL_DIGITS[digits_precision][7].part_3,
                                ),
                            )
                        },
                        '8' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][8].part_0,
                                    DECIMAL_DIGITS[digits_precision][8].part_1,
                                    DECIMAL_DIGITS[digits_precision][8].part_2,
                                    DECIMAL_DIGITS[digits_precision][8].part_3,
                                ),
                            )
                        },
                        '9' => {
                            result = Decimal::add_inner(
                                (result.1, result.2, result.3, result.4),
                                (
                                    DECIMAL_DIGITS[digits_precision][9].part_0,
                                    DECIMAL_DIGITS[digits_precision][9].part_1,
                                    DECIMAL_DIGITS[digits_precision][9].part_2,
                                    DECIMAL_DIGITS[digits_precision][9].part_3,
                                ),
                            )
                        },
                        _ => panic!("Invalid char"),
                    };
                },
                '.' => match scale {
                    Some(value) => return Err(DecimalError),
                    None => {
                        scale = Some(DecimalScale::try_from_usize(index).unwrap());
                    }
                },
                '+' => match sign {
                    Some(_) => return Err(DecimalError),
                    None => {
                        if index != text.len() - 1 {
                            return Err(DecimalError);
                        }
                        sign = Some(DecimalSign::Positive);
                    }
                },
                '-' => match sign {
                    Some(_) => return Err(DecimalError),
                    None => {
                        if index != text.len() - 1 {
                            return Err(DecimalError);
                        }
                        sign = Some(DecimalSign::Negative);
                    }
                },
                _ => return Err(DecimalError),
            };
        }
        if sign == None {
            sign = Some(DecimalSign::Positive);
        }
        if scale == None {
            scale = Some(DecimalScale::Scale00);
        }
        Ok(Decimal::new(
            sign.unwrap(),
            precision.unwrap(),
            scale.unwrap(),
            result.1,
            result.2,
            result.3,
            result.4,
        ))
    }
    pub fn to_f64(&self) -> f64 {
        todo!()
    }
    pub fn to_parts(&self) -> DecimalParts {
        (
            self.sign,
            self.precision,
            self.scale,
            self.part_0,
            self.part_1,
            self.part_2,
            self.part_3,
        )
    }
    pub fn normalize(&self, scale: DecimalScale) -> Decimal { // See add, sub
        Decimal::new(
            self.sign,
            self.precision,
            scale,
            self.part_0,
            self.part_1,
            self.part_2,
            self.part_3,
        )
    }
    fn _is_overflow(_value: Decimal) -> bool {
        todo!()
    }
    pub fn add(&self, rhs: &Decimal) -> Decimal { // Add normalize
        let (result_sign, result) = match (self.compare_modulo(rhs), self.sign, rhs.sign) {
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Positive) => {
                (DecimalSign::Positive, (false, 0, 0, 0, 0))
            }
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Negative) => {
                (DecimalSign::Positive, (false, 0, 0, 0, 0))
            }
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Positive) => (
                DecimalSign::Negative,
                Decimal::sub_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Negative) => (
                DecimalSign::Positive,
                Decimal::sub_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::sub_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::sub_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
        };
        if result.0 {
            panic!("Overflow"); // Test on overflow ABS(MAX) = 99999999999999999999999999999999999999
        } else {
            Decimal {
                sign: result_sign,
                precision: self.precision,
                scale: self.scale,
                part_0: result.1,
                part_1: result.2,
                part_2: result.3,
                part_3: result.4,
            }
        }
    }
    fn add_inner(
        lhs: (u32, u32, u32, u32),
        rhs: (u32, u32, u32, u32),
    ) -> (bool, u32, u32, u32, u32) {
        let (result_0, carry_0) = lhs.0.carrying_add(rhs.0, false);
        let (result_1, carry_1) = lhs.1.carrying_add(rhs.1, carry_0);
        let (result_2, carry_2) = lhs.2.carrying_add(rhs.2, carry_1);
        let (result_3, carry_3) = lhs.3.carrying_add(rhs.3, carry_2);
        (carry_3, result_0, result_1, result_2, result_3)
    }
    pub fn sub(&self, rhs: &Decimal) -> Decimal { // Add normalize
        let (result_sign, result) = match (self.compare_modulo(rhs), self.sign, rhs.sign) {
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Negative) => {
                (DecimalSign::Negative, (false, 0, 0, 0, 0))
            }
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Negative) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Positive) => {
                (DecimalSign::Positive, (false, 0, 0, 0, 0))
            }

            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::sub_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Positive) => (
                DecimalSign::Negative,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Negative) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::sub_inner(
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                ),
            ),

            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Negative) => (
                DecimalSign::Positive,
                Decimal::sub_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Positive) => (
                DecimalSign::Positive,
                Decimal::add_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Negative) => (
                DecimalSign::Negative,
                Decimal::add_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Positive) => (
                DecimalSign::Negative,
                Decimal::sub_inner(
                    (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3),
                    (self.part_0, self.part_1, self.part_2, self.part_3),
                ),
            ),
        };
        //if result.0 {
            //panic!("Overflow"); // Test on overflow ABS(MAX) = 99999999999999999999999999999999999999
        //} else {
            Decimal {
                sign: result_sign,
                precision: self.precision,
                scale: self.scale,
                part_0: result.1,
                part_1: result.2,
                part_2: result.3,
                part_3: result.4,
            }
        //}
    }
    fn sub_inner(
        lhs: (u32, u32, u32, u32),
        rhs: (u32, u32, u32, u32),
    ) -> (bool, u32, u32, u32, u32) {
        let (result_0, borrowing_0) = lhs.0.borrowing_sub(rhs.0, false);
        let (result_1, borrowing_1) = lhs.1.borrowing_sub(rhs.1, borrowing_0);
        let (result_2, borrowing_2) = lhs.2.borrowing_sub(rhs.2, borrowing_1);
        let (result_3, borrowing_3) = lhs.3.borrowing_sub(rhs.3, borrowing_2);
        (borrowing_3, result_0, result_1, result_2, result_3)
    }
    pub fn mul(&self, rhs: &Decimal) -> Result<Decimal, DecimalError> { // Replace with specific Error
        let (mul_00, carry_00) = self.part_0.carrying_mul(rhs.part_0, 0);
        let (mul_01, carry_01) = self.part_0.carrying_mul(rhs.part_1, carry_00);
        let (mul_02, carry_02) = self.part_0.carrying_mul(rhs.part_2, carry_01);
        let (mul_03, carry_03) = self.part_0.carrying_mul(rhs.part_3, carry_02);

        let (mul_10, carry_10) = self.part_1.carrying_mul(rhs.part_0, 0);
        let (mul_11, carry_11) = self.part_1.carrying_mul(rhs.part_1, carry_10);
        let (mul_12, carry_12) = self.part_1.carrying_mul(rhs.part_2, carry_11);
        let (mul_13, carry_13) = self.part_1.carrying_mul(rhs.part_3, carry_12);

        let (mul_20, carry_20) = self.part_2.carrying_mul(rhs.part_0, 0);
        let (mul_21, carry_21) = self.part_2.carrying_mul(rhs.part_1, carry_20);
        let (mul_22, carry_22) = self.part_2.carrying_mul(rhs.part_2, carry_21);
        let (mul_23, carry_23) = self.part_2.carrying_mul(rhs.part_3, carry_22);

        let (mul_30, carry_30) = self.part_3.carrying_mul(rhs.part_0, 0);
        let (mul_31, carry_31) = self.part_3.carrying_mul(rhs.part_1, carry_30);
        let (mul_32, carry_32) = self.part_3.carrying_mul(rhs.part_2, carry_31);
        let (mul_33, carry_33) = self.part_3.carrying_mul(rhs.part_3, carry_32);

        // 0
        let mul_0 = mul_00;
        
        // 1
        let (mul_1, carry) = mul_01.carrying_add(mul_10, false);

        // 2
        let (temp, carry) = mul_02.carrying_add(mul_11, carry);
        let (mul_2, carry) = temp.carrying_add(mul_20, carry);

        // 3
        let (temp, carry) = mul_03.carrying_add(mul_12, carry);
        let (temp, carry) = temp.carrying_add(mul_21, carry);
        let (mul_3, carry) = temp.carrying_add(mul_30, carry);

        // 4
        let (temp, carry) = mul_13.carrying_add(mul_22, carry);
        let (mul_4, carry) = temp.carrying_add(mul_31, carry);

        // 5
        let (mul_5, carry) = mul_23.carrying_add(mul_32, carry);

        // 6
        let (mul_6, carry) = mul_33.carrying_add(0, carry);

        if carry {
            Err(DecimalError) // Overflow Error
        } else {
            match self.scale.add(rhs.scale) {
                Ok(scale) =>
                    match (self.sign, rhs.sign) {
                        (DecimalSign::Negative, DecimalSign::Negative) => {
                            Ok(Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, scale, mul_0, mul_1, mul_2, mul_3))
                        },
                        (DecimalSign::Negative, DecimalSign::Positive) => {
                            Ok(Decimal::new(DecimalSign::Negative, DecimalPrecision::Precision38, scale, mul_0, mul_1, mul_2, mul_3))
                        },
                        (DecimalSign::Positive, DecimalSign::Negative) => {
                            Ok(Decimal::new(DecimalSign::Negative, DecimalPrecision::Precision38, scale, mul_0, mul_1, mul_2, mul_3))
                        },
                        (DecimalSign::Positive, DecimalSign::Positive) => {
                            Ok(Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, scale, mul_0, mul_1, mul_2, mul_3))
                        },
                    },
                Err(_) => Err(DecimalError),
            }
        }
    }
    pub fn mul_10(&self) -> Result<Decimal, DecimalError> { // ?
        todo!()
        // let scale = DecimalScale::inc(self.scale);
        // match scale {
        //     Ok(scale) => Ok(Decimal::new(
        //         self.sign,
        //         self.precision,
        //         scale,
        //         self.part_0,
        //         self.part_1,
        //         self.part_2,
        //         self.part_3,
        //     )),
        //     Err(_) => Err(DecimalError),
        // }
    }
    pub fn div(&self, rhs: &Decimal) -> Decimal {
        let mut numerator = Decimal::new(DecimalSign::Positive, self.precision, self.scale, self.part_0, self.part_1, self.part_2, self.part_3);
        let mut denominator = Decimal::new(DecimalSign::Positive, rhs.precision, rhs.scale, rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3);
        let mut quotient = Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0);
        let mut denominators: [Decimal; 38] = [
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
            Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 0, 0, 0, 0),
        ];
        let mut denominator_index: usize = 0;
        denominators[denominator_index] = denominator;
        loop {
            let denominator_mul_10 = denominator.mul(&Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 10, 0, 0, 0)).unwrap();
            match numerator.compare_modulo(&denominator_mul_10) {
                DecimalComparison::EQ | DecimalComparison::LT => break,
                DecimalComparison::GT => {
                    denominators[denominator_index] = denominator_mul_10;
                    denominator_index += 1;
                    denominator = denominator_mul_10;
                },
            };
        };
        loop {
            let mut quotient_digit: usize = 0;
            loop { // Calc quotient digit
                let numerator_previous = numerator;
                numerator = numerator.sub(&denominators[denominator_index]);
                match numerator.sign {
                    DecimalSign::Negative => {
                        numerator = numerator_previous;
                        break;
                    },
                    DecimalSign::Positive => quotient_digit += 1,
                    
                }
            }
            quotient = quotient.mul(&Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 10, 0, 0, 0)).unwrap();
            quotient = quotient.add(&DECIMAL_DIGITS[0][quotient_digit]);
            if denominator_index > 0 {
                loop { // Find denominator
                    match denominators[denominator_index].compare(&numerator) {
                        DecimalComparison::LT | DecimalComparison::EQ => break,
                        DecimalComparison::GT => {
                            quotient = quotient.mul(&Decimal::new(DecimalSign::Positive, DecimalPrecision::Precision38, DecimalScale::Scale00, 10, 0, 0, 0)).unwrap();

                        },
                    };
                    if denominator_index == 0 {
                        break;
                    } else {
                        denominator_index -= 1;
                    }
                };
            }
            if denominator_index == 0 {
                break;
            }
        };
        quotient
    }
    pub fn div_10(&self) -> Result<Decimal, DecimalError> { //
        todo!()
        // let scale = DecimalScale::dec(self.scale);
        // match scale {
        //     Ok(scale) => Ok(Decimal::new(
        //         self.sign,
        //         self.precision,
        //         scale,
        //         self.part_0,
        //         self.part_1,
        //         self.part_2,
        //         self.part_3,
        //     )),
        //     Err(_) => Err(DecimalError),
        // }
    }
    pub fn compare(&self, rhs: &Decimal) -> DecimalComparison {
        if self.sign == DecimalSign::Positive && rhs.sign == DecimalSign::Negative {
            return DecimalComparison::GT;
        }
        if self.sign == DecimalSign::Negative && rhs.sign == DecimalSign::Positive {
            return DecimalComparison::LT;
        }
        if self.scale == rhs.scale
            && self.part_0 == rhs.part_0
            && self.part_1 == rhs.part_1
            && self.part_2 == rhs.part_2
            && self.part_3 == rhs.part_3
        {
            DecimalComparison::EQ
        } else if self.scale == rhs.scale {
            if self.part_3 > rhs.part_3 {
                DecimalComparison::GT
            } else if self.part_3 < rhs.part_3 {
                DecimalComparison::LT
            } else if self.part_2 > rhs.part_2 {
                DecimalComparison::GT
            } else if self.part_2 < rhs.part_2 {
                DecimalComparison::LT
            } else if self.part_1 > rhs.part_1 {
                DecimalComparison::GT
            } else if self.part_1 < rhs.part_1 {
                DecimalComparison::LT
            } else if self.part_0 > rhs.part_0 {
                DecimalComparison::GT
            } else if self.part_0 < rhs.part_0 {
                DecimalComparison::LT
            } else {
                DecimalComparison::EQ
            }
        } else {
            let result = self.sub(rhs);
            if result.sign == DecimalSign::Negative {
                DecimalComparison::LT
            } else if result.part_0 == 0
                && result.part_1 == 0
                && result.part_2 == 0
                && result.part_3 == 0
            {
                DecimalComparison::EQ
            } else {
                DecimalComparison::GT
            }
        }
    }
    pub fn compare_modulo(&self, rhs: &Decimal) -> DecimalComparison {
        if self.scale == rhs.scale
            && self.part_0 == rhs.part_0
            && self.part_1 == rhs.part_1
            && self.part_2 == rhs.part_2
            && self.part_3 == rhs.part_3
        {
            DecimalComparison::EQ
        } else if self.scale == rhs.scale {
            if self.part_3 > rhs.part_3 {
                DecimalComparison::GT
            } else if self.part_3 < rhs.part_3 {
                DecimalComparison::LT
            } else if self.part_2 > rhs.part_2 {
                DecimalComparison::GT
            } else if self.part_2 < rhs.part_2 {
                DecimalComparison::LT
            } else if self.part_1 > rhs.part_1 {
                DecimalComparison::GT
            } else if self.part_1 < rhs.part_1 {
                DecimalComparison::LT
            } else if self.part_0 > rhs.part_0 {
                DecimalComparison::GT
            } else if self.part_0 < rhs.part_0 {
                DecimalComparison::LT
            } else {
                DecimalComparison::EQ
            }
        } else {
            let result = self.sub(rhs);
            if result.sign == DecimalSign::Negative {
                DecimalComparison::LT
            } else if result.part_0 == 0
                && result.part_1 == 0
                && result.part_2 == 0
                && result.part_3 == 0
            {
                DecimalComparison::EQ
            } else {
                DecimalComparison::GT
            }
        }
    }
}
