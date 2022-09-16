#![feature(bigint_helper_methods)]
#![feature(const_bigint_helper_methods)]

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-decimal!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalSign {
    Positive = 0,
    Negative = 1,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalPrecision {
    Precision00 = 0,
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
    Precision39 = 39,
}

#[wasm_bindgen]
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
    Scale39 = 39,
}

#[wasm_bindgen]
pub struct Decimal {
    sign: DecimalSign,
    precision: DecimalPrecision,
    scale: DecimalScale,
    part_0: u32,
    part_1: u32,
    part_2: u32,
    part_3: u32,
}

pub type DecimalParts = (DecimalSign, DecimalPrecision, DecimalScale, u32, u32, u32, u32);

pub const DECIMAL_ZERO: Decimal = Decimal {
    sign: DecimalSign::Positive,
    precision: DecimalPrecision::Precision00,
    scale: DecimalScale::Scale00,
    part_0: 0,
    part_1: 0,
    part_2: 0,
    part_3: 0,
};

pub const DECIMAL_MIN: Decimal = Decimal {
    sign: DecimalSign::Negative,
    precision: DecimalPrecision::Precision39,
    scale: DecimalScale::Scale39,
    part_0: 0,
    part_1: 0,
    part_2: 0,
    part_3: 1,
};

pub const DECIMAL_MAX: Decimal = Decimal {
    sign: DecimalSign::Positive,
    precision: DecimalPrecision::Precision39,
    scale: DecimalScale::Scale00,
    part_0: 0xFFFFFFFF,
    part_1: 0xFFFFFFFF,
    part_2: 0xFFFFFFFF,
    part_3: 0xFFFFFFFF,
};

pub enum DecimalComparison {
    GT,
    EQ,
    LT,
}

impl Decimal {
    pub fn new(sign: DecimalSign, precision: DecimalPrecision, scale: DecimalScale, part_0: u32, part_1: u32, part_2: u32, part_3: u32) -> Decimal {
        if part_0 == 0 && part_1 == 0 && part_2 == 0 && part_3 == 0 {
            Decimal { // fn normalize?
                sign: DecimalSign::Positive,
                precision: DecimalPrecision::Precision00,
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
    pub fn parse(text: &str) -> Decimal {
        todo!()
    }
    pub fn to_string(&self) -> String {
        todo!()
    }
    pub fn to_f64(&self) -> f64 {
        todo!()
    }
    pub fn to_parts(&self) -> DecimalParts {
        todo!()
    }
    pub fn normalize(&self, scale: DecimalScale) -> Decimal {
        return DECIMAL_ZERO;
    }
    pub fn add(&self, rhs: &Decimal) -> Decimal {
        let (result_sign, result) = match (self.compare_modulo(rhs), self.sign, rhs.sign) {
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Negative) => (DecimalSign::Negative, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))), 
            (DecimalComparison::EQ, DecimalSign::Negative, DecimalSign::Positive) => (DecimalSign::Positive, (false, 0, 0, 0, 0)), 
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Negative) => (DecimalSign::Positive, (false, 0, 0, 0, 0)), 
            (DecimalComparison::EQ, DecimalSign::Positive, DecimalSign::Positive) => (DecimalSign::Positive, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            
            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Negative) => (DecimalSign::Negative, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            (DecimalComparison::GT, DecimalSign::Negative, DecimalSign::Positive) => (DecimalSign::Negative, Decimal::sub_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Negative) => (DecimalSign::Positive, Decimal::sub_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            (DecimalComparison::GT, DecimalSign::Positive, DecimalSign::Positive) => (DecimalSign::Positive, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            
            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Negative) => (DecimalSign::Negative, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
            (DecimalComparison::LT, DecimalSign::Negative, DecimalSign::Positive) => (DecimalSign::Positive, Decimal::sub_inner((rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3), (self.part_0, self.part_1, self.part_2, self.part_3))),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Negative) => (DecimalSign::Negative, Decimal::sub_inner((rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3), (self.part_0, self.part_1, self.part_2, self.part_3))),
            (DecimalComparison::LT, DecimalSign::Positive, DecimalSign::Positive) => (DecimalSign::Positive, Decimal::add_inner((self.part_0, self.part_1, self.part_2, self.part_3), (rhs.part_0, rhs.part_1, rhs.part_2, rhs.part_3))),
        };
        if result.0 {
            panic!("Overflow");
        } else {
            Decimal { sign: result_sign, precision: self.precision, scale: self.scale, part_0: result.1, part_1: result.2, part_2: result.3, part_3: result.4 }
        }
    }
    fn add_inner(lhs: (u32, u32, u32, u32), rhs: (u32, u32, u32, u32)) -> (bool, u32, u32, u32, u32) {
        let (result_0, carry_0) = lhs.0.carrying_add(rhs.0, false);
        let (result_1, carry_1) = lhs.1.carrying_add(rhs.1, carry_0);
        let (result_2, carry_2) = lhs.2.carrying_add(rhs.2, carry_1);
        let (result_3, carry_3) = lhs.3.carrying_add(rhs.3, carry_2);
        (carry_3, result_0, result_1, result_2, result_3)
    }
    pub fn sub(&self, rhs: &Decimal) -> Decimal {
        todo!()
    }
    fn sub_inner(lhs: (u32, u32, u32, u32), rhs: (u32, u32, u32, u32)) -> (bool, u32, u32, u32, u32) {
        todo!()
    }
    pub fn mul(&self, rhs: &Decimal) -> Decimal {
        todo!()
    }
    pub fn mul_10(&self) -> Decimal {
        todo!()
    }
    pub fn div(&self, rhs: &Decimal) -> Decimal {
        todo!()
    }
    pub fn div_10(&self) -> Decimal {
        todo!()
    }
    pub fn compare(&self, rhs: &Decimal) -> DecimalComparison {
        if self.sign == DecimalSign::Positive && rhs.sign == DecimalSign::Negative {
            return DecimalComparison::GT;
        }
        if self.sign == DecimalSign::Negative && rhs.sign == DecimalSign::Positive {
            return DecimalComparison::LT;
        }
        if self.scale == rhs.scale && self.part_0 == rhs.part_0 && self.part_1 == rhs.part_1 && self.part_2 == rhs.part_2 && self.part_3 == rhs.part_3 {
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
            } else if result.part_0 == 0 && result.part_1 == 0 && result.part_2 == 0 && result.part_3 == 0 {
                DecimalComparison::EQ
            } else {
                DecimalComparison::GT
            }
        }
    }
    pub fn compare_modulo(&self, rhs: &Decimal) -> DecimalComparison {
        if self.scale == rhs.scale && self.part_0 == rhs.part_0 && self.part_1 == rhs.part_1 && self.part_2 == rhs.part_2 && self.part_3 == rhs.part_3 {
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
            } else if result.part_0 == 0 && result.part_1 == 0 && result.part_2 == 0 && result.part_3 == 0 {
                DecimalComparison::EQ
            } else {
                DecimalComparison::GT
            }
        }
    }
}
