use std::rc::Rc;

#[derive(Debug)]
struct Node {
    weight: usize,
    left: Rc<Rope>,
    right: Rc<Rope>,
}

#[derive(Debug)]
struct Leaf {
    buf: Rc<String>,
    start: usize,
    end: usize,
}

impl Leaf {
    fn New(s: &str) -> Leaf {
        let leaf = Leaf {
            buf: Rc::new(s.clone().to_string()),
            start: 0,
            end: s.len()-1,
        };
	leaf
    }
}

#[derive(Debug)]
enum Rope {
    Node(Node),
    Leaf(Leaf),
}

impl Rope {
    fn New(s: &str) -> Rope {
        Rope::Leaf(Leaf::New(s))
    }

    fn index(&self, i:usize) -> Option<char> {
	match self {
	    Rope::Leaf(leaf) => return leaf.buf.chars().nth(i),
	    Rope::Node(node) => {
		if node.weight <= i {
		    return node.left.as_ref().index(i)
		}
		return node.right.as_ref().index(i-node.weight)
	    }
	}
    }

    fn length(&self) -> usize {
	match self {
	    Rope::Leaf(leaf) => return leaf.end - leaf.start + 1,
	    Rope::Node(node) => {
		return node.weight + node.right.length()
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

    fn join(r:Rope, s:Rope) -> Rope {
	Rope::Node(Node{
	    weight: r.length(),
	    left: Rc::new(r),
	    right: Rc::new(s),
	})
    }

    fn split(&self, offset: usize) -> (Rope, Rope){
	match self {
	    Rope::Leaf(leaf) => {
		assert!(offset < leaf.end-leaf.start);
		let (left, right)  = leaf.buf.split_at(leaf.start+offset);
		(Rope::Leaf(Leaf::New(left)), Rope::Leaf(Leaf::New(right)))
	    }
	    Rope::Node(node) => {
		if offset <= node.weight {
		    return node.left.as_ref().split(offset)
		}
		return node.right.as_ref().split(offset - node.weight)
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
    let rope = Rope::New("Hello, World!");
    assert_eq!(rope.is_leaf(), true);
}

#[test]
fn test_rope_index() {
    let rope = Rope::New("Hello, World!");
    assert_eq!(rope.index(1), 'e');
    assert_eq!(rope.index(0), 'H');
    assert_eq!(rope.index(3), 'l');
    assert_eq!(rope.index(12), '!');
}

#[test]
fn test_rope_join() {
    let rope1 = Rope::New("Hello,");
    let rope2 = Rope::New(" World!");

    let rope = Rope::join(rope1, rope2);
    assert_eq!(rope.index(1), 'e');
    assert_eq!(rope.index(0), 'H');
    assert_eq!(rope.index(3), 'l');
    assert_eq!(rope.index(12), '!');
}

// #[test]
// fn test_rope_split() {
//     let rope = Rope::New("Hello, World!");
//     let (left, right) =  rope.split(5);
//     assert_eq!(left.buf.to_string(), "Hello");
//     assert_eq!(right.buf.to_string(), ", World!");
//     println!("rope: {:?}, left: {:?}, right: {:?}", rope, left, right)
// }
