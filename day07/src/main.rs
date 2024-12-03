#![allow(dead_code)]

#[derive(Debug, Clone)]
enum Value {
    Variable(String),
    Value(u16),
}

#[derive(Debug, Clone)]
enum Variable {
    VALUE { v: Value },
    AND { v1: Value, v2: Value },
    OR { v1: Value, v2: Value },
    NOT { v: Value },
    LSHIFT { v1: Value, v2: Value },
    RSHIFT { v1: Value, v2: Value },
}

impl Variable {
    fn is_value(&self) -> bool {
        match self {
            Variable::VALUE { v: _ } => true,
            _ => false,
        }
    }
}

fn puzzle1(input: &str, search: &str) -> u16 {
    let mut variables = Vec::new();
    for line in input.split('\n') {
        let a = line.split_whitespace().collect::<Vec<&str>>();
        if a.is_empty() {
            continue;
        }
        let variable = match a.len() {
            3 => {
                let v = if a[0].chars().all(char::is_numeric) {
                    Value::Value(a[0].parse::<u16>().unwrap())
                } else {
                    Value::Variable(a[0].to_string())
                };
                Variable::VALUE { v }
            }
            4 => {
                let v = if a[1].chars().all(char::is_numeric) {
                    Value::Value(a[1].parse::<u16>().unwrap())
                } else {
                    Value::Variable(a[1].to_string())
                };
                Variable::NOT { v }
            }
            5 => match a[1] {
                "OR" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::OR { v1, v2 }
                }
                "AND" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::AND { v1, v2 }
                }
                "RSHIFT" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::RSHIFT { v1, v2 }
                }
                "LSHIFT" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::LSHIFT { v1, v2 }
                }
                _ => panic!("Variable type Unknown"),
            },
            _ => {
                panic!("Incorect number of elements in line")
            }
        };
        variables.push((a[a.len() - 1].to_string(), variable));
    }

    variables.sort_by(|a, b| a.0.cmp(&b.0));

    let mut search_index = None;
    for i in 0..variables.len() {
        if variables[i].0 == search {
            search_index = Some(i);
            break;
        }
    }

    let search_index = search_index.expect(&format!("Parameter {search} not found"));

    eval(&mut variables, search_index, 0)
}

fn eval(variables: &mut Vec<(String, Variable)>, index: usize, indent: u16) -> u16 {
    let id = variables[index].0.clone();
    let op = variables[index].1.clone();
    for _ in 0..indent {
        print!(" ");
    }
    println!("{id} {:?}", op);
    let value = match op {
        Variable::VALUE { v } => match v {
            Value::Variable(var) => eval(variables, find(variables, &var), indent + 1),
            Value::Value(v) => v,
        },
        Variable::NOT { v } => match v {
            Value::Value(v) => !v,
            Value::Variable(v) => !eval(variables, find(variables, &v), indent + 1),
        },
        Variable::RSHIFT { v1, v2 } => {
            let v1 = match v1 {
                Value::Value(v1) => v1,
                Value::Variable(v1) => eval(variables, find(variables, &v1), indent + 1),
            };
            let v2 = match v2 {
                Value::Value(v2) => v2,
                Value::Variable(v2) => eval(variables, find(variables, &v2), indent + 1),
            };
            let (v, _) = v1.overflowing_shr(v2 as u32);
            v
        }
        Variable::LSHIFT { v1, v2 } => {
            let v1 = match v1 {
                Value::Value(v1) => v1,
                Value::Variable(v1) => eval(variables, find(variables, &v1), indent + 1),
            };
            let v2 = match v2 {
                Value::Value(v2) => v2,
                Value::Variable(v2) => eval(variables, find(variables, &v2), indent + 1),
            };
            let (v, _) = v1.overflowing_shl(v2 as u32);
            v
        }
        Variable::OR { v1, v2 } => {
            let v1 = match v1 {
                Value::Value(v1) => v1,
                Value::Variable(v1) => eval(variables, find(variables, &v1), indent + 1),
            };
            let v2 = match v2 {
                Value::Value(v2) => v2,
                Value::Variable(v2) => eval(variables, find(variables, &v2), indent + 1),
            };
            v1 | v2
        }
        Variable::AND { v1, v2 } => {
            let v1 = match v1 {
                Value::Value(v1) => v1,
                Value::Variable(v1) => eval(variables, find(variables, &v1), indent + 1),
            };
            let v2 = match v2 {
                Value::Value(v2) => v2,
                Value::Variable(v2) => eval(variables, find(variables, &v2), indent + 1),
            };
            v1 & v2
        }
    };

    for i in 0..variables.len() {
        match variables[i].1.clone() {
            Variable::VALUE { v: _ } => {
                if variables[i].0 == id {
                    variables[i].1 = Variable::VALUE {
                        v: Value::Value(value),
                    };
                }
            }
            Variable::RSHIFT { v1, v2 } => {
                match v1.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::RSHIFT {
                                v1: Value::Value(value),
                                v2: v2.clone(),
                            }
                        }
                    }
                    _ => {}
                }
                match v2.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::RSHIFT {
                                v1: v1.clone(),
                                v2: Value::Value(value),
                            }
                        }
                    }
                    _ => {}
                }
            }
            Variable::LSHIFT { v1, v2 } => {
                match v1.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::LSHIFT {
                                v1: Value::Value(value),
                                v2: v2.clone(),
                            }
                        }
                    }
                    _ => {}
                }
                match v2.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::LSHIFT {
                                v1: v1.clone(),
                                v2: Value::Value(value),
                            }
                        }
                    }
                    _ => {}
                }
            }
            Variable::OR { v1, v2 } => {
                match v1.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::OR {
                                v1: Value::Value(value),
                                v2: v2.clone(),
                            }
                        }
                    }
                    _ => {}
                }
                match v2.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::OR {
                                v1: v1.clone(),
                                v2: Value::Value(value),
                            }
                        }
                    }
                    _ => {}
                }
            }
            Variable::AND { v1, v2 } => {
                match v1.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::AND {
                                v1: Value::Value(value),
                                v2: v2.clone(),
                            }
                        }
                    }
                    _ => {}
                }
                match v2.clone() {
                    Value::Variable(var) => {
                        if *var == id {
                            variables[i].1 = Variable::AND {
                                v1: v1.clone(),
                                v2: Value::Value(value),
                            }
                        }
                    }
                    _ => {}
                }
            }
            Variable::NOT { v } => match v {
                Value::Variable(var) => {
                    if *var == id {
                        variables[i].1 = Variable::NOT {
                            v: Value::Value(value),
                        }
                    }
                }
                _ => {}
            },
        }
    }

    for _ in 0..indent {
        print!(" ");
    }
    println!("{id} {:?} = {value}", &variables[index].1);
    value
}

