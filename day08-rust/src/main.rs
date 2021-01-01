use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
enum Op {
    ACC(i32),
    JMP(i32),
    NOP(i32),
    INVALID(String),
}

#[derive(Debug)]
struct Ctx {
    acc: i32,
    ip: usize,
}

#[derive(Debug, Clone)]
struct InvalidInstr {
    msg: String,
}

fn parse_instruction(s: &str) -> Op {
    // parse the line
    // ex: acc +1
    //     nop +0
    let line = String::from(s);
    let tks: Vec<&str> = line.split_ascii_whitespace().collect();
    if tks.len() != 2 {
        return Op::INVALID(String::from(s));
    }

    // convert argument to i32
    let n = tks[1].parse::<i32>().unwrap();

    match tks[0] {
        "acc" => Op::ACC(n),
        "jmp" => Op::JMP(n),
        "nop" => Op::NOP(n),
        _ => Op::INVALID(String::from(s)),
    }
}

fn compile(istrs: Vec<String>) -> Result<Vec<Op>, InvalidInstr> {
    let mut ops: Vec<Op> = Vec::new();

    for istr in istrs {
        let opcode = parse_instruction(&istr);
        match opcode {
            Op::INVALID(line) => return Err(InvalidInstr { msg: line }),
            _ => ops.push(opcode),
        }
    }

    return Ok(ops);
}

fn execute(ops: Vec<Op>, mut ctx: Ctx) -> Ctx {
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        let opcode = ops[ctx.ip].clone();
        if visited.get(&ctx.ip).is_some() {
            break;
        }

        visited.insert(ctx.ip);

        match opcode {
            Op::ACC(v) => {
                ctx.acc += v;
                ctx.ip += 1
            }
            Op::JMP(v) => {
                if v >= 0 {
                    ctx.ip += usize::try_from(v).unwrap()
                } else {
                    ctx.ip -= usize::try_from(-v).unwrap()
                }
            }
            _ => ctx.ip += 1,
        }
    }

    return ctx;
}


fn load_fromfile(fname: &str) -> Vec<String> {
    let mut instrs: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(instr) = line {
                instrs.push(instr);
            }
        }
    }

    instrs
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // println!("result = {:?}", parse_instruction("nop +0"));
    // println!("result = {:?}", parse_instruction("acc +5"));
    // println!("result = {:?}", parse_instruction("jmp -5"));
    // println!("result = {:?}", parse_instruction("xxx +5"));
    // let instrs: Vec<&str> = vec![
    //     "nop +0",
    //     "acc +1",
    //     "jmp +4",
    //     "acc +3",
    //     "jmp -3",
    //     "acc -99",
    //     "acc +1",
    //     "jmp -4",
    //     "acc +6",
    // ];

    let instrs = load_fromfile("data/input.txt");
    let bin = compile(instrs).unwrap();
    println!("compiled = {:?}", bin);

    let ctx: Ctx = Ctx { acc: 0, ip: 0 };
    let r = execute(bin, ctx);
    println!("VM: {:?}", r);
}
