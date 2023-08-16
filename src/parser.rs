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

        let number = calculate(&mut (symbol_line[l_paren_index + 1..=r_paren_index - 1]).to_vec());

        symbol_line.drain(l_paren_index..=r_paren_index);
        symbol_line.insert(l_paren_index, Symbol::Number(number));
    }

    colapse_sequential_operands(symbol_line); 

    let mut index = 0;
    // ^
    for symbol in symbol_line.clone(){
        if symbol == Symbol::Exp{
            let new_number = collapse(symbol_line[index - 1].clone(), symbol, symbol_line[index + 1].clone());
            symbol_line.drain(index - 1..=index + 1);
            symbol_line.insert(index - 1, new_number);
            index -= 1;
            continue;
        }

        index += 1;
    } 
    index = 0;
    // * / %
    for symbol in symbol_line.clone(){
        if symbol == Symbol::Multi || symbol == Symbol::Div || symbol == Symbol::Remainder{
            let new_number = collapse(symbol_line[index - 1].clone(), symbol, symbol_line[index + 1].clone());
            symbol_line.drain(index - 1..=index + 1);
            symbol_line.insert(index - 1, new_number);
            index -= 1;
            continue;
        }

        index += 1;
    }
    index = 0;
    // + -
    for symbol in symbol_line.clone(){
        if symbol == Symbol::Plus || symbol == Symbol::Minus{
            let new_number = collapse(symbol_line[index - 1].clone(), symbol, symbol_line[index + 1].clone());
            symbol_line.drain(index - 1..=index + 1);
            symbol_line.insert(index - 1, new_number);
            index -= 1;
            continue;
        }

        index += 1;
    }

     match symbol_line.get(0).unwrap(){
        Symbol::Number(x) => return *x,
        _ => panic!("Invalid calculation"),
    };
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

    if !number.is_empty(){
        result.push(Symbol::to_symbol(number));
    }

    return result;
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
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
            "(" => Symbol::LParen,
            ")" => Symbol::RParen,
            _ => Symbol::Invalid,
        }
    }
}

fn valid_parentheses(expression: &Vec<Symbol>) -> bool{
    let mut paren_count: i16 = 0;
    let mut next_bad: bool = false;

    if !expression.contains(&Symbol::RParen) && !expression.contains(&Symbol::LParen){
        return true;
    }

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
        _ => {},
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
    let mut previous = Symbol::Invalid;
    let mut index = 0;
    for symbol in symbol_line.clone(){
        if (symbol == Symbol::Plus || symbol == Symbol::Minus) && (previous == Symbol::Plus || previous == Symbol::Minus){
            if (symbol == Symbol::Minus || previous == Symbol::Minus) && symbol != previous{
                symbol_line.drain(index - 1..=index);
                symbol_line.insert(index - 1, Symbol::Minus);
                previous = Symbol::Minus;
            } else {
                symbol_line.drain(index - 1..=index);
                symbol_line.insert(index - 1, Symbol::Plus);
                previous = Symbol::Plus
            }
            continue;
        }

        index += 1;
        previous = symbol.clone();
    }
}

fn collapse(first_number: Symbol, operand: Symbol, second_number: Symbol) -> Symbol{
    let first_number_integer: i64;
    let second_number_integer: i64;

    match(first_number, second_number){
        (Symbol::Number(x), Symbol::Number(y)) => {
            first_number_integer = x;
            second_number_integer = y;
        },
        _ => return Symbol::Invalid,
    }

    return Symbol::Number(match operand{
        Symbol::Plus => first_number_integer + second_number_integer,
        Symbol::Minus => first_number_integer - second_number_integer,
        Symbol::Multi => first_number_integer * second_number_integer,
        Symbol::Div => first_number_integer / second_number_integer,
        Symbol::Remainder => first_number_integer % second_number_integer,
        Symbol::Exp => first_number_integer.pow(second_number_integer as u32),
        _ => 0,
    });
}
