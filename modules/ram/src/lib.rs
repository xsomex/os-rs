#![no_std]
//! This lib is made in two parts: the virtual memory management and the physical memory
//! management. These two parts are used to create a global RamManager. This architecture allows to
//! change the way the physical memory is represented and the way the virtual memory is managed
//! idpendantly.
//!
//! It is not possible to have one without the other, because both are interdependant. Be careful,
//! if you try a new variant, to avoid cyclic dependency.
//!
//! Note: `size: usize` is always the number of 4kB blocks. If it's not the case, choose an other
//! name.
//!
//! TODO: phys api

pub mod virt;
pub mod phys;
