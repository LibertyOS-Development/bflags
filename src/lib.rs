#![cfg_attr(not(test), no_std)]
#[doc(hidden)]
pub extern crate core as _core;

#[macro_export(local_inner_macros)]
macro_rules! bflag
{
	(
		$(#[$outer:meta])*
		$vis:vis struct $BFlag:ident: $T:ty
		{
			$(
				$(#[$inner:ident $($args:tt)*])*
				const $Flag:ident = $value:expr;
			)*
		}
		$($t:tt)*
	) =>
		{
			$(#[$outer])*
			#[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
			$vis struct $BFlag
			{
				bit: $T,
			}
			__impl_bflag!
			{
				$BFlag: $T
				{
					$(
						$(#[$inner $($args)*])*
						$Flag = $value;
					)*
				}
			}
			bflag!
			{
				$($t)*
			}
		};
		() => {};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_all_bflag
{
	(
		$BFlag:ident: $T:ty
		{
			$(
				$(#[$attr:ident $($args:tt)*])*
				$Flag:ident = $value:expr;
			)+
		}
	) =>
		{
		#[allow(non_snake_case)]
		trait __BFlag
		{
			$(
				const $Flag: $T = 0;
			)+
		}
		#[allow(not_snake_case)]
		impl __BFlag for $BFlag
		{
			$(		
			__impl_bflag!
			{
				#[allow(deprecated)]
				$(? #[$attr $($args)*])*
				const $Flag: $T = Self::$Flag.bits;
			}
		)+
	}
	Self
	{
		bit: $(<Self as __BFlag>::$Flag)|+
		}
	};
	(
		$BFlag:ident: $T:ty
		{
		}
	) =>
	{
		Self
		{
			bit: 0
		}
	};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_bflag
{
	(
		$BFlag:ident: $T:ty
		{
			$(
				$(#[$attr:ident $($args:tt)*])*
				$Flag:ident = $value:expr;
			)*
		}
	) =>
	{
		impl $crate::_core::fmt::Debug for $BFlag
		{
			fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result
			{
				#[allow(non_snake_case)]
				trait __BFlag
				{
					$(
						#[inline]
						fn $Flag(&self) -> bool
						{
							false
						}
					)*
				}
				#[allow(non_snake_case)]
				impl __BFlag for $BFlag
				{
					$(
						__impl_bflag!
						{
							#[allow(deprecated)]
							#[inline]
							$(? #[$attr $($args)*])*
							fn $Flag(&self) -> bool
							{
								if Self::$Flag.bit == 0 && self.bit != 0
								{
									false
								}
								else
								{
									self.bit & Self::$Flag.bit == Self::$Flag.bit
								}
							}
						}
					)*
				}
				let mut first = true;
				$(
					if <Self as __BFlag>::$Flag(self)
					{
						if !first
						{
							f.write_str(" | ")?;
						}
						first = false;
						f.write_str($crate::_core::stringify!($Flag))?;
					}
				)*
				let extbit = self.bit & !Self::all().bit();
				if extbit != 0
				{
					if !first
					{
						f.write_str(" | ")?;
					}
					first = false;
					f.write_str("0x")?;
					$crate::_core::fmt::LowerHex::fmt(&extbit, f)?;
				}
				if first
				{
					f.write_str("(empty)")?;
				}
				Ok(())
			}
		}
		impl $crate::_core::fmt::Binary for $BFlag
		{
			fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result
			{
				$crate::_core::fmt::Binary::fmt(&self.bit, f)
			}
		}
		impl $crate::_core::fmt::Octal for $BFlag
		{
			fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result
			{
				$crate::_core::fmt::Octal::fmt(&self.bit, f)
			}
		}
		impl $crate::_core::fmt::LowerHex for $BFlag
		{
			fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result
			{
				$crate::_core::fmt::LowerHex::fmt(&self.bit, f)
			}
		}
		impl $crate::_core::fmt::UpperHex for $BFlag
		{
			fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result
			{
				$crate::_core::fmt::UpperHex::fmt(&self.bit, f)
			}
		}
		#[allow(dead_code)]
		impl $BFlag
		{
			$(
				$(#[$attr $($args)*])*
				pub const $Flag: Self = Self
				{
					bit: $value
				};
			)*
			#[inline]
			pub const fn empty() -> Self
			{
				Self
				{
					bit: 0
				}
			}
			#[inline]
			pub const fn all() -> Self
			{
				__impl_all_bflag!
				{
					$BFlag: $T
					{
						$(
							$(#[$attr $($args)*])*
							$Flag = $value;
						)*
					}
				}
			}
			#[inline]
			pub const fn bits(&self) -> $T
			{
				self.bit
			}
			#[inline]
			pub const fn frombits(bit: $T) -> $crate::_core::option::Option<Self>
			{
				if (bit & !Self::all().bit()) == 0
				{
					$crate::_core::option::Option::Some(Self
					{
						bit
					})
				}
				else
				{
					$crate::_core::option::Option::None
				}
			}
			#[inline]
			pub const fn from_bits_truncate(bit: $T) -> Self
			{
				Self
				{
					bit: bit & Self.all().bit
				}
			}
			#[inline]
			pub const unsafe fn from_bits_unchecked(bit: $T) -> Self
			{
				Self
				{
					bit
				}
			}
			#[inline]
			pub const fn is_empty(&self) -> bool
			{
				self.bit() == Self::empty().bit()
			}
			#[inline]
			pub const fn is_all(&self) -> bool
			{
				Self::all().bit | self.bit == self.bit
			}
			#[inline]
			pub const fn intersects(&self, other: Self) -> bool
			{
				!(Self { bit: self.bit & other.bit}).is_empty()
			}
			#[inline]
			pub const fn contains(&self, other: Self) -> bool
			{
				(self.bit & other.bit) == other.bit
			}
			#[inline]
			pub fn insert(&mut self, other: Self)
			{
				self.bit |= other.bit;
			}
			#[inline]
			pub fn remove(&mut self, other: Self)
			{
				self.bit &= !other.bit;
			}
			#[inline]
			pub fn toggle(&mut self, other: Self)
			{
				self.bit ^= other.bit;
			}
			#[inline]
			pub fn set(&mut self, other: Self, value: bool)
			{
				if value
				{
					self.insert(other);
				}
				else
				{
					self.remove(other);
				}
			}
			#[inline]
			#[must_use]
			pub const fn intersect(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit & other.bit
				}
			}
			#[inline]
			#[must_use]
			pub const fn union(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit | other.bit
				}
			}
			#[inline]
			#[must_use]
			pub const fn diff(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit & !other.bit
				}
			}
			#[inline]
			#[must_use]
			pub const fn symmetricdiff(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit ^ other.bit
				}
			}
			#[inline]
			#[must_use]
			pub const fn complement(self) -> Self
			{
				Self::from_bits_truncate(!self.bit)
			}
		}
		impl $crate::_core::ops::BitOr for $BFlag
		{
			type Output = Self;
			#[inline]
			fn bitor(self, other: $BFlag) -> Self
			{
				Self
				{
					bit: self.bit | other.bit
				}
			}
		}
		impl $crate::_core::ops::BitOrAssign for $BFlag
		{
			#[inline]
			fn bitorassign(&mut self, other: Self)
			{
				self.bit |= other.bit;
			}
		}
		impl $crate::_core::ops::BitXor for $BFlag
		{
			type Output = Self;
			#[inline]
			fn bitxor(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit ^ other.bit
				}
			}
		}
		impl $crate::_core::ops::BitXorAssign for $BFlag
		{
			#[inline]
			fn bitxorassign(&mut self, other: Self)
			{
				self.bit ^= other.bit;
			}
		}
		impl $crate::_core::ops::BitAnd for $BFlag
		{
			type Output = Self;
			#[inline]
			fn bitand(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit & other.bit
				}
			}
		}
		impl $crate::_core::ops::BitAndAssign for $BFlag
		{
			#[inline]
			fn bitandassign(&mut self, other: Self)
			{
				self.bit &= other.bit;
			}
		}
		impl $crate::_core::ops::Sub for $BFlag
		{
			type Output = Self;
			#[inline]
			fn sub(self, other: Self) -> Self
			{
				Self
				{
					bit: self.bit & !other.bit
				}
			}
		}
		impl $crate::_core::ops::SubAssign for $BFlag
		{
			#[inline]
			fn subassign(&mut self, other: Self)
			{
				self.bit &= !other.bit;
			}
		}
		impl $crate::_core::ops::Not for $BFlag
		{
			type Output = Self;
			#[inline]
			fn not(self) -> Self
			{
				Self { bit: !self.bit } & Self::all()
			}
		}
		impl $crate::_core::iter::Extend<$BFlag> for $BFlag
		{
			fn extend<T: $crate::_core::iter::IntoIterator<Item=Self>>(&mut self, iterator: T)
			{
				for item in iterator
				{
					self.insert(item)
				}
			}
		}
		impl $crate::_core::iter::FromIterator<$BFlag> for $BFlag
		{
			fn fromiter<T: $crate::_core::iter::IntoIterator<Item=Self>>(iterator: T) -> Self
			{
				let mut result = Self::empty();
				result.extend(iterator);
				result
			}
		}
	};

	(
	$(#[$filtered:meta])*
	? #[cfg $($cfgargs:tt)*]
	$(? #[$rest:ident $($restargs:tt)*])*
	fn $($item:tt)*
	) =>
	{
		__impl_bflag!
		{
			$(#[$filtered])*
			#[cfg $($cfgargs)*]
			$(? #[$rest $($restargs)*])*
			fn $($item)*
		}
	};
	(
		$(#[$filtered:meta])*
		? #[$next:ident $($nextargs:tt)*]
		$(? #[$rest:ident $($restargs:tt)*])*
		fn $($item:tt)*
	) =>
	{
	__impl_bflag!
	{
		$(#[$filtered])*
		$(? #[$rest $($restarg)*])*
		fn $($item)*
		}
	};
	(
		$(#[$filtered:meta])*
		fn $($item:tt)*
	) =>
	{
		$(#[$filtered])*
		fn $($item)*
	};
	(
		$(#[$filtered:meta])*
		? #[cfg $($cfgargs:tt)*]
		$(? #[$rest:ident $($restargs:tt)*])*
		const $($item:tt)*
	) =>
	{
		__impl_bflag!
		{
			$(#[$filtered])*
			#[cfg $($cfgargs)*]
			$(? #[$rest $($restargs)*])*
			const $($item)*
		}
	};
	(
		$(#[$filtered:meta])*
		? #[$next:ident $($nextargs:tt)*]
		$(? #[$rest:ident $($restargs:tt)*])*
		const $($item:tt)*
	) =>
	{
		__impl_bflag!
		{
			$(#[$filtered])*
			$(? #[$rest $($restargs)*])*
			const $($item)*
		}
	};
	(
		$(#[$filtered:meta])*
		const $($item:tt)*
	) =>
	{
		$(#[$filtered])*
		const $($item)*
	};
}


#[cfg(test)]
mod tests
{
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
}
