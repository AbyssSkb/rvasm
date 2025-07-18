use std::{collections::HashMap, env, fs, num::ParseIntError};

fn reg_name_to_num(reg_name: &str) -> Result<u32, String> {
    match reg_name {
        "x0" | "zero" => Ok(0),
        "x1" | "ra" => Ok(1),
        "x2" | "sp" => Ok(2),
        "x3" | "gp" => Ok(3),
        "x4" | "tp" => Ok(4),
        "x5" | "t0" => Ok(5),
        "x6" | "t1" => Ok(6),
        "x7" | "t2" => Ok(7),
        "x8" | "s0" | "fp" => Ok(8),
        "x9" | "s1" => Ok(9),
        "x10" | "a0" => Ok(10),
        "x11" | "a1" => Ok(11),
        "x12" | "a2" => Ok(12),
        "x13" | "a3" => Ok(13),
        "x14" | "a4" => Ok(14),
        "x15" | "a5" => Ok(15),
        "x16" | "a6" => Ok(16),
        "x17" | "a7" => Ok(17),
        "x18" | "s2" => Ok(18),
        "x19" | "s3" => Ok(19),
        "x20" | "s4" => Ok(20),
        "x21" | "s5" => Ok(21),
        "x22" | "s6" => Ok(22),
        "x23" | "s7" => Ok(23),
        "x24" | "s8" => Ok(24),
        "x25" | "s9" => Ok(25),
        "x26" | "s10" => Ok(26),
        "x27" | "s11" => Ok(27),
        "x28" | "t3" => Ok(28),
        "x29" | "t4" => Ok(29),
        "x30" | "t5" => Ok(30),
        "x31" | "t6" => Ok(31),
        _ => Err(format!("Unknown register name: {reg_name}")),
    }
}

fn encode_r_type(funct7: &str, rs2: &str, rs1: &str, funct3: &str, rd: &str, opcode: &str) -> u32 {
    let funct7_num = u32::from_str_radix(funct7, 2).unwrap();
    let rs2_num = reg_name_to_num(rs2).unwrap();
    let rs1_num = reg_name_to_num(rs1).unwrap();
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let rd_num = reg_name_to_num(rd).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (funct7_num << 25)
        | (rs2_num << 20)
        | (rs1_num << 15)
        | (funct3_num << 12)
        | (rd_num << 7)
        | opcode_num
}

fn encode_i_type(imm: i32, rs1: &str, funct3: &str, rd: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFF).unwrap();
    let rs1_num = reg_name_to_num(rs1).unwrap();
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let rd_num = reg_name_to_num(rd).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num << 20) | (rs1_num << 15) | (funct3_num << 12) | (rd_num << 7) | opcode_num
}

fn encode_s_type(imm: i32, rs2: &str, rs1: &str, funct3: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFF).unwrap();
    let rs2_num = reg_name_to_num(rs2).unwrap();
    let rs1_num = reg_name_to_num(rs1).unwrap();
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num >> 5) << 25
        | (rs2_num << 20)
        | (rs1_num << 15)
        | (funct3_num << 12)
        | ((imm_num & 0x1F) << 7)
        | opcode_num
}

fn encode_b_type(imm: i32, rs2: &str, rs1: &str, funct3: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0x1FFE).unwrap();
    let rs2_num = reg_name_to_num(rs2).unwrap();
    let rs1_num = reg_name_to_num(rs1).unwrap();
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num >> 12) << 31
        | ((imm_num >> 5) & 0x3F) << 25
        | (rs2_num << 20)
        | (rs1_num << 15)
        | (funct3_num << 12)
        | ((imm_num >> 1) & 0xF) << 8
        | ((imm_num >> 11) & 0x1) << 7
        | opcode_num
}

fn encode_u_type(imm: i32, rd: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFFFF).unwrap();
    let rd_num = reg_name_to_num(rd).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num << 12) | (rd_num << 7) | opcode_num
}

fn encode_j_type(imm: i32, rd: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0x1FFFFE).unwrap();
    let rd_num = reg_name_to_num(rd).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num >> 20) << 31
        | ((imm_num >> 1) & 0x3FF) << 21
        | ((imm_num >> 11) & 0x1) << 20
        | ((imm_num >> 12) & 0xFF) << 12
        | (rd_num << 7)
        | opcode_num
}

