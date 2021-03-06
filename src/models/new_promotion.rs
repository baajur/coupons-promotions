use crate::schema::promotions;
use crate::models::{PromotionReturn, PromotionType, DateTime};

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "promotions"]
pub struct NewPromotion {
    pub code: String,
    pub name: String,
    pub condition: String,
    pub active: bool,
    pub return_type: String,
    pub return_value: f64,
    pub type_: String,
    pub organization_id: String,
    pub expiration: DateTime,
}

impl NewPromotion {
    pub fn new(
        name: String,
        code: String,
        condition: String,
        active: bool,
        p_return: PromotionReturn,
        p_type: PromotionType,
        organization_id: String,
        expiration: DateTime,
    ) -> Self {
        let (return_type, return_value) = match p_return {
            PromotionReturn::Percentage(val) => ("percentage".into(), val),
            PromotionReturn::Fixed(val) => ("fixed".into(), val)
        };
        let type_ = match p_type {
            PromotionType::Coupon => "coupon".into(),
            PromotionType::Discount => "discount".into()
        };

        NewPromotion { name, condition, code, active, return_value, return_type, type_, organization_id, expiration }
    }
}