fn find(variables: &Vec<(String, Variable)>, id: &str) -> usize {
    for i in 0..variables.len() {
        if variables[i].0 == id {
            return i;
        }
    }
    panic!("{}", &format!("Couldn't find variable of id {id}"))
}

fn puzzle2(input: &str, search: &str, wire_b: u16) -> u16 {
    let mut variables = Vec::new();
    for line in input.split('\n') {
        let a = line.split_whitespace().collect::<Vec<&str>>();
        if a.is_empty() {
            continue;
        }
        let variable = match a.len() {
            3 => {
                let v = if a[0].chars().all(char::is_numeric) {
                    Value::Value(a[0].parse::<u16>().unwrap())
                } else {
                    Value::Variable(a[0].to_string())
                };
                Variable::VALUE { v }
            }
            4 => {
                let v = if a[1].chars().all(char::is_numeric) {
                    Value::Value(a[1].parse::<u16>().unwrap())
                } else {
                    Value::Variable(a[1].to_string())
                };
                Variable::NOT { v }
            }
            5 => match a[1] {
                "OR" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::OR { v1, v2 }
                }
                "AND" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::AND { v1, v2 }
                }
                "RSHIFT" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::RSHIFT { v1, v2 }
                }
                "LSHIFT" => {
                    let v1 = if a[0].chars().all(char::is_numeric) {
                        Value::Value(a[0].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[0].to_string())
                    };
                    let v2 = if a[2].chars().all(char::is_numeric) {
                        Value::Value(a[2].parse::<u16>().unwrap())
                    } else {
                        Value::Variable(a[2].to_string())
                    };
                    Variable::LSHIFT { v1, v2 }
                }
                _ => panic!("Variable type Unknown"),
            },
            _ => {
                panic!("Incorect number of elements in line")
            }
        };
        variables.push((a[a.len() - 1].to_string(), variable));
    }

    let index = find(&variables, "b");
    variables[index] = (
        "b".to_string(),
        Variable::VALUE {
            v: Value::Value(wire_b),
        },
    );

    variables.sort_by(|a, b| a.0.cmp(&b.0));

    let mut search_index = None;
    for i in 0..variables.len() {
        if variables[i].0 == search {
            search_index = Some(i);
            break;
        }
    }

    let search_index = search_index.expect(&format!("Parameter {search} not found"));

    eval(&mut variables, search_index, 0)
}

fn main() {
    let input = std::fs::read_to_string("day07/input.txt").unwrap();

    let p1 = puzzle1(&input, "a");
    println!("P1: {p1}");
    let p2 = puzzle2(&input, "a", p1);
    println!("P2: {p2}");
}
