use crate::{Pooled, Reset};
use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;

/// A lock-free, thread-safe object pool.
pub struct Pool<T: Default + Reset> {
	/// A [`Pool`] is just a wrapper over this.
	pub(super) inner: Arc<PoolInner<T>>,
}

// todo: don't show ArrayQueue to the rest of the crate
pub(super) type PoolInner<T> = ArrayQueue<T>;

impl<T: Default + Reset> Pool<T> {
	/// Create a new pool with the specified capacity.
	///
	/// Note: The capacity will be fully allocated.
	///
	/// # Panics
	/// Panics if the capacity is `0`.
	pub fn new(capacity: usize) -> Self {
		assert!(capacity > 0, "capacity must be more than 0");
		Self {
			inner: Arc::new(PoolInner::new(capacity)),
		}
	}

	/// Take an object from the pool, creating a new one if none are available.
	///
	/// ```
	/// # use dynamic_pooling::{Pool, Pooled};
	/// let pool: Pool<String> = Pool::new(69);
	/// let mut string: Pooled<String> = pool.take();
	/// // do something with it...
	/// ```
	pub fn take(&self) -> Pooled<T> {
		Pooled::new(self.inner.pop().unwrap_or_default(), self)
	}

	/// Take an object from the pool, returning [`None`] if none are available.
	///
	/// This will never allocate.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(69);
	/// assert!(pool.try_take().is_none());
	///
	/// // add an object to the pool
	/// let foo = pool.take();
	/// drop(foo);
	///
	/// assert!(pool.try_take().is_some());
	/// ```
	pub fn try_take(&self) -> Option<Pooled<T>> {
		self.inner.pop().map(|object| Pooled::new(object, self))
	}

	/// The number of available objects in the pool.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(69);
	///
	/// // add 3 objects to the pool
	/// let foo = pool.take();
	/// let bar = pool.take();
	/// let baz = pool.take();
	/// drop((foo, bar, baz));
	///
	/// assert_eq!(pool.len(), 3);
	/// ```
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// The number of objects currently being used.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(69);
	///
	/// // use 3 objects
	/// let foo = pool.take();
	/// let bar = pool.take();
	/// let baz = pool.take();
	///
	/// assert_eq!(pool.in_use(), 3);
	///
	/// // return them
	/// drop((foo, bar, baz));
	///
	/// assert_eq!(pool.in_use(), 0);
	/// ```
	pub fn in_use(&self) -> usize {
		Arc::weak_count(&self.inner)
	}

	/// Whether the pool is empty.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(69);
	/// assert!(pool.is_empty());
	///
	/// // add an object to the pool
	/// let foo = pool.take();
	/// drop(foo);
	/// assert_eq!(pool.is_empty(), false);
	///
	/// // take it back out
	/// let foo = pool.take();
	/// assert!(pool.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	/// Whether the pool is full.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(1);
	/// assert_eq!(pool.is_full(), false);
	///
	/// // add an object to the pool
	/// let foo = pool.take();
	/// drop(foo);
	/// assert!(pool.is_full());
	///
	/// // take it back out
	/// let foo = pool.take();
	/// assert_eq!(pool.is_full(), false);
	/// ```
	pub fn is_full(&self) -> bool {
		self.inner.is_full()
	}

	/// The maximum capacity of the pool.
	///
	/// ```
	/// # use dynamic_pooling::Pool as HiddenPool;
	/// # type Pool = HiddenPool<String>;
	/// let pool = Pool::new(69);
	/// assert_eq!(pool.capacity(), 69);
	/// ```
	pub fn capacity(&self) -> usize {
		self.inner.capacity()
	}

	/// The spare capacity of the pool.
	pub fn spare_capacity(&self) -> usize {
		self.capacity() - self.len()
	}

	/// Attach an object to the pool.
	pub fn attach(&self, object: T) -> Pooled<T> {
		Pooled::new(object, self)
	}
}

/// This returns a reference to the same [`Pool`].
impl<T: Default + Reset> Clone for Pool<T> {
	fn clone(&self) -> Self {
		Self {
			inner: Arc::clone(&self.inner),
		}
	}
}
