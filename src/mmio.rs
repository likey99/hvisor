#[repr(C)]
#[derive(Debug)]
/// MMIO region coordinates.
pub struct MMIORegionLocation {
    /// Start address of the region.
	pub start: u64,
	/// Region size.
	pub size: u64,
}

impl MMIORegionLocation {
	pub fn new() -> Self {
		Self {
			start: 0,
			size: 0 
		}
	}
}

#[repr(C)]
#[derive(Debug)]
/// MMIO region access handler description.
pub struct MMIORegionHandler {
	// / Access handling function.
	// pub function: mmio_handler,
	// / Argument to pass to the function.
	// pub arg: u64,
}

impl MMIORegionHandler {
	pub fn new() -> Self {
		Self {  }
	}
}