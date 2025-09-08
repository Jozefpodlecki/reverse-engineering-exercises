use std::{ffi::CString, ptr};

use iced_x86::{Code, Encoder, Instruction, MemoryOperand, Register};
use winapi::{shared::minwindef::{DWORD, LPVOID}, um::{libloaderapi::{GetModuleHandleA, GetProcAddress}, processthreadsapi::CreateThread, winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE}}};

type VirtualAllocFn = unsafe extern "system" fn(
    lpAddress: *mut std::ffi::c_void,
    dwSize: usize,
    flAllocationType: u32,
    flProtect: u32,
) -> *mut std::ffi::c_void;

type VirtualProtectFn = unsafe extern "system" fn(
    lpAddress: *mut std::ffi::c_void,
    dwSize: usize,
    flNewProtect: u32,
    lpflOldProtect: *mut u32,
) -> i32;

type RtlMoveMemoryFn = unsafe extern "system" fn(
    dest: *mut std::ffi::c_void,
    src: *mut std::ffi::c_void,
    length: usize,
);

type CreateThreadFn = unsafe extern "system" fn(
    lpThreadAttributes: *mut std::ffi::c_void,
    dwStackSize: usize,
    lpStartAddress: unsafe extern "system" fn(*mut std::ffi::c_void) -> u32,
    lpParameter: *mut std::ffi::c_void,
    dwCreationFlags: u32,
    lpThreadId: *mut u32,
) -> winapi::shared::ntdef::HANDLE;

type ThreadFn = unsafe extern "system" fn(lpParameter: *mut std::ffi::c_void) -> DWORD;
type ThreadFn1 = unsafe extern "system" fn(lpParameter: *mut winapi::ctypes::c_void) -> DWORD;

type WaitForSingleObjectFn = unsafe extern "system" fn(winapi::shared::ntdef::HANDLE, u32) -> u32;

fn main() {
    unsafe {

        /*
        mov rax, gs:[0x60]        ; RAX = PEB
mov rbx, [rax+0x18]       ; RBX = PEB->Ldr (PEB_LDR_DATA)
mov rbx, [rbx+0x20]       ; RBX = first module in InMemoryOrderModuleList

mov rbx, [rbx+0x20]       ; move to next module
mov rax, [rbx+0x10]       ; RAX = base address of kernel32.dll

for i in 0..NumberOfNames:
    lea rsi, [RAX + AddressOfNames[i]]   ; pointer to function name
    ; compare memory with "GetProcAddress"
    ; if match, load ordinal
    movzx r10, word ptr [RAX+AddressOfNameOrdinals[i]]
    mov r11, [RAX + AddressOfFunctions[r10]] ; RVA
    add r11, RAX                            ; absolute VA

   ; example: resolve WriteConsoleA
mov rcx, rax               ; kernel32 base
lea rdx, [rip+writeconsole_str] ; "WriteConsoleA"
call r11                    ; GetProcAddress(kernel32, "WriteConsoleA")
; returned pointer is ready in RAX 
         */

        // let mut encoder = Encoder::new(64);

        // let mem = MemoryOperand::new(
        //     Register::None,
        //     Register::None,
        //     1,
        //     0x60,
        //     8,
        //     false,
        //     Register::GS
        // );
        // let instr = Instruction::with_mem(Code::Mov_r64_m64, Register::RAX, mem).unwrap();
        // encoder.encode(&instr, 0).unwrap();

        // let instruction = Instruction::with1(Code::Call_rm64, Register::RAX).unwrap();
        // encoder.encode(&instruction, 0x0).unwrap();

        // let mut instr = Instruction::with(Code::Retnq);
        // encoder.encode(&instr, 0).unwrap();

        // let buffer = encoder.take_buffer();

        // println!("{:?}", buffer);

        let mut payload_src: [u8; 9] = [0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xC3];
        let payload_src_ptr = payload_src.as_mut_ptr() as *mut std::ffi::c_void;

        let module_name = CString::new("Kernel32.dll").unwrap_unchecked();
        let handle = GetModuleHandleA(module_name.as_ptr());

        let func_name = CString::new("VirtualAlloc").unwrap_unchecked();
        let virtual_alloc_ptr = GetProcAddress(handle, func_name.as_ptr());
        let virtual_alloc: VirtualAllocFn = std::mem::transmute(virtual_alloc_ptr);
        let payload_length = 0x100;
        let payload_dest = virtual_alloc(ptr::null_mut(), payload_length, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE); 

        let rtl_name = CString::new("RtlMoveMemory").unwrap();
        let rtl_ptr = GetProcAddress(handle, rtl_name.as_ptr());
        let rtl_move_memory: RtlMoveMemoryFn = std::mem::transmute(rtl_ptr);
        rtl_move_memory(payload_dest, payload_src_ptr, payload_src.len());

        let virtual_protect_name = CString::new("VirtualProtect").unwrap();
        let virtual_protect_ptr = GetProcAddress(handle, virtual_protect_name.as_ptr());
        let virtual_protect: VirtualProtectFn = std::mem::transmute(virtual_protect_ptr);
        let mut old_protect: u32 = 0;
        let result = virtual_protect(payload_dest, payload_length, PAGE_EXECUTE_READ, &mut old_protect);

        let create_thread_name = CString::new("CreateThread").unwrap();
        let create_thread_ptr = GetProcAddress(handle, create_thread_name.as_ptr());
        let create_thread: CreateThreadFn = std::mem::transmute(create_thread_ptr);
        let start_routine: ThreadFn = std::mem::transmute(payload_dest);
        let thread_handle = create_thread(
            std::ptr::null_mut(),           // lpThreadAttributes
            0,                              // dwStackSize
            start_routine, // lpStartAddress
            std::ptr::null_mut(),           // lpParameter
            0,                              // dwCreationFlags
            std::ptr::null_mut(),           // lpThreadId
        );

        // let thread_handle = CreateThread(
        //     std::ptr::null_mut(),
        // 0,
        // Some(start_routine),
        // std::ptr::null_mut(),
        // 0,
        // std::ptr::null_mut());
 
        let wso_name = CString::new("WaitForSingleObject").unwrap();
        let wso_ptr = GetProcAddress(handle, wso_name.as_ptr());
        let wso: WaitForSingleObjectFn = std::mem::transmute(wso_ptr);

        const INFINITE: u32 = 0xFFFFFFFF;
        wso(thread_handle, INFINITE);
    }
}