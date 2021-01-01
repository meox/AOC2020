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
struct InvalidInstr(String);

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
            Op::INVALID(line) => return Err(InvalidInstr(line)),
            _ => ops.push(opcode),
        }
    }

    return Ok(ops);
}

fn execute(ops: Vec<Op>, mut ctx: Ctx) -> (Ctx, bool) {
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        if ctx.ip >= ops.len() {
            break
        }

        let opcode = ops[ctx.ip].clone();
        if visited.get(&ctx.ip).is_some() {
            return (ctx, false);
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

    (ctx, true)
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
    //println!("compiled = {:?}", bin);

    let r = execute(bin.clone(), Ctx{ acc: 0, ip: 0 });
    println!("VM: {:?}", r);

    // second part
    // 1. change all jmp in nop (step by step)
    let len = bin.len();
    for i in 0..len {
        let mut copy_bin = bin.clone();
        if let Op::JMP(v) = copy_bin[i] {
            copy_bin[i] = Op::NOP(v);
            // try to execute
            let (r, terminated) = execute(copy_bin, Ctx{ acc: 0, ip: 0 });
            if terminated {
                println!("Patched: JMP -> NOP, IP:{} {:?}", i, r);
                return
            }
        }
    }

    // 2. change all nop in jump (step by step)
    for i in 0..len {
        let mut copy_bin = bin.clone();
        if let Op::NOP(v) = copy_bin[i] {
            copy_bin[i] = Op::JMP(v);
            // try to execute
            let (r, terminated) = execute(copy_bin, Ctx{ acc: 0, ip: 0 });
            if terminated {
                println!("Patched: NOP -> JMP, IP:{} {:?}", i, r);
                return
            }
        }
    }
}
