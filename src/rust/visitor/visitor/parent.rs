use crate::rust::visitor::visitor::Visitor;
use crate::rust::visitor::user::UserEnum;

pub struct Parent;

impl Visitor for Parent {
    fn visit(&self, user: UserEnum) {
        match user {
            UserEnum::Student(student) => {
                println!("visit student from parent visitor: Student {} ,marking {}",
                         student.name,
                         student.ranking())
            }
            UserEnum::Teacher(teacher) => {
                println!("visit teacher from parent visitor: Teacher {}",
                         teacher.name)
            }
        }
    }
}
