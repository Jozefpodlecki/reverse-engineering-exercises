use core::arch::asm;
use core::arch::x86_64::{__cpuid, __cpuid_count};
use core::mem::MaybeUninit;
use core::ptr;
use winapi::shared::minwindef::{FALSE, FILETIME};
use winapi::um::debugapi::{DebugBreak, IsDebuggerPresent};
use winapi::um::errhandlingapi::{RaiseFailFastException, SetUnhandledExceptionFilter, UnhandledExceptionFilter, LPTOP_LEVEL_EXCEPTION_FILTER};
use winapi::um::processthreadsapi::{ExitProcess, GetCurrentProcessId, GetCurrentThreadId, IsProcessorFeaturePresent};
use winapi::um::profileapi::QueryPerformanceCounter;
use winapi::um::sysinfoapi::GetSystemTimeAsFileTime;
use winapi::um::winnt::{RtlCaptureContext, CONTEXT, EXCEPTION_POINTERS, EXCEPTION_RECORD, LARGE_INTEGER, PEXCEPTION_POINTERS};

use crate::logger::ConsoleLogger;

#[no_mangle]
pub extern "C" fn __CxxFrameHandler3() {}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *dest.add(i) = *src.add(i); }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *dest.add(i) = c as u8; }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

static mut _security_cookie: u64 = 0x2b992ddfa232;
static mut _security_cookie_complement: u64 = !0x2b992ddfa232;
static mut IS_INITIALIZED_AS_DLL: bool = false;

#[repr(u8)]
#[derive(PartialEq)]
pub enum ScrtModuleType {
    EXE = 0,
    DLL = 1,
    Unknown = 255,
}

pub static mut __memset_fast_string_threshold: usize = 0;
pub static mut __memset_nt_threshold: isize = 0;
pub static mut __favor: u32 = 0;
pub static mut __isa_available: u32 = 0;
pub static mut __isa_enabled: u32 = 0;
pub static mut __isa_inverted: u64 = !0u64;
pub static mut __avx10_version: u32 = 0;

/// Read XCR0 using XGETBV (returns (low, high) -> use low)
#[inline]
unsafe fn read_xcr0() -> u64 {
    let eax: u32;
    let edx: u32;
    // XCR = 0
    asm!(
        "xgetbv",
        in("ecx") 0u32,
        out("eax") eax,
        out("edx") edx,
        options(nomem, nostack, preserves_flags)
    );
    ((edx as u64) << 32) | (eax as u64)
}

#[no_mangle]
pub extern "C" fn __scrt_acquire_startup_lock() -> bool { true }

#[no_mangle]
pub extern "C" fn __scrt_release_startup_lock(_lock: bool) {}

#[no_mangle]
pub extern "C" fn __scrt_current_native_startup_state() -> u8 { 0 }

#[no_mangle]
pub extern "C" fn _initterm_e(_start: *const (), _end: *const ()) -> i32 { 0 }

#[no_mangle]
pub extern "C" fn _initterm(_start: *const (), _end: *const ()) {}

#[no_mangle]
pub extern "C" fn __scrt_get_dyn_tls_init_callback() -> *mut Option<extern "C" fn()> {
    static mut EMPTY: Option<extern "C" fn()> = None;
    unsafe { &mut EMPTY }
}

#[no_mangle]
pub extern "C" fn __scrt_get_dyn_tls_dtor_callback() -> *mut Option<extern "C" fn()> {
    static mut EMPTY: Option<extern "C" fn()> = None;
    unsafe { &mut EMPTY }
}

#[no_mangle]
pub extern "C" fn __scrt_is_nonwritable_in_current_image(_ptr: *const ()) -> bool { true }

#[no_mangle]
pub extern "C" fn _guard_dispatch_icall_nop() {}

#[no_mangle]
pub extern "C" fn _register_thread_local_exe_atexit_callback(_cb: extern "C" fn()) {}

#[no_mangle]
pub extern "C" fn _get_initial_narrow_environment() -> *mut *mut u8 { core::ptr::null_mut() }

#[no_mangle]
pub extern "C" fn __p___argv() -> *mut *mut *mut u8 {
    static mut EMPTY_ARGV: *mut *mut u8 = core::ptr::null_mut();
    unsafe { &mut EMPTY_ARGV }
}

#[no_mangle]
pub extern "C" fn __p___argc() -> *mut i32 {
    static mut EMPTY_ARGC: i32 = 0;
    unsafe { &mut EMPTY_ARGC }
}

#[no_mangle]
pub extern "C" fn __scrt_is_managed_app() -> bool { false }

#[no_mangle]
pub extern "C" fn _cexit() {}

