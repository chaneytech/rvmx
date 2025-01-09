#[cfg(test)]
mod tests {
    use crate::vm::Vm;

    #[test]
    fn test_create_vm() {
        let test_vm = Vm::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = Vm::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = Vm::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = Vm::new();
        test_vm.program = vec![2, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = Vm::new();
        test_vm.program = vec![2, 0, 0, 1, 2, 1, 0, 2, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 3);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = Vm::new();
        test_vm.program = vec![2, 0, 0, 3, 2, 1, 0, 2, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = Vm::new();
        test_vm.program = vec![2, 0, 0, 3, 2, 1, 0, 2, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 6);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = Vm::new();
        test_vm.program = vec![2, 0, 0, 3, 2, 1, 0, 2, 6, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![9, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 7;
        test_vm.registers[1] = 8;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 7;
        test_vm.registers[1] = 8;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gte_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 8;
        test_vm.registers[1] = 8;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 7;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 8;
        test_vm.registers[1] = 8;
        test_vm.program = vec![15, 0, 1, 0, 15, 0, 1, 0, 15, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 7;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = Vm::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![16, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
