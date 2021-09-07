pub mod student;
pub mod teacher;

use crate::rust::visitor::visitor::Visitor;
use crate::rust::visitor::user::student::Student;
use crate::rust::visitor::user::teacher::Teacher;

pub trait User {
    fn accept(&self, visitor: &impl Visitor);
}

#[derive(Debug)]
pub enum UserEnum {
    Student(Student),
    Teacher(Teacher),
}