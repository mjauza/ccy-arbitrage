use crate::ccy;

pub struct FXRate {
    pub ccy1 : ccy::CCY,
    pub ccy2 : ccy::CCY,
    fx_rate_one1X2: f32, // 1 ccy1 == fx_rate_one1X2 ccy2
    fx_rate_one2X1: f32
}

impl FXRate {
    pub fn new(ccy1 : ccy::CCY , ccy2: ccy::CCY, fx_rate_one1X2:f32 ) -> Self {
        if fx_rate_one1X2 <= 0.0 {
            panic!("FX rate can not be negative");
        }
        return Self {
            ccy1 : ccy1,
            ccy2 : ccy2,
            fx_rate_one1X2 : fx_rate_one1X2,
            fx_rate_one2X1 : 1.0/fx_rate_one1X2
        };
    }    

    pub fn ccy1_to_ccy2(&self, ccy1_amnt: f32)  -> f32 {
        return self.fx_rate_one1X2 * ccy1_amnt;
    }
    pub fn ccy2_to_ccy1(&self, ccy2_amnt: f32) -> f32 {
        return self.fx_rate_one2X1*ccy2_amnt;
    }
    

}