fn encode_instruction(
    opcode: &str,
    rd: &str,
    rs1: &str,
    rs2: &str,
    imm: i32,
) -> Result<u32, String> {
    match opcode {
        "add" => Ok(encode_r_type("0000000", rs2, rs1, "000", rd, "0110011")),
        "sub" => Ok(encode_r_type("0100000", rs2, rs1, "000", rd, "0110011")),
        "and" => Ok(encode_r_type("0000000", rs2, rs1, "111", rd, "0110011")),
        "or" => Ok(encode_r_type("0000000", rs2, rs1, "110", rd, "0110011")),
        "xor" => Ok(encode_r_type("0000000", rs2, rs1, "100", rd, "0110011")),
        "sll" => Ok(encode_r_type("0000000", rs2, rs1, "001", rd, "0110011")),
        "srl" => Ok(encode_r_type("0000000", rs2, rs1, "101", rd, "0110011")),
        "sra" => Ok(encode_r_type("0100000", rs2, rs1, "101", rd, "0110011")),
        "slt" => Ok(encode_r_type("0000000", rs2, rs1, "010", rd, "0110011")),
        "sltu" => Ok(encode_r_type("0000000", rs2, rs1, "011", rd, "0110011")),
        "addi" => Ok(encode_i_type(imm, rs1, "000", rd, "0010011")),
        "andi" => Ok(encode_i_type(imm, rs1, "111", rd, "0010011")),
        "ori" => Ok(encode_i_type(imm, rs1, "110", rd, "0010011")),
        "xori" => Ok(encode_i_type(imm, rs1, "100", rd, "0010011")),
        "slli" => Ok(encode_i_type(imm & 0x1F, rs1, "001", rd, "0010011")),
        "srli" => Ok(encode_i_type(imm & 0x1F, rs1, "101", rd, "0010011")),
        "srai" => Ok(encode_i_type(imm & 0x1F | 0x400, rs1, "101", rd, "0010011")),
        "slti" => Ok(encode_i_type(imm, rs1, "010", rd, "0010011")),
        "sltiu" => Ok(encode_i_type(imm, rs1, "011", rd, "0010011")),
        "lb" => Ok(encode_i_type(imm, rs1, "000", rd, "0000011")),
        "lbu" => Ok(encode_i_type(imm, rs1, "100", rd, "0000011")),
        "lh" => Ok(encode_i_type(imm, rs1, "001", rd, "0000011")),
        "lhu" => Ok(encode_i_type(imm, rs1, "101", rd, "0000011")),
        "lw" => Ok(encode_i_type(imm, rs1, "010", rd, "0000011")),
        "jalr" => Ok(encode_i_type(imm, rs1, "000", rd, "1100111")),
        "sb" => Ok(encode_s_type(imm, rs2, rs1, "000", "0100011")),
        "sh" => Ok(encode_s_type(imm, rs2, rs1, "001", "0100011")),
        "sw" => Ok(encode_s_type(imm, rs2, rs1, "010", "0100011")),
        "beq" => Ok(encode_b_type(imm, rs2, rs1, "000", "1100011")),
        "bne" => Ok(encode_b_type(imm, rs2, rs1, "001", "1100011")),
        "blt" => Ok(encode_b_type(imm, rs2, rs1, "100", "1100011")),
        "bltu" => Ok(encode_b_type(imm, rs2, rs1, "110", "1100011")),
        "bge" => Ok(encode_b_type(imm, rs2, rs1, "101", "1100011")),
        "bgeu" => Ok(encode_b_type(imm, rs2, rs1, "111", "1100011")),
        "lui" => Ok(encode_u_type(imm, rd, "0110111")),
        "auipc" => Ok(encode_u_type(imm, rd, "0010111")),
        "jal" => Ok(encode_j_type(imm, rd, "1101111")),
        _ => Err(format!("Unknown opcode: {opcode}")),
    }
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    if let Some(hex) = s.strip_prefix("0x") {
        i32::from_str_radix(hex, 16)
    } else if let Some(oct) = s.strip_prefix("0o") {
        i32::from_str_radix(oct, 8)
    } else if let Some(bin) = s.strip_prefix("0b") {
        i32::from_str_radix(bin, 2)
    } else {
        s.parse()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Please specify the input file")
    }
    let file_path = &args[1];
    let output_coe = args.len() == 3 && &args[2] == "--coe";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut labels = HashMap::new();
    let mut address = 0;
    let mut binary_instructions = Vec::new();

    for line in contents.lines() {
        let cleaned_line = line.split('#').next().unwrap().trim();
        if cleaned_line.is_empty() {
            continue;
        }
        if cleaned_line.ends_with(':') {
            let label = cleaned_line.split(':').next().unwrap().trim();
            labels.insert(label, address);
            continue;
        }
        address += 4;
    }

    address = 0;
    for line in contents.lines() {
        let cleaned_line = line.split('#').next().unwrap().trim();
        if cleaned_line.is_empty() || cleaned_line.ends_with(':') {
            continue;
        }

        let (opcode, rest) = cleaned_line.split_once(' ').unwrap();
        let lowercase_opcode = opcode.to_lowercase();
        let opcode = lowercase_opcode.as_str();
        let mut split_rest = rest.split(',').map(|s| s.trim());
        let len = rest.split(',').count();

        let binary_inst;
        if len == 3 {
            let first_item = split_rest.next().unwrap();
            let second_item = split_rest.next().unwrap();
            let third_item = split_rest.next().unwrap();

            match opcode {
                "add" | "sub" | "and" | "or" | "xor" | "sll" | "srl" | "sra" | "slt" | "sltu" => {
                    binary_inst =
                        encode_instruction(opcode, first_item, second_item, third_item, -1);
                }
                "addi" | "andi" | "ori" | "xori" | "slli" | "srli" | "srai" | "slti" | "sltiu"
                | "jalr" => {
                    let imm = parse_number(third_item).unwrap();
                    binary_inst = encode_instruction(opcode, first_item, second_item, "x0", imm);
                }
                "beq" | "bne" | "blt" | "bltu" | "bge" | "bgeu" => {
                    let imm = labels.get(third_item).unwrap() - address;
                    binary_inst = encode_instruction(opcode, "x0", first_item, second_item, imm);
                }
                _ => {
                    panic!("Unsupported Instruction: {line}")
                }
            }
        } else if len == 2 {
            let first_item = split_rest.next().unwrap();
            let second_item = split_rest.next().unwrap();

            match opcode {
                "li" => {
                    let imm: i32 = parse_number(second_item).unwrap();
                    binary_inst = encode_instruction("addi", first_item, "x0", "x0", imm);
                }
                "mv" => {
                    binary_inst = encode_instruction("add", first_item, second_item, "x0", -1);
                }
                "not" => {
                    binary_inst = encode_instruction("xori", first_item, second_item, "x0", -1);
                }
                "neg" => {
                    binary_inst = encode_instruction("sub", first_item, "x0", second_item, -1);
                }
                "jal" => {
                    let imm = labels.get(second_item).unwrap() - address;
                    binary_inst = encode_instruction(opcode, first_item, "x0", "x0", imm);
                }
                "lui" | "auipc" => {
                    let imm = parse_number(second_item).unwrap();
                    binary_inst = encode_instruction(opcode, first_item, "x0", "x0", imm);
                }
                "lb" | "lbu" | "lh" | "lhu" | "lw" | "jalr" => {
                    let (imm, rs1) = second_item.split_once('(').unwrap();
                    let imm = parse_number(imm).unwrap();
                    let rs1 = rs1.split(')').next().unwrap();
                    binary_inst = encode_instruction(opcode, first_item, rs1, "x0", imm);
                }
                "sb" | "sh" | "sw" => {
                    let (imm, rs1) = second_item.split_once('(').unwrap();
                    let imm = parse_number(imm).unwrap();
                    let rs1 = rs1.split(')').next().unwrap();
                    binary_inst = encode_instruction(opcode, "x0", rs1, first_item, imm);
                }
                _ => {
                    panic!("Unsupported Instruction: {line}")
                }
            }
        } else if len == 1 {
            let first_item = split_rest.next().unwrap();

            match opcode {
                "jal" => {
                    let imm = labels.get(first_item).unwrap() - address;
                    binary_inst = encode_instruction(opcode, "ra", "x0", "x0", imm);
                }
                "j" => {
                    let imm = labels.get(first_item).unwrap() - address;
                    binary_inst = encode_instruction(opcode, "x0", "x0", "x0", imm);
                }
                _ => {
                    panic!("Unsupported Instruction: {line}")
                }
            }
        } else {
            panic!("Unsupported Instruction: {line}")
        }
        let binary_instruction = format!("{:08x}", binary_inst.unwrap());
        binary_instructions.push(binary_instruction);
        address += 4;
    }

    if output_coe {
        println!("memory_initialization_radix = 16;");
        println!("memory_initialization_vector =");
        let len = binary_instructions.len();
        for binary_instruction in &binary_instructions {
            if *binary_instruction == binary_instructions[len - 1] {
                println!("{binary_instruction};")
            } else {
                println!("{binary_instruction},");
            }
        }
    } else {
        for binary_instruction in binary_instructions {
            println!("{binary_instruction}");
        }
    }
}
