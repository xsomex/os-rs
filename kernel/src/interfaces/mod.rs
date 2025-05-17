use core::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use alloc::{collections::btree_map::BTreeMap, sync::Arc, sync::Weak};

pub struct InterfacesManager {
    interfaces: RefCell<BTreeMap<TypeId, (BTreeMap<usize, Arc<dyn Any>>, usize)>>,
    weak_self: RefCell<Weak<InterfacesManager>>,
}

impl InterfacesManager {
    pub fn new() -> Arc<Self> {
        let slf = Arc::new(InterfacesManager {
            interfaces: RefCell::new(BTreeMap::new()),
            weak_self: RefCell::new(Weak::new()),
        });
        *slf.weak_self.borrow_mut() = Arc::downgrade(&slf);
        slf
    }

    /// WARNING: To add a trait interface, use add_interface::<dyn MyTrait>(...)
    pub fn add_interface<T: 'static>(&self, interface: Arc<T>) -> InterfaceHandle<T> {
        let type_id = TypeId::of::<T>();
        let mut btree = self.interfaces.borrow_mut();

        if !btree.contains_key(&type_id) {
            btree.insert(type_id, (BTreeMap::new(), 0));
        }
        let to_change = btree.get_mut(&type_id).unwrap();
        let id = to_change.1;

        to_change.0.insert(id, interface);
        to_change.1 = id + 1;

        InterfaceHandle {
            id,
            store: self.weak_self.borrow().clone(),
            _phantom: PhantomData,
        }
    }
}

pub struct InterfaceHandle<T> {
    id: usize,
    store: Weak<InterfacesManager>,
    _phantom: PhantomData<T>,
}
impl<T> !Clone for InterfaceHandle<T> {}
impl<T> !Copy for InterfaceHandle<T> {}
impl<T> !Debug for InterfaceHandle<T> {}
impl<T> !Display for InterfaceHandle<T> {}

pub trait InterfaceInputOutput {
    type Input;
    type Output;
}

pub trait CallableObject<IO: InterfaceInputOutput> {
    fn call(&self, inputs: IO::Input) -> IO::Output;
}

pub trait CallInterface<T: 'static + CallableObject<IO>, IO: InterfaceInputOutput> {
    fn call(&self, interface: &InterfaceHandle<T>, inputs: IO::Input) -> Result<IO::Output, ()> {
        let interfaces_manager = interface.store.upgrade().unwrap();
        let btree = interfaces_manager.interfaces.borrow();
        if !btree.contains_key(&TypeId::of::<T>()) {
            return Err(());
        }
        let btree = btree.get(&TypeId::of::<T>()).unwrap();
        if !btree.0.contains_key(&interface.id) {
            return Err(());
        }
        let obj = btree.0.get(&interface.id).unwrap();

        match obj.downcast_ref::<T>() {
            None => Err(()),
            Some(object) => Ok(object.call(inputs)),
        }
    }
}

impl<T: 'static + CallableObject<IO>, IO: InterfaceInputOutput> CallInterface<T, IO>
    for InterfaceHandle<T>
{
}

impl<T: 'static> InterfaceHandle<T> {
    pub fn call<IO: InterfaceInputOutput>(&self, inputs: IO::Input) -> Result<IO::Output, ()>
    where
        T: CallableObject<IO>,
    {
        <InterfaceHandle<T> as CallInterface<T, IO>>::call(&self, &self, inputs)
    }
}

impl<T: 'static + CallableObject<IO>, IO: InterfaceInputOutput> CallInterface<T, IO>
    for InterfacesManager
{
    default fn call(
        &self,
        interface: &InterfaceHandle<T>,
        inputs: IO::Input,
    ) -> Result<IO::Output, ()> {
        let interfaces_manager = self;
        let btree = interfaces_manager.interfaces.borrow();
        if !btree.contains_key(&TypeId::of::<T>()) {
            return Err(());
        }
        let btree = btree.get(&TypeId::of::<T>()).unwrap();
        if !btree.0.contains_key(&interface.id) {
            return Err(());
        }
        let obj = btree.0.get(&interface.id).unwrap();

        match obj.downcast_ref::<T>() {
            None => Err(()),
            Some(object) => Ok(object.call(inputs)),
        }
    }
}



// ------------------------------------ DEMO TEST ---------------------------------------

pub struct Test;
pub struct TestFN;
impl InterfaceInputOutput for TestFN {
    type Input = usize;
    type Output = usize;
}
impl CallableObject<TestFN> for Test {
    fn call(
        &self,
        inputs: <TestFN as InterfaceInputOutput>::Input,
    ) -> <TestFN as InterfaceInputOutput>::Output {
        inputs
    }
}

impl CallInterface<Test, TestFN> for InterfacesManager {
    default fn call(
        &self,
        interface: &InterfaceHandle<Test>,
        inputs: <TestFN as InterfaceInputOutput>::Input,
    ) -> Result<<TestFN as InterfaceInputOutput>::Output, ()> {
        let interfaces_manager = self;
        let btree = interfaces_manager.interfaces.borrow();
        if !btree.contains_key(&TypeId::of::<Test>()) {
            return Err(());
        }
        let btree = btree.get(&TypeId::of::<Test>()).unwrap();
        if !btree.0.contains_key(&interface.id) {
            return Err(());
        }
        let obj = btree.0.get(&interface.id).unwrap();

        match obj.downcast_ref::<Test>() {
            None => Err(()),
            Some(object) => Ok(object.call(inputs)),
        };

        return Ok(2)
    }
}
