use std::ops::Deref;

/// The essence of the pattern is that resource initialisation is done in the constructor of an object and finalisation in the destructor.
struct Foo {
}

impl Foo{
    pub fn foo(&self){

    }
}
struct Mutex<T> {}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
}

impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        MutexGuard {
            data: self,
        }
    }
}

impl <'a,T> Drop for MutexGuard<'a,T>{
    fn drop(&mut self) {

    }
}

impl <'a,T> Deref for MutexGuard<'a,T>{
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x:Mutex<Foo>){
    let xx = x.lock();
    xx.foo();
}