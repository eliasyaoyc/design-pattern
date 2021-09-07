use crate::rust::visitor::visitor::Visitor;
use crate::rust::visitor::user::UserEnum;

pub struct Principle {}

impl Visitor for Principle {
    fn visit(&self, user: UserEnum) {
        match user {
            UserEnum::Student(student) => {
                println!("visit student from principle visitor: {}", student.name);
            }
            UserEnum::Teacher(teacher) => {
                println!("visit teacher from principle visitor: {}, entrance ratio: {}", teacher
                    .name, teacher
                    .entrance_ratio());
            }
        }
    }
}