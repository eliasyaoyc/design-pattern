pub trait ICuisine: ICuisineClone {
    fn cook(&self);
}

pub trait ICuisineClone {
    fn clone_box(&self) -> Box<dyn ICuisine>;
}

impl<T> ICuisineClone for T
    where
        T: 'static + ICuisine + Clone,
{
    fn clone_box(&self) -> Box<dyn ICuisine> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ICuisine> {
    fn clone(&self) -> Box<dyn ICuisine> {
        self.clone_box()
    }
}

pub trait ICook {
    fn doCook(&self);
}

#[derive(Clone)]
pub struct GuangDongCuisine {
    cook: GuangDongCook,
}

impl GuangDongCuisine {
    pub fn new() -> Self {
        Self { cook: GuangDongCook {} }
    }
}

impl ICuisine for GuangDongCuisine {
    fn cook(&self) {
        self.cook.doCook()
    }
}

#[derive(Clone)]
pub struct JiangSuCuisine {
    cook: JiangSuCook,
}

impl JiangSuCuisine {
    pub fn new() -> Self {
        Self { cook: JiangSuCook {} }
    }
}

impl ICuisine for JiangSuCuisine {
    fn cook(&self) {
        self.cook.doCook()
    }
}

#[derive(Clone)]
pub struct GuangDongCook {}

impl ICook for GuangDongCook {
    fn doCook(&self) {
        println!("广东师傅做菜");
    }
}

#[derive(Clone)]
pub struct JiangSuCook {}

impl ICook for JiangSuCook {
    fn doCook(&self) {
        println!("江苏师傅做菜");
    }
}

pub struct Waiter {
    cuisines: Vec<Box<dyn ICuisine>>,
}

impl Waiter {
    pub fn order(&mut self, cuisine: Box<dyn ICuisine>) {
        self.cuisines.push(cuisine.clone());
    }

    pub fn place_order(&self) {
        for x in &self.cuisines {
            x.as_ref().cook();
        }
    }
}

#[test]
pub fn test() {
    let mut waiter = Waiter { cuisines: vec![] };
    waiter.order(Box::new(GuangDongCuisine::new()));
    waiter.order(Box::new(JiangSuCuisine::new()));
    waiter.place_order();
}
