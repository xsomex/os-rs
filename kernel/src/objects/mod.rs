use core::{
    any::{Any, TypeId},
    cell::RefCell, fmt,
};

use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::ToString, sync::{Arc, Weak}};

use crate::display::text::WriteString;

pub trait ObjectFunction {
    type Input;
    type Output;
}

pub struct Object {
    data: Box<dyn Any>,
    functions: RefCell<BTreeMap<TypeId, Box<dyn Any>>>,
}

pub struct ObjectsManager {
    objects: RefCell<BTreeMap<usize, Object>>,
    counter: RefCell<usize>,
    weak_self: RefCell<Weak<Self>>,
}

impl Object {
    pub fn new(obj: Box<dyn Any>) -> Object {
        Object {
            data: obj,
            functions: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn set_fn<M: 'static + ObjectFunction>(
        &self,
        function: fn(&Box<dyn Any>, M::Input) -> Result<M::Output, Error>,
    ) {
        let id = TypeId::of::<M>();
        self.functions.borrow_mut().insert(id, Box::new(function));
    }

    pub fn call<M: 'static + ObjectFunction>(&self, args: M::Input) -> Result<M::Output, Error> {
        let funcs = self.functions.borrow();
        let func: &Box<dyn Any> = match funcs.get(&TypeId::of::<M>()) {
            Some(v) => v,
            _ => return Err(Error::NotImplemented),
        };
        let func =
            match func.downcast_ref::<fn(&Box<dyn Any>, M::Input) -> Result<M::Output, Error>>() {
                None => return Err(Error::TypeError),
                Some(v) => *v,
            };

        func(&self.data, args)
    }
}

#[derive(Debug, Clone)]
pub struct ObjectHandle {
    id: usize,
    store: Weak<ObjectsManager>,
}

impl ObjectsManager {
    pub fn new() -> Arc<Self> {
        let s = Arc::new(Self {
            objects: RefCell::new(BTreeMap::new()),
            counter: RefCell::new(0),
            weak_self: RefCell::new(Weak::new()),
        });
        *s.weak_self.borrow_mut() = Arc::downgrade(&s);
        s
    }

    pub fn add_object(&self, obj: Object) -> Arc<ObjectHandle> {
        let mut store = self.objects.borrow_mut();
        let id = *self.counter.borrow();
        store.insert(id, obj);
        *self.counter.borrow_mut() = id + 1;

        Arc::new(ObjectHandle {
            id,
            store: self.weak_self.borrow().clone(),
        })
    }
}

pub trait CallManager<M: 'static + ObjectFunction> {
    fn call(&self, handle: Arc<ObjectHandle>, args: M::Input) -> Result<M::Output, Error>;
}

impl<M: 'static + ObjectFunction> CallManager<M> for ObjectsManager {
    default fn call(&self, handle: Arc<ObjectHandle>, args: M::Input) -> Result<M::Output, Error> {
        let store = self.objects.borrow();
        let obj = match store.get(&handle.id) {
            Some(v) => v,
            None => return Err(Error::ObjectNotExisting),
        };
        obj.call::<M>(args)
    }
}

impl ObjectHandle {
    pub fn call<M: 'static + ObjectFunction>(&self, args: M::Input) -> Result<M::Output, Error> {
        <ObjectsManager as CallManager<M>>::call(
            &self.store.upgrade().unwrap(),
            Arc::new(self.clone()),
            args,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    NotImplemented,
    TypeError,
    ObjectNotExisting,
    Other,
}

pub fn inbox<T>(obj: T) -> Box<T> {
    Box::new(obj)
}

pub struct ArcObjectHandle(pub Arc<ObjectHandle>);

impl fmt::Write for ArcObjectHandle {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.call::<WriteString>(s.to_string()).unwrap();
        Ok(())
    }
}
