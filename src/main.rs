mod node;
use node::Node;
use core::fmt;

//MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplusb() {
        let expression = "a+b".to_string();
        let expected = Node {
            value: Some('+'),
            left: Some(Box::new(Node {
                value: Some('a'),
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: Some('b'),
                left: None,
                right: None,
            })),
        };
        assert_eq!(build_tree(&expression, &mut 0).0, expected);
    }

    #[test]
    fn aplusbplusc() {
        let expression = "(a+b)+c".to_string();
        let expected = Node {
            value: Some('+'),
            left: Some(Box::new(Node {
                value: Some('+'),
                left: Some(Box::new(Node {
                    value: Some('a'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: Some('b'),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Node {
                value: Some('c'),
                left: None,
                right: None,
            })),
        };
        assert_eq!(build_tree(&expression, &mut 0).0, expected);
    }

    #[test]
    fn sum_of_sum() {
        let expression = "(a+b)+(c+d)".to_string();
        let expected = Node {
            value: Some('+'),
            left: Some(Box::new(Node {
                value: Some('+'),
                left: Some(Box::new(Node {
                    value: Some('a'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: Some('b'),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Node {
                value: Some('+'),
                left: Some(Box::new(Node {
                    value: Some('c'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: Some('d'),
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(build_tree(&expression, &mut 0).0, expected);
    }
}

// MARK: Builder
fn build_tree(expression: &String, height: &mut i32) -> (Node, i32) {
    let chars = expression.chars().collect::<Vec<char>>();
    let mut node = Node::new(None);
    let mut left = Node::new(None);
    let mut right = Node::new(None);
    if chars[0] == '(' {
        let closing_index = find_closing_parenthesis(expression);
        node.value = Some(chars[closing_index + 1]);
        (left, *height) = build_tree(&expression[1..closing_index].to_string(), height);
    }
    if chars[chars.len() - 1] == ')' {
        let opening_index = find_opening_parenthesis(expression);
        node.value = Some(chars[opening_index - 1]);
        (right, *height) = build_tree(&expression[opening_index + 1..expression.len() - 1].to_string(), height);
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
    (node, *height + 1)
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
        if i == 0 {
            break;
        }
        i -= 1;
    }
    panic!("No closing parenthesis found");
}

// MARK: ExpressionTree
struct ExpressionTree {
    expression: String,
    root: Node,
    height: i32,
}

impl ExpressionTree {
    fn new(expression: String) -> Self {
        ExpressionTree {
            expression,
            root: Node::new(None),
            height: 1,
        }
    }

    fn build(&mut self) {
        self.root = build_tree(&self.expression, &mut self.height).0;
    }
}

impl fmt::Display for ExpressionTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", " ".repeat(2_usize.pow(self.height as u32)))?;
        write!(f, "{}\n", self.root.value.unwrap())?;

        let mut current_layer: Vec<&Box<Node>> = vec![&self.root.left.as_ref().unwrap(), &self.root.right.as_ref().unwrap()];
        let current_len = current_layer.len();
        let mut layer_count = 1;

        while !current_layer.is_empty() {

            let mut next_layer = Vec::new();
            let padding = 2_usize.pow(self.height as u32 - layer_count );
            write!(f, "{}", " ".repeat(padding))?;
            for node in current_layer {
                if let Some(value) = node.value {
                    write!(f, "{}{}", value," ".repeat(2*padding - 1))?;
                }
                if node.left.is_some() {
                    next_layer.push(node.left.as_ref().unwrap());
                }
                if node.right.is_some() {
                    next_layer.push(node.right.as_ref().unwrap());
                }
            }
            write!(f, "\n")?;
            for _ in 0..current_len {
                if next_layer.len()  == 0 {
                    break;
                }
            }
            write!(f, "\n")?;
            current_layer = next_layer;
            layer_count += 1;
        }
        Ok(())
    }
}

use clap::Parser;
#[derive(Parser)]
struct Cli {
    expression: String,
}

fn main() {
    let args = Cli::parse();
    let mut my_tree = ExpressionTree::new(args.expression);
    my_tree.build();
    println!("{}", my_tree);
}
