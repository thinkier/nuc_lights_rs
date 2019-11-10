#[repr(C)]
pub struct Pointer<X>(
	pub X
);

impl<X> Pointer<X> {
	pub fn as_ptr<T>(&mut self) -> *mut T {
		let ptr: *mut _ = &mut self.0;

		return ptr.cast();
	}
}