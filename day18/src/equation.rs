/// Parts of an equation.
enum EqPart {
    Num(u64),
    Op(char),
    Par(Vec<EqPart>),
}

/// A full equation.
pub struct Equation {
    root: Vec<EqPart>,
}

/// Evaluates a sequences of equation parts.
fn eval(eq_parts: &Vec<EqPart>) -> u64 {
    if eq_parts.len() == 0 {
        return 0;
    }
    let mut val = 0;
    let mut last_op = '+';
    for p in eq_parts.iter() {
        if let EqPart::Op(op) = p {
            last_op = *op;
            continue;
        }
        let num = match p {
            EqPart::Num(n) => *n,
            EqPart::Par(par) => eval(&par),
            _ => unreachable!(),
        };
        match last_op {
            '+' => val += num,
            '*' => val *= num,
            _ => unreachable!(),
        }
    }
    val
}

impl Equation {
    pub fn new(equation: String) -> Result<Self, String> {
        let mut eq_stack: Vec<Vec<EqPart>> = vec![Vec::new()];
        let mut top = &mut eq_stack[0];
        for (i, ch) in equation.chars().enumerate() {
            if ch.is_numeric() {
                let num = ch.to_digit(10).unwrap() as u64;
                top.push(EqPart::Num(num));
            } else if ch == '+' || ch == '*' {
                top.push(EqPart::Op(ch));
            } else if ch == '(' {
                eq_stack.push(Vec::new());
                let last_i = eq_stack.len() - 1;
                top = &mut eq_stack[last_i];
            } else if ch == ')' {
                let eq = eq_stack.pop().unwrap();
                let last_i = eq_stack.len() - 1;
                top = &mut eq_stack[last_i];
                top.push(EqPart::Par(eq));
            } else if !ch.is_whitespace() {
                return Err(format!("Unexpected character \"{}\" at position {}", ch, i));
            }
        }
        Ok(Self {
            root: eq_stack.pop().unwrap(),
        })
    }

    /// Evaluates the equation.
    pub fn eval(&self) -> u64 {
        eval(&self.root)
    }
}
