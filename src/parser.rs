// 1.()   2. ^   3. * / %   4. + -
pub fn evaluate(expression: String) -> i64{
    let mut symbol_line = tokenize(expression);
    if !valid_parentheses(&symbol_line) || !pre_check(&mut symbol_line){
        return i64::MAX; 
    }

    return calculate(&mut symbol_line);
}

pub fn calculate(symbol_line: &mut Vec<Symbol>) -> i64{
    while symbol_line.contains(&Symbol::LParen){
        //replaces range between parentheses with a calculated number
        let l_paren_index = symbol_line.iter().position(|x| x == &Symbol::LParen).unwrap();
        let r_paren_index = find_matching_paren(symbol_line, l_paren_index);

        let number = calculate(&mut (symbol_line[l_paren_index..r_paren_index]).to_vec());

        symbol_line.drain(l_paren_index..r_paren_index);
        symbol_line.insert(l_paren_index, Symbol::Number(number));
    }


    return 1;
}

#[allow(unused_assignments)]
fn tokenize(expression: String) -> Vec<Symbol>{
    //return expression.split(' ').map(|x| Symbol::to_symbol(x.to_string())).collect();
    let mut result: Vec<Symbol> = vec![];
    let mut number: String = "".to_string();
    let mut tokenizing_number: bool = false;

    for symbol in expression.chars().filter(|c| !c.is_whitespace()){
        if !symbol.is_numeric() && tokenizing_number{
            tokenizing_number = false;
            result.push(Symbol::to_symbol(number));
            number = "".to_string();
        }

        if symbol.is_numeric(){
            number.push(symbol);
            tokenizing_number = true;
        } else {
            result.push(Symbol::to_symbol(symbol.to_string()));
            tokenizing_number = false;
        }
    }

    return result;
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum Symbol{
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

//finds the index of a matching parenthesis
fn find_matching_paren(symbols: &Vec<Symbol>, lparen_index: usize) -> usize{
    let (mut to_skip, mut i) = (0, lparen_index + 1);

    for symbol in symbols[lparen_index+1..].iter(){
        if symbol == &Symbol::LParen{
            to_skip += 1;
        } else if symbol == &Symbol::RParen && to_skip != 0{
            to_skip -= 1;
        } else if symbol == &Symbol::RParen && to_skip == 0{
            return i;
        }

        i += 1;
    }
    return 0;
}

fn colapse_sequential_operands(symbol_line: &mut Vec<Symbol>){
    let (mut plus_count, mut minus_count, index) = (0, 0, 0);
    for symbol in symbol_line{
        match symbol{ 
            &mut Symbol::Plus => plus_count += 1,
            &mut Symbol::Minus => minus_count += 1,
            _ => (plus_count, minus_count) = (0, 0),
        }
        
        if plus_count > 0 && minus_count > 0{
            if minus_count % 2 == 0{

            }
        }
    }
}

fn collapse() -> Symbol{
    return Symbol::Number(0);
}
