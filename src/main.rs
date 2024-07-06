#[derive(Debug)]
struct Node {
    value: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: char) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

fn binary_expression(expression: &String) -> Node {
    let chars = expression.chars().collect::<Vec<char>>();
    let mut node = Node::new('0');
    let mut left = Node::new('0');
    let mut right = Node::new('0');
    if chars[0] == '(' {
        let closing_index = find_closing_parenthesis(expression);
        node.value = chars[closing_index + 1];
        left = binary_expression(&expression[1..closing_index].to_string());
    }
    if chars[chars.len() - 1] == ')' {
        let opening_index = find_opening_parenthesis(expression);
        node.value = chars[opening_index - 1];
        right = binary_expression(&expression[opening_index + 1..expression.len() - 1].to_string());
    }
    if chars[0] != '(' && chars[chars.len() - 1] != ')' {
        node.value = chars[1];
    }
    if chars[0] != '(' {
        left = Node::new(chars[0]);
    }
    if chars[chars.len() - 1] != ')' {
        right = Node::new(chars[chars.len() - 1]);
    }
    node.left = Some(Box::new(left));
    node.right = Some(Box::new(right));
    node
}

fn find_closing_parenthesis(expression: &String) -> usize {
    let mut count: i32 = 0;
    let mut found: bool = false;
    let mut i = 0;
    let chars = expression.chars().collect::<Vec<char>>();
    while count > 0 || !found {
        if chars[i] == '(' {
            count += 1;
        } else if chars[i] == ')' {
            found = true;
            count -= 1;
        }
        if count == 0 && found {
            return i;
        }
        i += 1;
    }
    panic!("No closing parenthesis found");
}

fn find_opening_parenthesis(expression: &String) -> usize {
    let mut count: i32 = 0;
    let mut found: bool = false;
    let mut i = expression.len() - 1;
    let chars = expression.chars().collect::<Vec<char>>();
    while count > 0 || !found {
        if chars[i] == ')' {
            count += 1;
        } else if chars[i] == '(' {
            found = true;
            count -= 1;
        }
        if count == 0 && found {
            return i;
        }
        i -= 1;
    }
    panic!("No closing parenthesis found");
}

fn main() {
    println!("{:?}", binary_expression(&"(a+b)+(c+d)".to_string()));
}
