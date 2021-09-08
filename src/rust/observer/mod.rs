#[derive(Clone, Debug)]
pub struct Event {
    pub name: &'static str,
    pub content: &'static str,
}

pub trait Listener {
    fn onEvent(&self, event: Event);
    fn type_id(&self) -> usize;
}

impl<T: Listener> PartialEq<T> for dyn Listener {
    fn eq(&self, other: &T) -> bool {
        self.type_id() == other.type_id()
    }
}

pub struct MQListener {}

impl Listener for MQListener {
    fn onEvent(&self, event: Event) {
        println!("MQ 发送用户 {}, 内容 {}", event.name, event.content);
    }

    fn type_id(&self) -> usize {
        1
    }
}

pub struct MessageListener {}

impl Listener for MessageListener {
    fn onEvent(&self, event: Event) {
        println!("MESSAGE 发送用户 {}, 内容 {}", event.name, event.content);
    }

    fn type_id(&self) -> usize {
        2
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum EventType {
    MQ,
    MESSAGE,
}

pub struct EventManager {
    pub listeners: std::collections::HashMap<EventType, Vec<Box<dyn Listener>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let mut listeners = std::collections::HashMap::new();
        listeners.insert(EventType::MQ, vec![]);
        listeners.insert(EventType::MESSAGE, vec![]);

        Self { listeners }
    }

    pub fn subscribe(&mut self, event: EventType, listener: impl Listener + 'static) {
        let l = self.listeners.get_mut(&event).unwrap();
        l.push(Box::new(listener));
    }

    pub fn unsubscribe(&mut self, event: EventType, listener: impl Listener + 'static) {
        let l = self.listeners.get_mut(&event).unwrap();
        let pos = l.iter().position(|x| *(*x) == listener).unwrap();
        l.remove(pos);
    }

    pub fn notify(&mut self, event: EventType, e: Event) {
        let l = self.listeners.get(&event).unwrap();
        for i in l {
            i.onEvent(e.clone());
        }
    }
}

pub struct Business {
    manager: EventManager,
}

impl Business {
    pub fn new() -> Self {
        let mut manager = EventManager::new();
        manager.subscribe(EventType::MESSAGE, MessageListener {});
        manager.subscribe(EventType::MQ, MQListener {});
        Self { manager }
    }

    pub fn handle(&mut self, user: &'static str, content: &'static str) {
        let event = Event { name: user, content };
        self.manager.notify(EventType::MESSAGE, event.clone());
        self.manager.notify(EventType::MQ, event.clone());
    }
}

#[test]
pub fn test() {
    let mut b = Business::new();
    b.handle("yyc", "yyc");
}
