use crate::{
	pool::{Pool, PoolInner},
	reset::Reset,
};
use alloc::sync::{Arc, Weak};
use core::{
	borrow::{Borrow, BorrowMut},
	cmp::Ordering,
	fmt::{self, Debug, Display},
	hash::{Hash, Hasher},
	ops::{Deref, DerefMut},
};

/// An object taken from a [`Pool`].
///
/// When dropped, it will be [`Reset`] and returned to the pool if it has spare capacity.
pub struct Pooled<T: Default + Reset> {
	pool_inner: Weak<PoolInner<T>>,

	// (internal docs, users don't need to worry about this)
	/// ⚠️ If you set this to [`None`], you must ensure that the [`Pooled`] cannot be used anymore.
	///
	/// Almost all methods (including dereferencing) assume this is [`Some`], and they'll panic
	/// otherwise.
	object: Option<T>,
}

impl<T: Default + Reset> Pooled<T> {
	pub(super) fn new(object: T, pool: &Pool<T>) -> Self {
		Self {
			object: Some(object),
			pool_inner: Arc::downgrade(&pool.inner),
		}
	}

	/// Detach this object from the pool.
	///
	/// It will not be returned to the pool once dropped.
	///
	/// ```
	/// # use dynamic_pooling::{Pool, Pooled};
	/// let pool: Pool<String> = Pool::new(69);
	/// let foo: Pooled<String> = pool.take();
	/// assert_eq!(pool.in_use(), 1);
	///
	/// let foo: String = Pooled::detach(foo);
	/// assert_eq!(pool.in_use(), 0);
	/// ```
	pub fn detach(mut this: Self) -> T {
		this.object.take().expect("always some")
	}

	/// Get the pool associated with this object.
	///
	/// Objects can outlive the pool they came from, so this returns an [`Option`].
	///
	/// ```
	/// # use dynamic_pooling::{Pool, Pooled};
	/// let pool = Pool::<String>::new(69);
	/// let foo = pool.take();
	/// assert!(Pooled::get_pool(&foo).is_some());
	/// drop(pool);
	/// assert!(Pooled::get_pool(&foo).is_none());
	/// ```
	pub fn get_pool(this: &Self) -> Option<Pool<T>> {
		this.pool_inner.upgrade().map(|inner| Pool { inner })
	}
}

impl<T: Default + Reset> Drop for Pooled<T> {
	fn drop(&mut self) {
		if let Some(pool_inner) = self.pool_inner.upgrade() {
			if let Some(mut object) = self.object.take() {
				object.reset();
				let _ = pool_inner.push(object);
			}
		}
	}
}

impl<T: Default + Reset> Deref for Pooled<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.object.as_ref().expect("always some")
	}
}

impl<T: Default + Reset> DerefMut for Pooled<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.object.as_mut().expect("always some")
	}
}

impl<T: Default + Reset> AsRef<T> for Pooled<T> {
	fn as_ref(&self) -> &T {
		self
	}
}

impl<T: Default + Reset> AsMut<T> for Pooled<T> {
	fn as_mut(&mut self) -> &mut T {
		self
	}
}

impl<T: Default + Reset> Borrow<T> for Pooled<T> {
	fn borrow(&self) -> &T {
		self
	}
}

impl<T: Default + Reset> BorrowMut<T> for Pooled<T> {
	fn borrow_mut(&mut self) -> &mut T {
		self
	}
}

impl<T: Default + Reset> Hash for Pooled<T>
where
	T: Hash,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state);
	}
}

impl<T: Default + Reset> PartialEq for Pooled<T>
where
	T: PartialEq<T>,
{
	fn eq(&self, other: &Self) -> bool {
		(**self).eq(other)
	}
}

impl<T: Default + Reset> PartialEq<T> for Pooled<T>
where
	T: PartialEq<T>,
{
	fn eq(&self, other: &T) -> bool {
		(**self).eq(other)
	}
}

impl<T: Default + Reset> Eq for Pooled<T> where T: Eq {}

impl<T: Default + Reset> PartialOrd for Pooled<T>
where
	T: PartialOrd<T>,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(**self).partial_cmp(other)
	}
}

impl<T: Default + Reset> Ord for Pooled<T>
where
	T: Ord,
{
	fn cmp(&self, other: &Self) -> Ordering {
		(**self).cmp(other)
	}
}

impl<T: Default + Reset + PartialOrd<T>> PartialOrd<T> for Pooled<T> {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		(**self).partial_cmp(other)
	}
}

impl<T: Default + Reset> Debug for Pooled<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T: Default + Reset> Display for Pooled<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}
