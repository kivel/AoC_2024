use core::panic;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// Lets do a bit of 3-bit assembler

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Adv, // division | The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    Bxl, // bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    Bst, // calculates the value of its combo operand modulo 8
    Jnz, // If Register A is Zero, do nothing, else set instruction pointer to literal value, do _not_ jump forward by 2!
    Bxc, // bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    Out, // calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Bdv, // works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    Cdv, // works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
}

impl Instruction {
    fn from_code(code: &usize) -> Instruction {
        match code {
            0b000 => Instruction::Adv,
            0b001 => Instruction::Bxl,
            0b010 => Instruction::Bst,
            0b011 => Instruction::Jnz,
            0b100 => Instruction::Bxc,
            0b101 => Instruction::Out,
            0b110 => Instruction::Bdv,
            0b111 => Instruction::Cdv,
            _ => panic!("Invalid instruction code: {}", code),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Register {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Computer {
    register: Register,
    program: Vec<usize>,
    pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn get_instruction(&self) -> Instruction {
        Instruction::from_code(&self.program[self.pointer])
    }

    fn get_operand(&self) -> usize {
        self.program[self.pointer + 1]
    }

    fn combo_operand(&self) -> usize {
        match self.get_operand() {
            operand @ 0..=3 => operand,
            4 => self.register.a,
            5 => self.register.b,
            6 => self.register.c,
            7 => panic!("b111 is an invalid operand."),
            operand if operand > 7 => {
                panic!("operand -> {operand} exceeds the 3-bit the computer can handle.")
            }
            _ => panic!("invalid operand."),
        }
    }

    fn division(&self) -> usize {
        let numerator = self.register.a;
        let base: usize = 2;
        let denominator = base.pow(self.combo_operand() as u32);
        numerator.saturating_div(denominator)
    }

    fn process(&mut self) {
        match self.get_instruction() {
            Instruction::Adv => {
                self.register.a = self.division();
                self.pointer += 2;
            }
            Instruction::Bxl => {
                self.register.b ^= self.get_operand();
                self.pointer += 2;
            }
            Instruction::Bst => {
                self.register.b = self.combo_operand() % 8;
                self.pointer += 2;
            }
            Instruction::Jnz => {
                if self.register.a == 0 {
                    self.pointer += 2;
                } else {
                    self.pointer = self.get_operand();
                }
            }
            Instruction::Bxc => {
                self.register.b ^= self.register.c;
                self.pointer += 2;
            }
            Instruction::Out => {
                self.output.push(self.combo_operand() % 8);
                self.pointer += 2;
            }
            Instruction::Bdv => {
                self.register.b = self.division();
                self.pointer += 2;
            }
            Instruction::Cdv => {
                self.register.c = self.division();
                self.pointer += 2;
            }
            _ => panic!("not implemented"),
        }
    }
}

// Implement the Iterator trait
impl Iterator for Computer {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer < self.program.len() {
            let result = (self.program[self.pointer], self.program[self.pointer + 1]);
            self.pointer += 2;
            Some(result) // Yield the current value
        } else {
            None // End of iteration
        }
    }
}

fn day17_1(computer: &mut Computer) -> String {
    while computer.pointer < computer.program.len() {
        computer.process();
        println!("Computer: {:?}", computer);
    }
    let result = computer
        .output
        .iter() // Create an iterator over the Vec
        .map(|n| n.to_string()) // Convert each number to a String
        .collect::<Vec<_>>() // Collect into a Vec of strings
        .join(",");
    println!("RESULT:  {:?}", result);
    println!("EXPECTED: 4,6,3,5,6,3,5,2,1,0");
    result
}

fn main() {
    let mut computer = Computer {
        register: Register { a: 729, b: 0, c: 0 },
        program: vec![0, 1, 5, 4, 3, 0],
        pointer: 0,
        output: Vec::new(),
    };
    let sum = day17_1(&mut computer);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::{day17_1, Computer, Register};

    #[test]
    fn test_combo_operand_ok() {
        let mut computer = Computer {
            register: Register {
                a: 10,
                b: 11,
                c: 12,
            },
            program: vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8],
            pointer: 0,
            output: Vec::new(),
        };
        computer.pointer = 0;
        assert_eq!(computer.combo_operand(), 0);
        computer.pointer = 2;
        assert_eq!(computer.combo_operand(), 1);
        computer.pointer = 4;
        assert_eq!(computer.combo_operand(), 2);
        computer.pointer = 6;
        assert_eq!(computer.combo_operand(), 3);
        computer.pointer = 8;
        assert_eq!(computer.combo_operand(), 10);
        computer.pointer = 10;
        assert_eq!(computer.combo_operand(), 11);
        computer.pointer = 12;
        assert_eq!(computer.combo_operand(), 12);
    }
    #[test]
    #[should_panic]
    fn test_combo_operand_panic() {
        let mut computer = Computer {
            register: Register {
                a: 10,
                b: 11,
                c: 12,
            },
            program: vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8],
            pointer: 0,
            output: Vec::new(),
        };
        computer.pointer = 14;
        assert_eq!(computer.combo_operand(), 0);
        computer.pointer = 16;
        assert_eq!(computer.combo_operand(), 0);
    }

    #[test]
    fn division_test() {
        let mut computer = Computer {
            register: Register {
                a: 1600,
                b: 40,
                c: 800,
            },
            program: vec![0, 1, 6, 2, 7, 3],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.register.a, 800);
        computer.process();
        assert_eq!(computer.register.b, 200);
        computer.process();
        assert_eq!(computer.register.c, 100);
    }

    #[test]
    fn bxl_test() {
        let mut computer = Computer {
            register: Register { a: 0, b: 5, c: 0 },
            program: vec![1, 3],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.register.b, 6);
    }

    #[test]
    fn bst_test() {
        let mut computer = Computer {
            register: Register { a: 0, b: 255, c: 0 },
            program: vec![2, 5],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.register.b, 7);
    }

    #[test]
    fn jnz_test() {
        let mut computer = Computer {
            register: Register { a: 0, b: 0, c: 0 },
            program: vec![3, 0, 3, 0],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.pointer, 2);
        computer.register.a = 1;
        computer.process();
        assert_eq!(computer.pointer, 0);
    }

    #[test]
    fn bxc_test() {
        let mut computer = Computer {
            register: Register { a: 0, b: 5, c: 3 },
            program: vec![4, 0],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.register.b, 6);
    }

    #[test]
    fn out_test() {
        let mut computer = Computer {
            register: Register { a: 0, b: 255, c: 0 },
            program: vec![5, 5],
            pointer: 0,
            output: Vec::new(),
        };
        computer.process();
        assert_eq!(computer.output.pop(), Some(7));
    }

    #[test]
    fn reg_c_prg_2_6_test() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut computer = Computer {
            register: Register { a: 0, b: 0, c: 9 },
            program: vec![2, 6],
            pointer: 0,
            output: Vec::new(),
        };
        day17_1(&mut computer);
        assert_eq!(computer.register.b, 1);
    }

    #[test]
    fn res_test() {
        let mut computer = Computer {
            register: Register { a: 729, b: 0, c: 0 },
            program: vec![0, 1, 5, 4, 3, 0],
            pointer: 0,
            output: Vec::new(),
        };
        let result = day17_1(&mut computer);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn res_final() {
        let mut computer = Computer {
            register: Register {
                a: 46323429,
                b: 0,
                c: 0,
            },
            program: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 0, 3, 5, 5, 3, 0],
            pointer: 0,
            output: Vec::new(),
        };
        let result = day17_1(&mut computer);
        assert_eq!(result, "7,6,1,5,3,1,4,2,6".to_string());
    }
}
