#[derive(Debug, Clone)]
enum Ast {
    BinOp(String, Box<Ast>, Box<Ast>),
    UnOp(String, i32),
}

use Ast::*;
use std::collections::HashMap;

struct Compiler {
    op: HashMap<String, usize>,
    step:usize,
}

impl Compiler {
    fn new() -> Compiler {
        let mut op = HashMap::new();
        op.insert("+".to_string(), 1);
        op.insert("-".to_string(), 1);
        op.insert("*".to_string(), 2);
        op.insert("/".to_string(), 2);
        Compiler { op: op,step:0 }
    }

    fn tokenize<'a>(&self, program: &'a str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];

        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a' ... 'z' | 'A' ... 'Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    }
                    '0' ... '9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    }
                    ' ' => { iter.next(); }
                    _ => {
                        tokens.push(iter.next().unwrap().to_string());
                    }
                },
                None => break
            }
        }

        tokens
    }

    fn compile(&mut self, program: &str) -> Vec<String> {
        self.step=1;
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn pass1(&mut self, program: &str) -> Ast {
        self.step=2;
        let tokens = self.tokenize(program);
        println!("token:{:?}", tokens);
        let mut iter = tokens.iter().peekable();
        let mut args = HashMap::new();

        let mut arg_idx = 0;
        while let Some(s) = iter.next() {
            println!("{}",s);
            if s.as_str()=="]"{
                break;
            } else {
                match s.as_str() {
                    "[" => arg_idx = 0,
                    c  => {
                        println!("args:{}", c);
                        args.insert(c.to_string(), arg_idx);
                        arg_idx += 1;
                    }
                }
            }
        }

        let mut tokens=tokens[(arg_idx+2)..].to_vec();
        tokens.insert(0, "(".to_string());
        tokens.push(")".to_string());

        let mut ops = Vec::new();
        let mut ds = Vec::new();

        let mut iter=tokens.iter().rev().peekable();
          println!("iter:{:?}",iter);
        while let Some(s) =  iter.next() {
            match s.as_str() {
                c if self.op.contains_key(&c.to_string()) => {
                    println!("op=0>:{}", c);
                    println!("op=0>ops:{:?}", ops);
                    println!("op=0>ds:{:?}", ds);
                   loop {
                       if ops.is_empty() || ops.last().unwrap() == ")"
                           || self.op.get(&c.to_string()) >= self.op.get(ops.last().unwrap()) {
                           ops.push(c.to_string());
                           break;
                       } else {
                           let a = ds.pop().unwrap();
                           let b = ds.pop().unwrap();
                           ds.push(BinOp(ops.pop().unwrap(), Box::new(a), Box::new(b)));

                       }
                   }

                    println!("op=1>ops:{:?}", ops);
                    println!("op=1>ds:{:?}", ds);
                }
                "(" => {
                    println!("(=0>: oprator=>{:?}", ops);

                    println!("(=0>ds:{:?}", ds);
                    while let Some(p) = ops.pop() {
                        if p == ")".to_string() {
                            break;
                        } else {
                            let a = ds.pop().unwrap();
                            let b = ds.pop().unwrap();
                            ds.push(BinOp(p, Box::new(a), Box::new(b)));
                        }
                    }

                    println!("(=1>ds:{:?}", ds);
                }
                c if c == ")" => {
                    ops.push(c.to_string());

                    println!("(=>: oprator=>{:?}", ops);
                }
                c if args.contains_key(&c.to_string()) => {
                    if let Some(&id) = args.get(&c.to_string()) {
                        ds.push(UnOp("arg".to_string(), id as i32));
                    }
                    println!("exp arg:{}", c);
                }
                c => {
                    println!("exp other:{}", c);
                    ds.push(UnOp("imm".to_string(), c.parse::<i32>().unwrap()));
                    println!("exp other:{}", c);
                }
            }
        }
        println!("datastack=>{:?}", ds);
        ds[0].clone()
    }

    fn pass1_1(&mut self, program: &str) -> Ast {
        self.step=2;
        let mut tokens = self.tokenize(program);

        tokens.insert(0, "(".to_string());
        tokens.push(")".to_string());
        println!("{:?}", tokens);
        let mut iter = tokens.iter().peekable();
        let mut args = HashMap::new();
        let mut ops = Vec::new();
        let mut ds = Vec::new();
        let mut arg_idx = -1;

        while let Some(s) = iter.next() {
            match s.as_str() {
                "[" => arg_idx = 0,
                "]" => arg_idx = -1,
                c if arg_idx >= 0 => {
                    println!("args:{}", c);
                    args.insert(c.to_string(), arg_idx);
                    arg_idx += 1;
                }
                c if self.op.contains_key(&c.to_string()) => {
                    println!("op=0>:{}", c);
                    println!("op=0>ops:{:?}", ops);
                    println!("op=0>ds:{:?}", ds);
                    if ops.is_empty() || ops.last().unwrap() == "("
                        || self.op.get(&c.to_string()) >= self.op.get(ops.last().unwrap()) {
                        ops.push(c.to_string());
                    } else {
                        let a = ds.pop().unwrap();
                        let b = ds.pop().unwrap();
                        ds.push(BinOp(ops.pop().unwrap(), Box::new(b), Box::new(a)));
                        ops.push(c.to_string());
                    }

                    println!("op=1>ops:{:?}", ops);
                    println!("op=1>ds:{:?}", ds);
                }
                c if c == "(" => {
                    ops.push(c.to_string());

                    println!("(=>: oprator=>{:?}", ops);
                }
                ")" => {
                    println!(")=0>: oprator=>{:?}", ops);

                    println!(")=0>ds:{:?}", ds);
                    while let Some(p) = ops.pop() {
                        if p == "(".to_string() {
                            break;
                        } else {
                            let a = ds.pop().unwrap();
                            let b = ds.pop().unwrap();
                            ds.push(BinOp(p, Box::new(b), Box::new(a)));
                        }
                    }

                    println!(")=1>ds:{:?}", ds);
                }
                c if args.contains_key(&c.to_string()) => {
                    if let Some(&id) = args.get(&c.to_string()) {
                        ds.push(UnOp("arg".to_string(), id as i32));
                    }
                    println!("exp arg:{}", c);
                }
                c => {
                    ds.push(UnOp("imm".to_string(), c.parse::<i32>().unwrap()));
                    println!("exp other:{}", c);
                }
            }
        }
        println!("datastack=>{:?}", ds);
        ds[0].clone()
    }

    fn pass2(&mut self, ast: &Ast) -> Ast {
        self.step=3;
        match ast {
            &BinOp(ref op, ref a, ref b) => {
                let a = self.pass2(&*a);
                let b = self.pass2(&*b);

                match (&a, &b) {
                    (&UnOp(ref opa, na), &UnOp(ref opb, nb)) if opa.as_str() == "imm" && opb.as_str() == "imm" => {
                        UnOp("imm".to_string(), match op.as_str() {
                            "+" => na + nb,
                            "-" => na - nb,
                            "*" => na * nb,
                            "/" => na / nb,
                            _ => { 0 }
                        })
                    }
                    _ => {
                        BinOp(op.clone(), Box::new(a.clone()), Box::new(b.clone()))
                    }
                }
            }
            &UnOp(ref op, n) => {
                UnOp(op.clone(), n)
            }
        }
    }

    fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        self.step=4;
        let mut asm = Vec::new();
        match ast {
            &UnOp(ref op, n) => {
                if op == "imm" {
                    asm.push(format!("IM {}", n));
                } else if op == "arg" {
                    asm.push(format!("AR {}", n));
                }
            }
            &BinOp(ref op, ref a, ref b) => {
                asm.append(&mut self.pass3(&*a.clone()));

                asm.push("PU".to_string());
                asm.append(&mut self.pass3(&*b.clone()));
                asm.push("SW".to_string());
                asm.push("PO".to_string());
                asm.push(match op.as_str() {
                    "+" => "AD".to_string(),
                    "-" => "SU".to_string(),
                    "*" => "MU".to_string(),
                    "/" => "DI".to_string(),
                    _ => "".to_string()
                });
            }
        }

        asm
    }
}

