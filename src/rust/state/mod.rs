use std::time::SystemTime;
use std::alloc::{System, Layout};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;

pub type Res = Result<&'static str, &'static str>;

pub trait State {
    fn arraignment(&self, id: &'static str, curr: Status) -> Res;
    fn check_pass(&self, id: &'static str, curr: Status) -> Res;
    fn check_refuse(&self, id: &'static str, curr: Status) -> Res;
    fn check_revoke(&self, id: &'static str, curr: Status) -> Res;
    fn close(&self, id: &'static str, curr: Status) -> Res;
    fn open(&self, id: &'static str, curr: Status) -> Res;
    fn doing(&self, id: &'static str, curr: Status) -> Res;
}

pub struct StateHandler {
    state_map: HashMap<Status, Box<dyn State>>,
}

impl StateHandler {
    pub fn new() -> Self {
        let mut state_map = HashMap::<Status, Box<dyn State>>::new();
        // state_map.insert(Status::Check, Box::new(Check {}));
        // state_map.insert(Status::Doing, Box::new(Doing {}));
        state_map.insert(Status::Editing, Box::new(Editing {}));
        // state_map.insert(Status::Open, Box::new(Open {}));
        // state_map.insert(Status::Pass, Box::new(Pass {}));
        // state_map.insert(Status::Refuse, Box::new(Refuse {}));
        Self { state_map }
    }

    pub fn arraignment(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().arraignment(id, curr)
    }

    pub fn check_pass(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_pass(id, curr)
    }

    pub fn check_refuse(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_refuse(id, curr)
    }

    pub fn check_revoke(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_revoke(id, curr)
    }

    pub fn close(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().close(id, curr)
    }

    pub fn open(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().open(id, curr)
    }

    pub fn doing(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().doing(id, curr)
    }
}

pub struct Editing {}

impl State for Editing {
    fn arraignment(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Check);
        Ok("arraignment , success")
    }

    fn check_pass(&self, id: &'static str, curr: Status) -> Res {
        Err("editing, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot revoke")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("not close, cannot open")
    }

    fn doing(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot modify")
    }
}


#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum Status {
    Editing,
    Check,
    Pass,
    Refuse,
    Doing,
    Close,
    Open,
}

#[derive(Debug)]
pub struct ActivityInfo {
    id: String,
    name: String,
    status: Status,
    begin_time: SystemTime,
    end_time: SystemTime,
}

impl ActivityInfo {
    pub fn new(id: String, name: String, status: Status, begin_time: SystemTime, end_time: SystemTime) -> Self {
        Self { id, name, status, begin_time, end_time }
    }
}

static STATUS_MAP: Lazy<Mutex<HashMap<&str, Status>>> = Lazy::new(|| {
    Mutex::new({
        let m = HashMap::new();
        m
    })
});


pub struct ActivityService {}

impl ActivityService {
    pub fn init(id: &'static str, status: Status) {
        STATUS_MAP.lock().unwrap().insert(id, status);
    }

    pub fn query_activity(id: &str) -> ActivityInfo {
        let activity = ActivityInfo::new(
            id.into(),
            String::from("New activity"),
            *STATUS_MAP.lock().unwrap().get(id).unwrap(),
            SystemTime::now(),
            SystemTime::now(),
        );
        activity
    }

    pub fn query_activity_status(id: &str) -> Status {
        *STATUS_MAP.lock().unwrap().get(id).unwrap()
    }

    pub fn exec_status(id: &'static str, before: Status, after: Status) {
        if before != *STATUS_MAP.lock().unwrap().get(id).unwrap() {
            return;
        }
        STATUS_MAP.lock().unwrap().insert(id, after);
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::rust::state::{ActivityService, Status, StateHandler};

    // cargo test -- --nocapture
    #[test]
    fn edit2arraign() {
        let id = "100001";
        ActivityService::init(id, Status::Editing);
        let handler = StateHandler::new();
        let result = handler.arraignment(id, Status::Editing);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn edit2open() {
        let id = "100001";
        ActivityService::init(id, Status::Editing);
        let handler = StateHandler::new();
        let result = handler.open(id, Status::Editing);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn refuse2do() {
        let id = "100001";
        ActivityService::init(id, Status::Refuse);
        let handler = StateHandler::new();
        let result = handler.doing(id, Status::Refuse);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn refuse2revoke() {
        let id = "100001";
        ActivityService::init(id, Status::Refuse);
        let handler = StateHandler::new();
        let result = handler.check_revoke(id, Status::Refuse);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }
}





















