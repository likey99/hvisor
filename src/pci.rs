use crate::config::HvPciDevice;
use crate::cell::Cell;

pub const PCI_NUM_BARS: usize = 6;

#[repr(C)]
pub struct PciDevice<'a> {
    /// Reference to static device configuration.
    pub info: &'a HvPciDevice,
    /// Owning cell.
    pub cell: &'a Cell<'a>,
    /// Shadow BAR
    pub bar: [u32; PCI_NUM_BARS],
}