#[no_mangle]
pub extern "C" fn __scrt_uninitialize_crt(_b1: bool, _b2: bool) {}

#[no_mangle]
pub extern "C" fn exit(code: u32) -> ! {
    unsafe { ExitProcess(code) }
    unsafe { core::hint::unreachable_unchecked() } 
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *mut *mut u8, _env: *mut *mut u8) -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn __isa_available_init() -> i32 {
	unsafe {
        // CPUID(0) - highest basic leaf + vendor string in EBX/EDX/ECX
        let c0 = __cpuid(0);
        let highest_basic = c0.eax as u32;
        // vendor string: EBX, EDX, ECX
        // "Genu" = 0x756e6547 , "ineI" = 0x49656e69 , "ntel" = 0x6c65746e
        const VENDOR_EBX: u32 = 0x756e6547; // "Genu"
        const VENDOR_EDX: u32 = 0x49656e69; // "ineI"
        const VENDOR_ECX: u32 = 0x6c65746e; // "ntel"

        // CPUID(1) - version/info
        let c1 = __cpuid(1);
        // puVar2[3] in your decomp -> c1.edx
        let v_edx = c1.edx;
        // *puVar2 in decomp -> c1.eax
        let v_eax = c1.eax;

        // Check vendor == "GenuineIntel"
        if c0.ebx == VENDOR_EBX && c0.edx == VENDOR_EDX && c0.ecx == VENDOR_ECX {
            // uVar8 = *puVar2 & 0xfff3ff0
            let mut uvar8 = (v_eax as u32) & 0x0fff3ff0u32; // mask from decomp (note adjusted)
            // Set thresholds
            __memset_fast_string_threshold = 0x8000;
            __memset_nt_threshold = -1; // as in decomp
            // check various microcode/stepping patterns
            // Decomp compared uVar8 against several constants:
            // 0x106c0, 0x20660, 0x20670, range (0x30650 .. 0x30650+0x21)
            if uvar8 == 0x00106c0u32
                || uvar8 == 0x0020660u32
                || uvar8 == 0x0020670u32
                || (uvar8 >= 0x0030650u32 && uvar8 <= 0x0030650u32 + 0x20)
            {
                __favor = __favor | 1;
            }
        }

        // Defaults
        let mut u8_val: u32 = 0;
        let mut u9_val: u32 = 0;
        let mut u10_val: u32 = 0;
        let mut u11_val: u64 = 0;

        if highest_basic > 6 {
            // CPUID leaf 7, subleaf 0: extended feature flags
            let c7 = __cpuid_count(7, 0);
            u8_val = c7.ebx; // ebx -> features bits (like BMI etc.)
            u9_val = c7.ecx; // ecx
            if (u8_val >> 9) & 1 != 0 {
                __favor = __favor | 2;
            }
            if (c7.eax as i32) > 0 {
                // read again (decomp does this), get dword at offset +8 -> edx?
                // we'll use edx from cpuid(7) as u10
                u10_val = c7.edx;
            }
            if highest_basic > 0x23 {
                // leaf 0x24
                let c24 = __cpuid(0x24);
                u11_val = (c24.ebx as u64) << 32 | (c24.ecx as u64); // somewhat heuristic
                // original decomp used *(uint *)(lVar4 + 4) -> probably ecx
                // store something in u11_val for later extraction
                u11_val = c24.ecx as u64;
            }
        }

        __isa_available = 1;
        __isa_enabled = 2;
        let mut new_inverted = __isa_inverted & !1u64; // clear bit0

        // check bit 20 of v_edx (v_edx >> 0x14)
        if ((v_edx >> 0x14) & 1) != 0 {
            __isa_available = 2;
            __isa_enabled = 6;
            new_inverted = __isa_inverted & !0x10u64; // clear bit 4? original had 0xffffffffffffffee mask
        }
        __isa_inverted = new_inverted;

        // check bit 27 (0x1b) of v_edx
        if ((v_edx >> 0x1b) & 1) != 0 {
            // check bit 28 (0x1c) and XCR0 bits
            // read XCR0
            let xcr0 = read_xcr0();
            let bvar6 = (xcr0 & 0xff) as u8;
            if ((v_edx >> 0x1c) & 1) != 0 && (bvar6 & 6) == 6 {
                __isa_available = 3;
                let mut tmp_inv = __isa_inverted;
                let mut tmp_enabled = __isa_enabled | 8;

                if (u8_val & 0x20) != 0 {
                    __isa_available = 5;
                    tmp_inv = __isa_inverted & !(1u64 << 1); // & ~2?
                    tmp_enabled = __isa_enabled | 0x28;
                    // additional check: (u8_val & 0xd0030000) == 0xd0030000 && (bvar6 & 0xe0) == 0xe0
                    if (u8_val & 0xd003_0000) == 0xd003_0000 && (bvar6 & 0xe0) == 0xe0 {
                        __isa_enabled = __isa_enabled | 0x68;
                        __isa_available = 6;
                        tmp_inv = __isa_inverted & !(0x26u64); // mask like decomp
                        tmp_enabled = __isa_enabled;
                    }
                }

                __isa_enabled = tmp_enabled;
                __isa_inverted = tmp_inv;

                // if u9_val has bit 0x17 set
                if ((u9_val >> 0x17) & 1) != 0 {
                    __isa_inverted = __isa_inverted & !(1u64 << 24); // clear corresponding bit in inverted
                }

                // AVX10 related: if u10 has bit 0x13 and XCR0 bits set
                if ((u10_val >> 0x13) & 1) != 0 && (bvar6 & 0xe0) == 0xe0 {
                    __avx10_version = (u11_val as u32) & 0x400ff;
                    // update inverted mask similarly to decomp:
                    let part = (((u11_val >> 16) as u32) & 6) as u64;
                    __isa_inverted = !((part as u64) | 0x1000029) & __isa_inverted;
                    if (1u8 < (__avx10_version as u8)) {
                        __isa_inverted = __isa_inverted & !(1u64 << 6); // clear some bit
                    }
                }
            }

            // if u10 has bit 0x15 and XCR0 has bit 0x13
            if ((u10_val >> 0x15) & 1) != 0 && ((xcr0 >> 0x13) & 1) != 0 {
                __isa_inverted = __isa_inverted & !(1u64 << 7);
            }
        }

        // function returns 0 in the decompilation
        0
    }
}

