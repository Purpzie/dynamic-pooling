use alloc::{
	boxed::Box,
	collections::{BinaryHeap, VecDeque},
	string::String,
	vec::Vec,
};
#[cfg(feature = "std")]
use std::{
	collections::{HashMap, HashSet},
	ffi::OsString,
	path::PathBuf,
};

/// Trait for types that can be reset to their [`Default`] while keeping allocated memory.
///
/// Types should not implement this trait if memory is deallocated when reset, such as
/// [`BTreeMap`](std::collections::BTreeMap).
///
/// ```
/// # use dynamic_pooling::Reset;
/// struct MyEpicStruct {
/// 	vec: Vec<bool>,
/// 	option: Option<bool>,
/// }
///
/// impl Reset for MyEpicStruct {
/// 	fn reset(&mut self) {
/// 		self.vec.clear();
/// 		self.option = None;
/// 	}
/// }
/// ```
pub trait Reset {
	/// Reset to the default state while keeping allocated memory.
	fn reset(&mut self);
}

impl<T> Reset for &mut T
where
	T: Reset,
{
	fn reset(&mut self) {
		Reset::reset(&mut **self);
	}
}

impl<T> Reset for Box<T>
where
	T: Reset,
{
	fn reset(&mut self) {
		Reset::reset(&mut **self);
	}
}

impl<T> Reset for Vec<T> {
	fn reset(&mut self) {
		self.clear();
	}
}

impl Reset for String {
	fn reset(&mut self) {
		self.clear();
	}
}

impl<T> Reset for VecDeque<T> {
	fn reset(&mut self) {
		self.clear();
	}
}

impl<T> Reset for BinaryHeap<T> {
	fn reset(&mut self) {
		self.clear();
	}
}

#[cfg(feature = "std")]
impl Reset for PathBuf {
	fn reset(&mut self) {
		self.clear();
	}
}

#[cfg(feature = "std")]
impl Reset for OsString {
	fn reset(&mut self) {
		self.clear();
	}
}

#[cfg(feature = "std")]
impl<T, U> Reset for HashMap<T, U> {
	fn reset(&mut self) {
		self.clear();
	}
}

#[cfg(feature = "std")]
impl<T, U> Reset for HashSet<T, U> {
	fn reset(&mut self) {
		self.clear();
	}
}

macro_rules! tuple_hell {
	($(($($letter:ident:$number:tt),+$(,)?))+) => {$(
		impl<$($letter),+> Reset for ($($letter),+,) where $($letter: Reset),+ {
			fn reset(&mut self) {
				$(self.$number.reset();)+
			}
		}
	)+}
}

tuple_hell! {
	(A:0)
	(A:0, B:1)
	(A:0, B:1, C:2)
	(A:0, B:1, C:2, D:3)
	(A:0, B:1, C:2, D:3, E:4)
	(A:0, B:1, C:2, D:3, E:4, F:5)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11)
	(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12)
}
