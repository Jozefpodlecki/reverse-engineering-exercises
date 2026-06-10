mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_mov_reg_memory() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov rax, [rbx]").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_r64_rm64);  // mov rax, [rbx] is mov r64, rm64
    }

    #[test]
    fn test_mov_memory_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov [rax], rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_rm64_r64);  // mov [rax], rbx is mov rm64, r64
    }

    #[test]
    fn test_mov_reg_memory_with_displacement() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov rax, [rbx+0x10]").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_r64_rm64);
    }

    #[test]
    fn test_mov_memory_reg_with_displacement() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov [rax+0x20], rcx").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_rm64_r64);
    }

    #[test]
    fn test_mov_reg_memory_rsp() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov rax, [rsp]").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_r64_rm64);
    }

    #[test]
    fn test_mov_memory_reg_rsp() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov [rsp], rax").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_rm64_r64);
    }

    #[test]
    fn test_mov_reg_memory_with_rbp() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mov rcx, [rbp-0x8]").unwrap();
        assert_ice(&result, iced_x86::Code::Mov_r64_rm64);
    }

    #[test]
    fn test_movzx_byte() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movzx rax, byte [rbx]").unwrap();
        assert_ice(&result, iced_x86::Code::Movzx_r64_rm8);
    }

    #[test]
    fn test_movsx_byte() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movsx rax, byte [rbx]").unwrap();
        assert_ice(&result, iced_x86::Code::Movsx_r64_rm8);
    }

    #[test]
    fn test_movzx_word() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movzx rax, word [rbx]").unwrap();
        assert_ice(&result, iced_x86::Code::Movzx_r64_rm16);
    }

    #[test]
    fn test_movsx_word() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movsx rax, word [rbx]").unwrap();
        assert_ice(&result, iced_x86::Code::Movsx_r64_rm16);
    }
}