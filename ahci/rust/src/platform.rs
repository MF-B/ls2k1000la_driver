use core::arch::asm;

// 等待数毫秒
#[unsafe(no_mangle)]
pub extern "C" fn ahci_mdelay(ms: u32) {
    // TODO: 实现实际的延时逻辑
}

// 同步dcache中所有cached和uncached访存请求
#[unsafe(no_mangle)]
pub extern "C" fn ahci_sync_dcache() {
    unsafe {
        asm!("dbar 0");
    }
}

// 分配按align字节对齐的内存
#[unsafe(no_mangle)]
pub extern "C" fn ahci_malloc_align(size: u64, align: u32) -> u64 {
    // TODO: 实现实际的内存分配逻辑
    0
}

// 物理地址转换为uncached虚拟地址
#[unsafe(no_mangle)]
pub extern "C" fn ahci_phys_to_uncached(pa: u64) -> u64 {
    pa
}

// cached虚拟地址转换为物理地址
// ahci dma可以接受64位的物理地址
#[unsafe(no_mangle)]
pub extern "C" fn ahci_virt_to_phys(va: u64) -> u64 {
    va
}