#[no_mangle]
pub extern "C" fn is_write_vectored() -> bool { true }

#[no_mangle]
pub extern "C" fn __crt_debugger_hook(code: u32) -> ! {
	// .data offset?
	// __scrt_debugger_hook_flag = 0;
	unsafe { core::hint::unreachable_unchecked() } 
}


#[no_mangle]
pub extern "C" fn __scrt_fastfail(code: u32) -> ! {
	unsafe {
        const PF_FASTFAIL_AVAILABLE: u32 = 0x17;

        // If processor supports FastFail, use the fast path.
        if IsProcessorFeaturePresent(PF_FASTFAIL_AVAILABLE) != 0 {
            // Prefer RaiseFailFastException if available; it does not return.
            // Passing NULL for exception record and context per typical usage.
            // If RaiseFailFastException is missing on some platform, you could fallback to `int 0x29` via asm.
            // We'll call it and then loop to satisfy diverging semantics (it shouldn't return).
            let _ = RaiseFailFastException(ptr::null_mut(), ptr::null_mut(), 0);
            // If it returns for some reason, terminate.
            ExitProcess(code);
        }

        // Non-fastfail path: let debugger hook run (gives debugger a chance)
        __crt_debugger_hook(3);

        // Prepare a CONTEXT and capture current context
        let mut ctx_uninit: MaybeUninit<CONTEXT> = MaybeUninit::uninit();
        RtlCaptureContext(ctx_uninit.as_mut_ptr());
        let mut ctx = ctx_uninit.assume_init();

        // Build a tidy EXCEPTION_RECORD. The CRT used 0x40000015 in the disasm; keep it.
        let mut er_uninit: MaybeUninit<EXCEPTION_RECORD> = MaybeUninit::uninit();
        // zero-initialize the record
        let er_ptr = er_uninit.as_mut_ptr();
        // SAFETY: fill fields explicitly
        // We'll write through the pointer to set the fields we need.
        // Since EXCEPTION_RECORD layout may have more fields, we'll zero the memory first:
        core::ptr::write_bytes(er_ptr as *mut u8, 0, core::mem::size_of::<EXCEPTION_RECORD>());

        let er: &mut EXCEPTION_RECORD = &mut *er_ptr;
        er.ExceptionCode = 0x4000_0015; // same code as in decompiled CRT
        er.ExceptionFlags = 1; // EXCEPTION_NONCONTINUABLE (1)
        er.ExceptionRecord = ptr::null_mut();
        // ExceptionAddress: best effort — use instruction pointer from CONTEXT
        // On x86_64 CONTEXT has Rip field; on x86 it has Eip. We attempt Rip; if not present behavior is platform-dependent.
        // We'll set ExceptionAddress to the address of the captured IP if accessible.
        #[cfg(target_arch = "x86_64")]
        {
            use winapi::ctypes::c_void;

            er.ExceptionAddress = ctx.Rip as *mut c_void;
        }
        #[cfg(target_arch = "x86")]
        {
            er.ExceptionAddress = (ctx.Eip as usize) as *mut core::ffi::c_void;
        }

        // Build EXCEPTION_POINTERS pointing to our records
        let mut exinfo_uninit: MaybeUninit<EXCEPTION_POINTERS> = MaybeUninit::uninit();
        let exinfo_ptr = exinfo_uninit.as_mut_ptr();
        // set fields
        (*exinfo_ptr).ExceptionRecord = er as *mut EXCEPTION_RECORD;
        (*exinfo_ptr).ContextRecord = &mut ctx as *mut CONTEXT;

        // Clear top-level unhandled exception filter so default handling happens
        SetUnhandledExceptionFilter(None);

        // Call UnhandledExceptionFilter with our constructed pointers
        let res = UnhandledExceptionFilter(exinfo_ptr as PEXCEPTION_POINTERS);

        // If exception was not handled by WER/filters AND no debugger attached, call debugger hook or break
        if res == 0 && !IsDebuggerPresent() == FALSE {
            __crt_debugger_hook(3);
            // If still nothing, trigger a breakpoint
            DebugBreak();
        }

        // As a last resort, terminate process with the original code
        ExitProcess(code);
		unsafe { core::hint::unreachable_unchecked() } 
    }
}

