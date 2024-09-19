

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

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
	} // end fn from

	pub async fn get ( &self ) -> T where T: Clone {
		return self.value.read().await.clone();
	} // end fn get

	pub async fn set ( &self, value: T ) {
		*self.value.write().await = value;
	} // end fn set

	pub async fn with<F: Fn (&T) -> R, R> ( &self, closure: F ) -> R {
		return closure ( &*self.value.read().await );
	} // end fn with

	pub async fn with_mut<F: Fn (&mut T) -> R, R> ( &self, closure: F ) -> R {
		return closure ( &mut *self.value.write().await );
	} // end fn with_mut

	pub async fn read ( &self ) -> RwLockReadGuard<'_, T> {
		return self.value.read().await;
	} // end fn read

	pub async fn write ( &self ) -> RwLockWriteGuard<'_, T> {
		return self.value.write().await;
	} // end fn write

}