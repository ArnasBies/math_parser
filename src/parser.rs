// 1.()   2. ^   3. * / %   4. + -
pub fn evaluate(expression: String) -> i64{
    let mut symbol_line = tokenize(expression);
    if !valid_parentheses(&symbol_line) || !pre_check(&mut symbol_line){
        return i64::MAX; 
    }

    return 1;
}

fn tokenize(expression: String) -> Vec<Symbol>{
    return expression.split(' ').map(|x| Symbol::to_symbol(x.to_string())).collect();
}

#[allow(dead_code)]
#[derive(PartialEq)]
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
    let mut paren_count: i16 = 0;
    let mut next_bad: bool = false;

    for symbol in expression{
        //keeps track of current open parentheses also checks if there are no empty parentheses
        match symbol{
            &Symbol::LParen => {
                paren_count += 1;
                next_bad = true;
            },
            &Symbol::RParen => {
                paren_count -= 1;
                if next_bad{
                    return false;
                }
            },
            _ => next_bad = false,
        } 

        if paren_count < 0{
            return false;
        }
    }

    if paren_count != 0 {
        return false;
    }

    return true;
}

fn pre_check(expression: &mut Vec<Symbol>) -> bool{
    if expression.is_empty() {
        return false;
    }

    match expression.get(0).unwrap(){
        Symbol::LParen => {},
        Symbol::Minus => {
            expression.remove(0);
            if let Symbol::Number(x) = expression.get_mut(1).unwrap() {
                *x *= -1;
            } 
        },
        Symbol::Plus => {
            expression.remove(0);
        },
        _ => {
            return false;
        },
    }

    return check_pair(vec![Symbol::Multi, Symbol::Div, Symbol::Remainder, Symbol::Exp, Symbol::Plus, Symbol::Minus],
                  vec![Symbol::Multi, Symbol::Div, Symbol::Remainder, Symbol::Exp, Symbol::RParen],
                  expression);
}

//checks if there are two symbols next to each other from both symbol lists
fn check_pair(first_list: Vec<Symbol>, second_list: Vec<Symbol>, expression: &Vec<Symbol>) -> bool{
    let mut next_bad = false;
    for symbol in expression{
        if next_bad && second_list.contains(symbol){
            return false;
        }
        if first_list.contains(symbol){
            next_bad = true;
        } else{
            next_bad = false;
        }
    }

    return true;
}
