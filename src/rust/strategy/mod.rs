use std::collections::HashMap;
use std::ptr::eq;

pub trait Strategy<T> {
    fn amount(&self, info: T, price: f64) -> f64;
}

pub struct MJStrategy {}

impl Strategy<HashMap<&str, f64>> for MJStrategy {
    fn amount(&self, info: HashMap<&str, f64>, price: f64) -> f64 {
        1.0
    }
}


pub struct ZJStrategy {}

impl Strategy<f64> for ZJStrategy {
    fn amount(&self, info: f64, price: f64) -> f64 {
        2.0
    }
}

pub struct Context<T> {
    coupon: Box<dyn Strategy<T>>,
}

impl<T> Context<T> {
    pub fn new(coupon: Box<dyn Strategy<T>>) -> Self {
        Self { coupon }
    }

    pub fn amount(&self, info: T, price: f64) -> f64 {
        self.coupon.amount(info, price)
    }
}

#[test]
fn test() {
    let context = Context::new(Box::new(ZJStrategy {}));
    let price = context.amount(10.0, 100.0);
    assert_eq!(price, 2.0);

    let context = Context::new(Box::new(MJStrategy {}));
    let price = context.amount(HashMap::new(), 100.0);
    assert_eq!(price,1.0);
}
