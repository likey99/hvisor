use spin::Mutex;
use numeric_enum_macro::numeric_enum;

// use libc::cpu_set_t;
use crate::arch::Stage2PageTable;
use crate::config::{CellConfig, HvSystemConfig};
use crate::error::HvResult;
use crate::memory::addr::{GuestPhysAddr, HostPhysAddr};
use crate::memory::{GenericPageTableImmut, MemFlags, MemoryRegion, MemorySet};
use crate::mmio::{MMIORegionLocation, MMIORegionHandler};

numeric_enum! {
    #[repr(u64)]
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum CellState {
        HVCellRunning = 0,
        HVCellRunningLocked = 1,
        HVCellShutDown = 2,
        HVCellFailed = 3,
        HVCellFailedCommRev = 4,
    }
}

#[derive(Debug)]
pub struct Cell<'a> {
    /// Cell configuration.
    pub config: CellConfig<'a>,
    /// Guest physical memory set.
    pub gpm: MemorySet<Stage2PageTable>,
    /// Cell's CPU set.
    // pub cpu_set: &'a cpu_set_t,
    /// Pointer to next cell in the system.
    // pub next: Option<&'a mut Cell<'a>>,
    /// Lock protecting changes to mmio_locations, mmio_handlers, and num_mmio_regions.
    pub mmio_region_lock: Mutex<()>,
    /// Generation counter of mmio_locations, mmio_handlers, and num_mmio_regions.
    pub mmio_generation: u64,
    /// Number of pages used for storing cell-specific states and configuration data.
	pub data_pages: u64,
    /// True while the cell can be loaded by the root cell.
    pub loadable: bool,
    /// MMIO region description table.
    pub mmio_locations: MMIORegionLocation,
    /// MMIO region handler table.
    pub mmio_handlers: MMIORegionHandler,
    /// Number of MMIO regions in use.
	pub num_mmio_regions: u32,
	/// Maximum number of MMIO regions.
	pub max_mmio_regions: u32,
}

impl<'a> Cell<'a> {
    fn new_root() -> HvResult<Self> {
        let sys_config = HvSystemConfig::get();
        let cell_config = sys_config.root_cell.config();
        let mmcfg_start = sys_config.platform_info.pci_mmconfig_base;
        let mmcfg_size = (sys_config.platform_info.pci_mmconfig_end_bus + 1) as u64 * 256 * 4096;
        let hv_phys_start = sys_config.hypervisor_memory.phys_start as usize;
        let hv_phys_size = sys_config.hypervisor_memory.size as usize;

        let mut gpm: MemorySet<Stage2PageTable> = MemorySet::new();

        // Map hypervisor memory to the empty page.
        // gpm.insert(MemoryRegion::new_with_empty_mapper(
        //     hv_phys_start,
        //     hv_phys_size,
        //     MemFlags::READ | MemFlags::NO_HUGEPAGES,
        // ))?;

        gpm.insert(MemoryRegion::new_with_offset_mapper(
            hv_phys_start as GuestPhysAddr,
            hv_phys_start as HostPhysAddr,
            hv_phys_size as usize,
            MemFlags::READ | MemFlags::NO_HUGEPAGES,
        ))?;

        // Map all physical memory regions.
        gpm.insert(MemoryRegion::new_with_offset_mapper(
            0x09000000 as GuestPhysAddr,
            0x09000000 as HostPhysAddr,
            0x37000000 as usize,
            MemFlags::READ | MemFlags::WRITE,
        ))?;
        gpm.insert(MemoryRegion::new_with_offset_mapper(
            0x40000000 as GuestPhysAddr,
            0x40000000 as HostPhysAddr,
            0x3fb00000 as usize,
            MemFlags::READ | MemFlags::WRITE,
        ))?;
        gpm.insert(MemoryRegion::new_with_offset_mapper(
            0x7fb00000 as GuestPhysAddr,
            0x7fb00000 as HostPhysAddr,
            0x100000 as usize,
            MemFlags::READ | MemFlags::WRITE,
        ))?;
        // for region in cell_config.mem_regions() {
        //     gpm.insert(MemoryRegion::new_with_offset_mapper(
        //         region.virt_start as GuestPhysAddr,
        //         region.phys_start as HostPhysAddr,
        //         region.size as usize,
        //         region.flags,
        //     ))?;
        // }

        gpm.insert(MemoryRegion::new_with_offset_mapper(
            mmcfg_start as GuestPhysAddr,
            mmcfg_start as HostPhysAddr,
            mmcfg_size as usize,
            MemFlags::READ | MemFlags::WRITE | MemFlags::IO,
        ))?;

        trace!("Guest phyiscal memory set: {:#x?}", gpm);

        let mut mmio_loca = MMIORegionLocation::new();
        let mut mmio_hdl = MMIORegionHandler::new();

        Ok(Self {
            config: cell_config,
            gpm,
            // next: None,
            mmio_region_lock: Mutex::new(()),
            mmio_generation: 0,
            data_pages: 0,
            loadable: false,
            mmio_locations: mmio_loca,
            mmio_handlers: mmio_hdl,
            num_mmio_regions: 0,
            max_mmio_regions: 0,
        })
    }

    fn cell_suspend() -> HvResult {
        let _cpu = 0;
        Ok(())
    }
}

static ROOT_CELL: spin::Once<Cell> = spin::Once::new();

pub fn root_cell<'a>() -> &'a Cell<'a> {
    ROOT_CELL.get().expect("Uninitialized root cell!")
}

pub fn init() -> HvResult {
    let root_cell = Cell::new_root()?;
    info!("Root cell init end.");
    debug!("{:#x?}", root_cell);

    ROOT_CELL.call_once(|| root_cell);
    Ok(())
}
