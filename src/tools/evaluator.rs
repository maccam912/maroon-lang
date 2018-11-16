use tools::logging::*;
use tools::io_tools::*;
use tools::string_tools::*;
use tools::preprocessor::*;
use std::process;

pub struct Evaluator {
    preserved_program: String,
    data: Vec<String>,
    logging: bool
}

impl Evaluator {
    pub fn new(program: &str, actual_line: &str) -> Self {
        Self {
            preserved_program: actual_line.to_string(),
            data: split(program),
            logging: false,
            // logging: true
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let result: Option<String>;

        if self.data.len() > 0 {
            result = Some(self.data[0].clone());
            self.data = self.data[1..].to_vec();
        } else {
            result = None;
        }

        return result;
    }

    pub fn pop(&mut self) -> Option<String> {
        let result: Option<String>;

        if self.data.len() > 0 {
            result = self.data.pop();
        } else {
            result = None;
        }

        return result;
    }

    // pub fn pop_front(&mut self) -> Option<String> {
    //     let result: Option<String>;

    //     if self.data.len() > 0 {
    //         result = Some(self.data[0].clone());
    //         self.data = self.data[1..].to_vec();
    //     } else {
    //         result = None;
    //     }

    //     return result;
    // }

    pub fn push(&mut self, n: Vec<String>) {
        // let mut data = n;
        if n.len() > 0 {

            // let data: Vec<String> = n
            //     .into_iter()
            //     .map(
            //         |x|
            //             x.replace("\\_", " ")
            //     )
            //     .collect();

            self.data.extend(n);
        }
    }

    pub fn push_front(&mut self, n: Vec<String>) {
        if n.len() > 0 {
            // self.data.extend(n);

            // let n: Vec<String> = n
            //     .into_iter()
            //     .map(
            //         |x|
            //             x.replace("\\_", " ")
            //     )
            //     .collect();

            let mut val = n.clone();
            val.append(&mut self.data);
            self.data = val;
        }
    }

    pub fn end(&mut self) -> bool {
        let mut is_end = true;
        for s in &self.data {
            if ["!".to_string(), 
                "@eq".to_string(),
                "@eval".to_string(),
                "@input".to_string(),
                "@print".to_string(),
                "@print*".to_string(),
                "@println".to_string(),
                "@println*".to_string()].contains(s) {
                is_end = false;
            }
        }

        return is_end;
    }

    pub fn safe_pop(&mut self) -> String {
        match self.pop() {
            Some(n) => n,
            None => {
                error(
                    format!(
                        "Attempted to call function with too few parameters: \n\n\"{}\"",
                        self.preserved_program
                        )
                    );
                process::exit(1);
            }
        }
    }


    pub fn step(&mut self) {
        let instruction = self.next();

        match instruction {
            Some(n) => {
                if n == "!".to_string() {
                    let argument = &self.safe_pop();
                    let function = &self.safe_pop();
                    self.push_front(
                        split(&call(
                                function,
                                argument
                            ))
                        );
                } else if n == "@input".to_string() {
                    self.push(vec![stdin()]);

                } else if n == "@eq".to_string() {
                    let arg1 = &self.safe_pop();
                    let arg2 = &self.safe_pop();

                    let result = match arg1 == arg2 {
                        true => "a.b.a",
                        false => "a.b.b"
                    };
                    // println!("arg1: {}, arg2: {}, result: {}", arg1, arg2, result);
                    self.push(vec![result.to_string()]);


                } else if n == "@eval".to_string() {
                    let mut preprocessor = Preprocessor::new();
                    let arg = &self.safe_pop();
                    self.push(vec![Evaluator::new(&mut preprocessor.process(arg), arg).eval().join("")]);

                } else if n == "@print".to_string() {
                    print!("{}",
                        &(self.safe_pop()
                            .replace("\\\\", "\\")
                            .replace("\\x", "!")
                            .replace("\\rp", ")")
                            .replace("\\lp", "(")
                            .replace("\\rb", "]")
                            .replace("\\lb", "[")
                            .replace("\\_", " ")
                        )
                    );
                } else if n == "@print*".to_string() {
                    print!("{}", self.data.clone().join("")
                            .replace("\\\\", "\\")
                            .replace("\\x", "!")
                            .replace("\\rp", ")")
                            .replace("\\lp", "(")
                            .replace("\\rb", "]")
                            .replace("\\lb", "[")
                            .replace("\\_", " ")
                    );
                } else if n == "@println".to_string() {
                    println!("{}",
                        &(
                            self.safe_pop()
                                .replace("\\\\", "\\")
                                .replace("\\x", "!")
                                .replace("\\rp", ")")
                                .replace("\\lp", "(")
                                .replace("\\rb", "]")
                                .replace("\\lb", "[")
                                .replace("\\_", " ")
                        )
                    );
                } else if n == "@println*".to_string() {
                    println!("{}",
                        self.data.clone().join("")
                            .replace("\\\\", "\\")
                            .replace("\\x", "!")
                            .replace("\\rp", ")")
                            .replace("\\lp", "(")
                            .replace("\\rb", "]")
                            .replace("\\lb", "[")
                            .replace("\\_", " ")
                    );
                    
                } else {
                    self.push(split(&n));
                }
            }
            None => {
                process::exit(1);            
            }
        };

        if self.logging {
            debug(format!(
                "{:?}", self.data
            ));
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        while self.data.len() > 0 && !self.end() {
            self.step();
        }
        return self.data.clone();
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("{:?}", self.data);
    }
}
