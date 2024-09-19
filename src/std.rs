use std::{sync::{Arc, RwLock}};
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

#[derive(Clone)]
pub struct Vossel<T> {
	value: Arc<RwLock<T>>
} // end struct Vossel

impl<T> Vossel<T> {

	pub fn new ( ) -> Self where T: Default {
		return Vossel::from( Default::default() );
	} // end fn new

	pub fn from ( value: T ) -> Self {
		return Self { value:
			Arc::new(RwLock::new(value))
		}; // end return
	} // end fn new

	pub fn get ( &self ) -> T where T: Clone {
		return self.value.clone().read().unwrap().clone();
	} // end fn get

	pub fn set ( &self, value: T ) {
		*self.value.clone().write().unwrap() = value;
	} // end fn set

	pub fn with<F: Fn (&T) -> R, R> ( &self, closure: F ) -> R {
		return closure ( &self.value.clone().read().unwrap() );
	} // end fn with

	pub fn with_mut<F: Fn (&mut T) -> R, R> ( &self, closure: F ) -> R {
		return closure ( &mut self.value.clone().write().unwrap() );
	} // end fn with_mut

	pub fn read ( &self ) -> RwLockReadGuard<'_, T> {
		return self.value.read().unwrap();
	} // end fn read

	pub fn write ( &self ) -> RwLockWriteGuard<'_, T> {
		return self.value.write().unwrap();
	} // end fn write

}