#[no_mangle]
pub extern "C" fn __security_init_cookie() {
	unsafe {
		if _security_cookie == 0x2b992ddfa232 {
            let mut ft: FILETIME = core::mem::zeroed();
            GetSystemTimeAsFileTime(&mut ft);

            let mut cookie_acc: u64 =
                ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);

            let thread_id = GetCurrentThreadId() as u64;
            let process_id = GetCurrentProcessId() as u64;
            cookie_acc ^= thread_id;
            cookie_acc ^= process_id;

			let mut li = MaybeUninit::<LARGE_INTEGER>::uninit();
            QueryPerformanceCounter(li.as_mut_ptr());
			let qpc: i64 = *li.assume_init().QuadPart();

            cookie_acc ^= qpc as u64;

            cookie_acc &= 0x0000_ffff_ffff_ffff;

            _security_cookie = if cookie_acc == 0x2b992ddfa232 {
                0x2b992ddfa233
            } else {
                cookie_acc
            };
        }

        _security_cookie_complement = !_security_cookie;	
	};
}

#[no_mangle]
pub extern "C" fn __scrt_initialize_crt(param: ScrtModuleType) -> bool {
    unsafe {
        if param == ScrtModuleType::DLL {
            IS_INITIALIZED_AS_DLL = true;
        }

        __isa_available_init();

        let mut result = is_write_vectored();
        if result {
            result = is_write_vectored();
            if !result {
                return false;
            }

            is_write_vectored();
        }

        result
    }
}

#[no_mangle]
pub extern "C" fn __scrt_common_main_seh() -> u32 {
    unsafe {
        let logger = ConsoleLogger::new();

        let mut success = __scrt_initialize_crt(ScrtModuleType::EXE);

        if !success {
            __scrt_fastfail(7);
        }

        let mut already_initialized = false;
        let lock = __scrt_acquire_startup_lock();

        match __scrt_current_native_startup_state() {
            0 => { // uninitialized
                // mark as initializing
                // call constructors
                let ret = _initterm_e(core::ptr::null(), core::ptr::null());
                if ret != 0 {
                    return 0xff;
                }
                _initterm(core::ptr::null(), core::ptr::null());
                // mark as initialized
            }
            1 => { // initializing
                __scrt_fastfail(7);
            }
            _ => already_initialized = true,
        }


        __scrt_release_startup_lock(lock);

        // TLS init callback
        let tls_init = __scrt_get_dyn_tls_init_callback();
        if let Some(cb) = *tls_init {
            if __scrt_is_nonwritable_in_current_image(cb as *const _) {
                _guard_dispatch_icall_nop();
            }
        }

        // TLS dtor callback
        let tls_dtor = __scrt_get_dyn_tls_dtor_callback();
        if let Some(cb) = *tls_dtor {
            if __scrt_is_nonwritable_in_current_image(cb as *const _) {
                _register_thread_local_exe_atexit_callback(cb);
            }
        }

        let env = _get_initial_narrow_environment();
   
        let argv_ptr = __p___argv();

        let argv = *argv_ptr;

        let argc_ptr = __p___argc();

        let argc = *argc_ptr;

        let ret = main(argc, argv, env);

        if __scrt_is_managed_app() {
            if !already_initialized {
                _cexit();
            }
            __scrt_uninitialize_crt(true, false);
            
            return ret;
        }

        exit(ret);
    }
}