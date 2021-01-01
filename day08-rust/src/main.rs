use std::collections::HashSet;
use std::convert::TryFrom;

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

fn compile(istrs: Vec<&str>) -> Result<Vec<Op>, InvalidInstr> {
    let mut ops: Vec<Op> = Vec::new();

    for istr in istrs {
        let opcode = parse_instruction(istr);
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

fn main() {
    // println!("result = {:?}", parse_instruction("nop +0"));
    // println!("result = {:?}", parse_instruction("acc +5"));
    // println!("result = {:?}", parse_instruction("jmp -5"));
    // println!("result = {:?}", parse_instruction("xxx +5"));
    let instrs: Vec<&str> = vec!["nop +0", "acc +5", "jmp -2"];

    let bin = compile(instrs).unwrap();
    println!("compiled = {:?}", bin);

    let ctx: Ctx = Ctx { acc: 0, ip: 0 };
    let r = execute(bin, ctx);
    println!("VM: {:?}", r);
}