fn simulate(assembly : Vec<&str>, argv : Vec<i32>) -> i32 {
    let mut r = (0, 0);
    let mut stack : Vec<i32> = vec![];

    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
            Some("AR") => r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize],
            Some("SW") => r = (r.1,r.0),
            Some("PU") => stack.push(r.0),
            Some("PO") => r.0 = stack.pop().unwrap(),
            Some("AD") => r.0 += r.1,
            Some("SU") => r.0 -= r.1,
            Some("MU") => r.0 *= r.1,
            Some("DI") => r.0 /= r.1,
            _ => panic!("Invalid instruction encountered"),
        }
    }

    r.0
}

fn main() {
    let s = "[ x y z] (2*3*x+5*y-3*z)/(1+3+2*2)";
    //let s="[x] x+2*5";
    let argv=vec![3i32,1,2];
    let mut c = Compiler::new();
    // println!("{:?}", c.tokenize("[ xx y ] 1 + 4*( xx + y ) / 2"));
    //let a=UnOP("arg".to_string(),0);
    let ast = c.pass1(&s);
    println!("{:?}", ast);
    let ast = c.pass2(&ast);
    println!("{:?}", ast);
    let r=c.pass3(&ast);
    println!("r:{:?}",r);
    //println!("{}", simulate(r.iter().map(|a|a.as_str()).collect(),argv));
}
