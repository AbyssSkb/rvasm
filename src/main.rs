use std::{collections::HashMap, env, fmt::format, fs};

fn reg_name_to_num(reg_name: &str) -> Result<u8, String> {
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
        _ => Err(format!("Unknown register name: {}", reg_name)),
    }
}

fn encode_r_type(funct7: &str, rs2: u8, rs1: u8, funct3: &str, rd: u8, opcode: &str) -> u32 {
    let funct7_num = u32::from_str_radix(funct7, 2).unwrap();
    let rs2_num = u32::from(rs2);
    let rs1_num = u32::from(rs1);
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let rd_num = u32::from(rd);
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (funct7_num << 25)
        | (rs2_num << 20)
        | (rs1_num << 15)
        | (funct3_num << 12)
        | (rd_num << 7)
        | opcode_num
}

fn encode_i_type(imm: i32, rs1: u8, funct3: &str, rd: u8, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFF).unwrap();
    let rs1_num = u32::from(rs1);
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let rd_num = u32::from(rd);
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num << 20) | (rs1_num << 15) | (funct3_num << 12) | (rd_num << 7) | opcode_num
}

fn encode_s_type(imm: i32, rs2: u8, rs1: u8, funct3: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFF).unwrap();
    let rs2_num = u32::from(rs2);
    let rs1_num = u32::from(rs1);
    let funct3_num = u32::from_str_radix(funct3, 2).unwrap();
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num >> 5) << 25
        | (rs2_num << 20)
        | (rs1_num << 15)
        | (funct3_num << 12)
        | ((imm_num & 0x1F) << 7)
        | opcode_num
}

fn encode_b_type(imm: i32, rs2: u8, rs1: u8, funct3: &str, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0x1FFF).unwrap();
    let rs2_num = u32::from(rs2);
    let rs1_num = u32::from(rs1);
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

fn encode_u_type(imm: i32, rd: u8, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0xFFFFF).unwrap();
    let rd_num = u32::from(rd);
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num << 12) | (rd_num << 7) | opcode_num
}

fn encode_j_type(imm: i32, rd: u8, opcode: &str) -> u32 {
    let imm_num = u32::try_from(imm & 0x1FFFFF).unwrap();
    let rd_num = u32::from(rd);
    let opcode_num = u32::from_str_radix(opcode, 2).unwrap();

    (imm_num >> 20) << 31
        | ((imm_num >> 1) & 0x3FF) << 21
        | ((imm_num >> 11) & 0x1) << 20
        | ((imm_num >> 12) & 0xFF) << 12
        | (rd_num << 7)
        | opcode_num
}

fn encode_instruction(opcode: &str, rd: u8, rs1: u8, rs2: u8, imm: i32) -> Result<u32, String> {
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
        "slli" => Ok(encode_i_type(imm, rs1, "001", rd, "0010011")),
        "srli" => Ok(encode_i_type(imm, rs1, "101", rd, "0010011")),
        "srai" => Ok(encode_i_type(imm, rs1, "101", rd, "0010011")),
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
        _ => Err(format!("Unknown opcode: {}", opcode)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut labels = HashMap::new();
    let mut address = 0;
    for line in contents.lines() {
        let cleaned_line = line.split('#').next().unwrap().trim();
        if cleaned_line.is_empty() {
            continue;
        }
        if cleaned_line.ends_with(':') {
            let label = cleaned_line.split(':').next().unwrap().trim();
            labels.insert(label, address);
        } else {
            let (opcode, rest) = cleaned_line.split_once(' ').unwrap();
            match opcode {
                "add" => {
                    let mut split_results = rest.split(',');
                    let rd = split_results.next().unwrap();
                    let rs1 = split_results.next().unwrap();
                    let rs2 = split_results.next().unwrap();
                    println!()
                },
                _ => todo!()
            }
        }
        address += 4;
    }
}
