use std::rc::Rc;

#[derive(Debug)]
struct Node {
    weight: usize,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

// impl Node {
//     fn new(left: Box<Rope>, right: Box<Rope>, weight: usize) > Node{
// 	Node{
// 	    left: Some(left),
// 	    right: Some(right),
// 	    weight: weight,
// 	}
//     }
// }

#[derive(Debug)]
struct Leaf {
    buf: Rc<String>,
    start: usize,
    end: usize,
}

impl Leaf {
    fn new(s: &str) -> Leaf {
        let leaf = Leaf {
            buf: Rc::new(s.clone().to_string()),
            start: 0,
            end: s.len() - 1,
        };
        leaf
    }

    fn weight(&self) -> usize {
        return self.end - self.start + 1;
    }

    fn split(&self, offset: usize) -> (Leaf, Leaf) {
        if offset == 0 {
            return (
		Leaf::new(""),
		Leaf::new(&self.buf.as_ref().clone()),
            );
	}
	
	if offset >= self.weight() {
            return (
		Leaf::new(&self.buf.as_ref().clone()),
		Leaf::new(""),
            );
        }
	
	let (left, right) = self.buf.split_at(self.start + offset);
        ((Leaf::new(left)), Leaf::new(right))
    }
}

#[derive(Debug)]
enum Rope {
    Node(Node),
    Leaf(Leaf),
}

impl Rope {
    fn new(s: &str) -> Rope {
        Rope::Leaf(Leaf::new(s))
    }

    fn index(&self, i: usize) -> Option<char> {
        match self {
            Rope::Leaf(leaf) => return leaf.buf.chars().nth(i),
            Rope::Node(node) => {
                if i <= node.weight {
                    return node.left.as_ref()?.index(i);
                }
                node.right.as_ref()?.index(i - node.weight)
            }
        }
    }

    // weight of the RopeNode is weight of Node or bufLen of Leaf
    fn weight(&self) -> usize {
        match self {
            Rope::Leaf(leaf) => return leaf.weight(),
            Rope::Node(node) => return node.weight,
        }
    }

    // length of the RopeNode is either the weight of the node if its leaf
    // or its weight (left child weight) + length of its right node
    fn length(&self) -> usize {
        match self {
            Rope::Leaf(leaf) => return leaf.weight(),
            Rope::Node(node) => {
                return node.weight
                    + node
                        .right
                        .as_ref()
                        .expect("right node cannot be None")
                        .length()
            }
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Rope::Leaf(_) => true,
            Rope::Node(_) => false,
        }
    }

    fn is_node(&self) -> bool {
        match self {
            Rope::Leaf(_) => false,
            Rope::Node(_) => true,
        }
    }

    fn join(r: Box<Rope>, s: Box<Rope>) -> Rope {
        Rope::Node(Node {
            weight: r.length(),
            left: Some(r),
            right: Some(s),
        })
    }

    fn split(&self, offset: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(leaf) => {
		let (l, r) = leaf.split(offset);
		return (Rope::Leaf(l), Rope::Leaf(r))
            }
            Rope::Node(node) => {
		let w = node.weight;

		// < not <= because w - always length of the string (offset -1)
                if offset < w {
		    let (l, r) = node.left.as_ref().expect("left child cannot be empty").split(offset);
		    r = Rope::join(Box::new(r), node.right.expect("right child cannot be empty"));
		    return (l, r)
                    // return node.left.as_ref().unwrap().split(offset);
                }

		let (l, r) = node.right.as_ref().expect("right child cannot be empty").split(offset -w);
		l = Rope::join(Box::new(l), node.right.expect("left child cannot be empty"));
		return (l, r)
            }
        }
    }

    // fn report(&self, start: usize, end: usize) -> String {
    // 	let res:String;
    // 	let b = match self {
    // 	    Rope::Leaf(leaf) => {
    // 		return leaf.report(start, end)
    // 	    }
    // 	    Rope::Node(node) => {
    // 		let offset = node.
    // 		if offset <= node.weight {

    // 		}
    // 	    }
    // 	}
    // }

    // fn join(&self)
}

#[test]
fn test_rope_new() {
    let rope = Rope::new("Hello, World!");
    assert_eq!(rope.is_leaf(), true);
}

#[test]
fn test_rope_index() {
    let rope = Rope::new("Hello, World!");
    assert_eq!(rope.index(1).unwrap(), 'e');
    assert_eq!(rope.index(0).unwrap(), 'H');
    assert_eq!(rope.index(3).unwrap(), 'l');
    assert_eq!(rope.index(12).unwrap(), '!');
}

#[test]
fn test_rope_join() {
    let rope1 = Rope::new("Hello,");
    let rope2 = Rope::new(" World!");

    let rope = Rope::join(Box::new(rope1), Box::new(rope2));

    println!("weight: {:?}", rope.weight());

    assert_eq!(rope.index(1).unwrap(), 'e');
    assert_eq!(rope.index(0).unwrap(), 'H');
    assert_eq!(rope.index(3).unwrap(), 'l');
    assert_eq!(rope.index(12).unwrap(), '!');
}

// #[test]
// fn test_rope_split() {
//     let rope = Rope::New("Hello, World!");
//     let (left, right) =  rope.split(5);
//     assert_eq!(left.buf.to_string(), "Hello");
//     assert_eq!(right.buf.to_string(), ", World!");
//     println!("rope: {:?}, left: {:?}, right: {:?}", rope, left, right)
// }
