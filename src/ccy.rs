use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, Copy)]
pub enum CCY {
    USD = 0,
    GBP = 1,
    EUR = 2,
    JPY = 3,
    CHF = 4,
    CNY = 5
}

impl CCY {
    pub fn to_string(self) -> String {
        match self {
            CCY::USD => "USD".to_string(),
            CCY::GBP => "GBP".to_string(),
            CCY::EUR => "EUR".to_string(),
            CCY::JPY => "JPY".to_string(),
            CCY::CHF => "CHF".to_string(),
            CCY::CNY => "CNY".to_string(),
        }
    }
    pub fn from_int(input: &u32) -> Self {
        match input {
            0 => CCY::USD,
            1 => CCY::GBP,
            2 => CCY::EUR,
            3 => CCY::JPY,
            4 => CCY::CHF,
            5 => CCY::CNY,
            _ => CCY::USD,
        }
    }
}



pub fn get_num_ccys() -> usize {   
    return CCY::iter().count();
}
