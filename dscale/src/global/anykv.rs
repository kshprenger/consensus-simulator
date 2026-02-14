//! Global key-value storage for simulation state and metrics.
//!
//! This module provides a global key-value store that can hold values of any type.
//! It's useful for sharing state, metrics, or configuration between processes or
//! for passing data back to the host application after simulation completion.
//!
//! The storage is thread-local and persists throughout the simulation lifetime.
//! All functions operate on a per-simulation basis and are reset when a new
//! simulation starts.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub(crate) static ANY_KV: RefCell<HashMap<String, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Stores a value of any type in the global key-value store.
///
/// This function allows you to store values that can be retrieved later using
/// [`get`] or modified using [`modify`]. The value can be of any type that
/// implements `'static`.
///
/// # Type Parameters
///
/// * `T` - The type of the value to store. Must be `'static`.
///
/// # Arguments
///
/// * `key` - A string key to identify the stored value
/// * `value` - The value to store
///
/// # Examples
///
/// ```rust
/// use dscale::global::anykv;
///
/// // Store different types of values
/// anykv::set("counter", 42u32);
/// anykv::set("name", "simulation_1".to_string());
/// anykv::set("metrics", vec![1.0, 2.0, 3.0]);
/// ```
///
/// # Panics
///
/// This function does not panic under normal circumstances.
pub fn set<T: 'static>(key: &str, value: T) {
    ANY_KV.with(|m| {
        m.borrow_mut().insert(key.to_string(), Box::new(value));
    });
}

/// Retrieves a cloned copy of a value from the global key-value store.
///
/// This function returns a clone of the stored value. The original value
/// remains in the store and can be retrieved again.
///
/// # Type Parameters
///
/// * `T` - The expected type of the stored value. Must be `'static` and `Clone`.
///
/// # Arguments
///
/// * `key` - The string key identifying the value to retrieve
///
/// # Returns
///
/// A cloned copy of the stored value.
///
/// # Examples
///
/// ```rust
/// use dscale::global::anykv;
///
/// // Store and retrieve a value
/// anykv::set("counter", 42u32);
/// let counter: u32 = anykv::get("counter");
/// assert_eq!(counter, 42);
///
/// // Store and retrieve a more complex type
/// anykv::set("metrics", vec![1.0, 2.0, 3.0]);
/// let metrics: Vec<f64> = anykv::get("metrics");
/// assert_eq!(metrics, vec![1.0, 2.0, 3.0]);
/// ```
///
/// # Panics
///
/// This function panics if:
/// * The key does not exist in the store
/// * The stored value cannot be downcast to type `T`
pub fn get<T: 'static + Clone>(key: &str) -> T {
    ANY_KV.with(|m| {
        m.borrow()
            .get(key)
            .expect("No key")
            .downcast_ref::<T>()
            .cloned()
            .expect("Wrong type cast")
    })
}

/// Modifies a value in the global key-value store in-place.
///
/// This function allows you to modify a stored value without retrieving and
/// re-storing it. The modification is performed through a closure that receives
/// a mutable reference to the stored value.
///
/// # Type Parameters
///
/// * `T` - The expected type of the stored value. Must be `'static`.
///
/// # Arguments
///
/// * `key` - The string key identifying the value to modify
/// * `f` - A closure that receives a mutable reference to the stored value
///
/// # Examples
///
/// ```rust
/// use dscale::global::anykv;
///
/// // Store a counter and increment it
/// anykv::set("counter", 0u32);
/// anykv::modify("counter", |counter: &mut u32| {
///     *counter += 1;
/// });
/// let result: u32 = anykv::get("counter");
/// assert_eq!(result, 1);
///
/// // Modify a vector
/// anykv::set("metrics", vec![1.0, 2.0]);
/// anykv::modify("metrics", |metrics: &mut Vec<f64>| {
///     metrics.push(3.0);
/// });
/// let result: Vec<f64> = anykv::get("metrics");
/// assert_eq!(result, vec![1.0, 2.0, 3.0]);
/// ```
///
/// # Panics
///
/// This function panics if:
/// * The key does not exist in the store
/// * The stored value cannot be downcast to type `T`
pub fn modify<T: 'static>(key: &str, f: impl FnOnce(&mut T)) {
    ANY_KV.with(|m| {
        f(m.borrow_mut()
            .get_mut(key)
            .expect("No key")
            .downcast_mut::<T>()
            .expect("Wrong type cast"));
    });
}

pub fn drop_anykv() {
    ANY_KV.take();
}
