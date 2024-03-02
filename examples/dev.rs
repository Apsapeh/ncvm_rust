use ncvm::{self, Instruction, ThreadSettings};
use ncvm::opcode as op;

fn main() {
    // Инструкции виртуальной машины
    let mut instr  = vec![
        Instruction::new_i(op::ISMLD, 0, 0, 1),  // Записываем в регистр 0 значение из статической памяти из ячейки 0 
        Instruction::new_i(op::ISMLD, 1, 1, 1),  // Записываем в регистр 1 значение из статической памяти из ячейки 1 
        Instruction::new_i(op::ISR,   2, 0, 12), // Записываем в регистр 2 число 12 
        Instruction::new_i(op::IADD,  0, 0, 1),  // Записываем в регистр 0 сумму регистров 0 и 1
        Instruction::new_i(op::IADD,  0, 0, 2),  // Записываем в регистр 0 сумму регистров 0 и 2
        Instruction::new_i(op::ISR,   1, 0, 3),
        Instruction::new_i(op::IMULT, 0, 0, 1),
        Instruction::new_i(op::ISMST, 2, 0, 1),  // Записываем в ячейку 2 статической памяти значение из регистра 0 
        Instruction::new_i(op::ISR,   1, 0, 5),
        Instruction::new_i(op::IMOD,  0, 0, 1),
        Instruction::new_i(op::ISMST, 3, 0, 1),  // Записываем в ячейку 2 статической памяти значение из регистра 0 
        Instruction::new_i(op::STOP,  0, 0, 0),  // Завершаем выполнение
    ];

    // Статическая память
    let mut s_mem: Vec<u8> = vec![
    //   16    21     0     0
        0x10, 0x15, 0x00, 0x00
    ];

    let mut vm = ncvm::VM::new(&mut instr, &mut s_mem);
    //vm.execute(ncvm::ThreadSettings::default());
    vm.create_thread(1, &mut vec![], ThreadSettings::default());

    

    println!("{:?}", s_mem);
}