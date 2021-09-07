pub mod parent;
pub mod principle;

use crate::rust::visitor::user::UserEnum;

pub trait Visitor {
    fn visit(&self, user: UserEnum);
}