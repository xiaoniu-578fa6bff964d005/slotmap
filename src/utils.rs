#![allow(dead_code)]

//! Contains some utilities.

use std::marker::PhantomData;

pub type Invariant<T> = PhantomData<fn(T) -> T>;
