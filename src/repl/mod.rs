use crate::vm::Vm;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: Vm,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: Vm::new(),
            command_buffer: vec![],
        }
    }
    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }

                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                println!("{}", byte);
                                self.vm.add_byte(byte)
                            }
                        }
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    };
                    self.vm.run_once();
                }
            }
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let u8 = u8::from_str_radix(hex_string, 16)?;
            results.push(u8);
        }
        Ok(results)
    }
}
