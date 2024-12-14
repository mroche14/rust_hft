// Provides a function to prefetch memory into cache to reduce cache miss latency.
// Works on x86_64 systems. If on other arch, adapt accordingly.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_prefetch;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const _MM_HINT_T0: i32 = 3;

pub fn maybe_prefetch(ptr: *const u8, distance: usize, enabled: bool) {
    if enabled {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            _mm_prefetch(ptr.add(distance) as *const i8, _MM_HINT_T0);
        }
    }
}
