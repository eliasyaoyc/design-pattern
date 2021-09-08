use std::hash::Hash;
use std::collections::HashMap;
use std::thread::sleep;

pub trait MallTrait {
    fn new(u_id: String, u_pwd: String) -> Self;
    fn login(&self) -> bool;
    fn reptile(&self, url: String) -> HashMap<String, String>;
    fn create_base64(&self, goods_info: HashMap<String, String>) -> String {
        "asa".to_string()
    }

    fn generate_goods_poster(&self, url: String) -> String {
        if !self.login() {
            String::default()
        } else {
            let reptile = self.reptile(url);
            self.create_base64(reptile)
        }
    }
}


pub struct JdMall {
    u_id: String,
    u_pwd: String,
}

impl MallTrait for JdMall {
    fn new(u_id: String, u_pwd: String) -> Self {
        Self { u_id, u_pwd }
    }

    fn login(&self) -> bool {
        println!("login to the JD as {:?} with {:?}", self.u_id, self.u_pwd);
        true
    }

    fn reptile(&self, url: String) -> HashMap<String, String> {
        println!("reptile : {}", url);
        HashMap::default()
    }
}

#[cfg(test)]
mod test {
    use crate::rust::template::{JdMall, MallTrait};

    #[test]
    fn t() {
        let jd = JdMall::new("yyc".into(), "yyc".into());
        let result = jd.generate_goods_poster("1212".into());
        println!("result : {}", result)
    }
}














