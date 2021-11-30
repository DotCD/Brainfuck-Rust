fn main() {
    //Create new instance of "Bf" with instructions/program
    let mut bf = Bf::new(String::from("++++.[-]>++<.>.>,."));
    let debug = false;
    
    let mut i = 0;
    //Loop through instructions until complete
    while i < bf.input.len() {
        i = tick(&mut bf, i);
        //prints the strip every step if enabled
        if debug {
            bf.display();   
        }
    }

}

///Takes instruction index and Bf instance. Parses and performs instruction then returns next index
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
        '[' => { //if loop should be broken, break
            if bf.cur() == 0 {
                loop {
                    i += 1;
                    if bf.input.chars().nth(i).unwrap() == ']' {
                        return i+1;
                    }          
                }
            }
        },
        ']' => { //if loop should continue, return to starting position
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
            std::io::stdin().read_line(&mut data).expect("expected input");
            bf.strip.data[bf.strip.cursor] = data.trim().parse::<i32>().unwrap();
        },
        _ => return i+1,
    }
    return i+1;
}

///Data Strip Struct.
///This is the what the brainfuck commands modify
struct Strip {
    data: Vec<i32>,
    cursor: usize
}

impl Strip {
    ///Creates new strip with Data[0] set to 0
    pub fn new() -> Strip {
        let mut s = Strip {
            data: Vec::new(),
            cursor: 0
        };
        
        s.data.push(0);
        
        return s;
    }

    ///Moves cursor forward one
    fn fw(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.data.len() {
            self.data.push(0);
        }
    }
    
    ///Moves cursor back one with floor of 0
    fn bk(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    
    ///Returns data at cursor
    fn cur(&self) -> i32 {
        return self.data[self.cursor];
    }
    
    ///Increments cell at cursor by 1
    fn inc(&mut self) {
        self.data[self.cursor] += 1;
    }
    
    ///Decrements cell at cursor by 1
    fn dec(&mut self) {
        self.data[self.cursor] -= 1;
    }
}

///Contains the data Strip and the input program
struct Bf {
    strip: Strip,
    input: String
}

impl Bf {
    ///Creates new Bf instance with passed input
    pub fn new(inst: String) -> Bf {
        let b = Bf {
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
    
    ///Loops through data strip and prints the strip. Highlights data cell at cursor location
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