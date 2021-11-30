use std::io::stdin;

fn main() {
    let mut bf = Bf::new(String::from("++++.[-]>++<.>.>,."));
    let debug = false;
    
    let mut i = 0;
    while i < bf.input.len() {
        i = tick(&mut bf, i);
        if debug {
            bf.display();   
        }
    }

}

fn tick(bf: &mut Bf, mut i: usize) -> usize {
    match bf.input.chars().nth(i).unwrap() {
        '<' => {
            bf.bk();
        },
        '>' => {
            bf.fw();
        }
        '+' => {
            bf.inc();
        },
        '-' => {
            bf.dec();
        },
        '[' => {
            if bf.cur() == 0 {
                loop {
                    i += 1;
                    if bf.input.chars().nth(i).unwrap() == ']' {
                        return i+1;
                    }          
                }
            }
        },
        ']' => {
            if bf.cur() != 0 {
                loop {
                    i -= 1;
                    if bf.input.chars().nth(i).unwrap() == '[' {
                        return i+1;
                    }          
                }
            }
        },
        '.'=> {
            println!("{}", bf.cur());
        },
        ',' => {
            let mut data = String::new();
            println!("Input: ");
            stdin().read_line(&mut data).expect("expected input");
            bf.strip.data[bf.strip.cursor] = data.trim().parse::<i32>().unwrap();
        },
        _ => return i+1,
    }
    return i+1;
}
struct Strip {
    data: Vec<i32>,
    cursor: usize
}

impl Strip {
    pub fn new() -> Strip {
        let mut s = Strip {
            data: Vec::new(),
            cursor: 0
        };
        
        s.data.push(0);
        
        return s;
    }

    fn fw(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.data.len() {
            self.data.push(0);
        }
    }
    
    fn bk(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    
    fn cur(&self) -> i32 {
        return self.data[self.cursor];
    }
    
    fn inc(&mut self) {
        self.data[self.cursor] += 1;
    }
    
    fn dec(&mut self) {
        self.data[self.cursor] -= 1;
    }
}

struct Bf {
    strip: Strip,
    input: String
}

impl Bf {
    pub fn new(inst: String) -> Bf {
        let mut b = Bf {
            strip: Strip::new(),
            input: inst
        };
        
        return b;
    }
    fn fw(&mut self) {
        self.strip.fw();
    }
    
    fn bk(&mut self) {
        self.strip.bk();
    }
    
    fn cur(&self) -> i32 {
        return self.strip.cur();
    }
    
    fn inc(&mut self) {
        self.strip.inc();
    }
    
    fn dec(&mut self) {
        self.strip.dec();
    }
    
    fn display(&self) {
        for i in 0..self.strip.data.len() {
            if i == self.strip.cursor {
                print!("({})", self.strip.data[i]);
            } else {
                print!("[{}]", self.strip.data[i]);
            }
        }
        println!();
    }
}