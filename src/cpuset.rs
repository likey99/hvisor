
#[repr(C)]
#[derive(Debug)]
/// Describes a CPU set.
pub struct CpuSet {
    /// Maximum CPU ID expressible with this set.
    pub max_cpu_id: u64,
    /// Bitmap of CPUs in the set.
    pub bitmap: [u64; 1],
}