// 1.()   2. ^   3. * / %   4. + -
pub fn evaluate(expression: String) -> i64{
    let symbol_line = tokenize(expression);
    if !valid_parentheses(&symbol_line) || !pre_check(&symbol_line){
        return i64::MAX;
    }
    return 1;
}

fn tokenize(expression: String) -> Vec<Symbol>{
    return expression.split(' ').map(|x| Symbol::to_symbol(x.to_string())).collect();
}

enum Symbol{
    Plus,
    Minus,
    Multi,
    Div,
    Remainder,
    Exp,
    LParen,
    RParen,
    Number(i64),
    Invalid,
}

impl Symbol{
    pub fn to_symbol(str: String) -> Symbol{
        if str.trim().parse::<i64>().is_ok(){
            return Symbol::Number(str.trim().parse::<i64>().unwrap());
        }

        return match str.trim(){
            "+" => Symbol::Plus,
            "-" => Symbol::Minus,
            "*" => Symbol::Multi,
            "/" => Symbol::Div,
            "%" => Symbol::Remainder,
            "^" => Symbol::Exp,
            "(" => Symbol::RParen,
            ")" => Symbol::LParen,
            _ => Symbol::Invalid,
        }
    }
}

fn valid_parentheses(expression: &Vec<Symbol>) -> bool{
    return false;
}

fn pre_check(expression: &Vec<Symbol>) -> bool{
    return false;
}
