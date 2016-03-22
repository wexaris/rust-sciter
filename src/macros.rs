//! Macros

/// Rust string to UTF-8 conversion. See also `utf::u2s`.
///
/// # Example:
///
/// ```ignore
/// let (cstr, len) = s2u("hello"); // ffi::CString
/// libc::printf("%.*hs", len, cstr.as_ptr());
/// ```
///
#[macro_export]
macro_rules! s2u {
	($s:expr) => ( $crate::utf::s2un($s) )
}


/// Rust string to UTF-16 conversion. See also `utf::w2s`.
///
/// # Example:
///
/// ```ignore
/// let (cwstr, len) = s2w("hello"); // Vec<u16>
/// libc::printf("%.*ws", len, cwstr.as_ptr());
/// ```
///
#[macro_export]
macro_rules! s2w {
	($s:expr) => ( $crate::utf::s2vecn($s) )
}


/// UTF-16 to `String` conversion.
#[macro_export]
macro_rules! w2s {
	($s:expr) => ( $crate::utf::w2s($s) )
}


/// UTF-8 to `String` conversion.
#[macro_export]
macro_rules! u2s {
	($s:expr) => ( $crate::utf::u2s($s) )
}


/// Pack arguments to call the sciter script function.
#[macro_export]
macro_rules! pack_args {
	() => { $crate::value::Value::pack_args(&[]) };

	( $($s:expr),* ) => {
		{
			let args = [
			$(
				$crate::value::Value::from($s)
			 ),*
			];
			$crate::value::Value::pack_args(&args)
		}
	};
}

/// Make `[Value]` sequence to call the sciter script function.
#[macro_export]
macro_rules! make_args {
	() => { { let args : [$crate::value::Value; 0] = []; args } };

	( $($s:expr),* ) => {
		{
			let args = [
			$(
				$crate::value::Value::from($s)
			 ),*
			];
			args
		}
	};
}

/// Declare handle type (native pointer).
#[macro_export]
macro_rules! MAKE_HANDLE {
	($name:ident, $inner:ident) => {
		#[repr(C)]
		pub struct $inner { _unused: usize }
		pub type $name = *mut $inner;
	};
}
