#[derive(Debug)]
struct Node {
    value: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: Option<char>) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

fn expression_tree(expression: &String) -> Node {
    let chars = expression.chars().collect::<Vec<char>>();
    let mut node = Node::new(None);
    let mut left = Node::new(None);
    let mut right = Node::new(None);
    if chars[0] == '(' {
        let closing_index = find_closing_parenthesis(expression);
        node.value = Some(chars[closing_index + 1]);
        left = expression_tree(&expression[1..closing_index].to_string());
    }
    if chars[chars.len() - 1] == ')' {
        let opening_index = find_opening_parenthesis(expression);
        node.value = Some(chars[opening_index - 1]);
        right = expression_tree(&expression[opening_index + 1..expression.len() - 1].to_string());
    }
    if chars[0] != '(' && chars[chars.len() - 1] != ')' {
        node.value = Some(chars[1]);
    }
    if chars[0] != '(' {
        left = Node::new(Some(chars[0]));
    }
    if chars[chars.len() - 1] != ')' {
        right = Node::new(Some(chars[chars.len() - 1]));
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
    println!("{:?}", expression_tree(&"(a+b)+(c+d)".to_string()));
}
