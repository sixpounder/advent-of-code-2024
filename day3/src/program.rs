static INSTRUCTION_PARSER_RE: &str = r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do(?:n\'t)?\(\)"#;

pub enum Instruction {
    Mul(i32, i32),
    Enable,
    Disable,
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new<S: AsRef<str>>(code: S) -> Self {
        let regex = regex::Regex::new(INSTRUCTION_PARSER_RE).unwrap();
        let mut instructions = vec![];
        for capture_match in regex.captures_iter(code.as_ref()) {
            let matched = capture_match.get(0).unwrap().as_str();
            if matched == "do()" {
                instructions.push(Instruction::Enable);
            } else if matched == "don't()" {
                instructions.push(Instruction::Disable);
            } else {
                let operand1: i32 = capture_match
                    .get(1)
                    .expect("Syntax error: expected operand")
                    .as_str()
                    .parse()
                    .expect("Syntax error: operand must be a number");
                let operand2: i32 = capture_match
                    .get(2)
                    .expect("Syntax error: expected operand")
                    .as_str()
                    .parse()
                    .expect("Syntax error: operand must be a number");
                instructions.push(Instruction::Mul(operand1, operand2));
            }
        }

        Self { instructions }
    }

    pub fn execute(&self) -> i32 {
        let mut rc = 0;
        let mut enable_math = true;
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mul(op1, op2) => {
                    if enable_math {
                        rc += op1 * op2;
                    }
                }
                Instruction::Enable => enable_math = true,
                Instruction::Disable => enable_math = false,
            };
        }

        rc
    }
}
