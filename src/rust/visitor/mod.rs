use crate::rust::visitor::user::{UserEnum, User};
use crate::rust::visitor::user::student::Student;
use crate::rust::visitor::user::teacher::Teacher;
use crate::rust::visitor::visitor::Visitor;

mod user;
mod visitor;

pub struct DateView {
    user_lists: Vec<UserEnum>,
}

impl DateView {
    pub fn new() -> Self {
        let user_lists = vec![
            UserEnum::Student(Student::new("a", "1", "A")),
            UserEnum::Student(Student::new("b", "2", "A")),
            UserEnum::Teacher(Teacher::new("c", "3", "A")),
            UserEnum::Teacher(Teacher::new("d", "4", "B")),
        ];
        Self { user_lists }
    }

    pub fn show(&self, visitor: impl Visitor) {
        for user in &self.user_lists {
            match user {
                UserEnum::Student(s) => {
                    s.accept(&visitor);
                }
                UserEnum::Teacher(t) => {
                    t.accept(&visitor);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::visitor::DateView;
    use crate::rust::visitor::visitor::parent::Parent;
    use crate::rust::visitor::visitor::principle::Principle;

    #[test]
    fn it_work() {
        let data_view = DateView::new();
        println!("view from parent");
        data_view.show(Parent {});

        println!("view from principle");
        data_view.show(Principle {})
    }
}