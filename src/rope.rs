use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    weight: usize,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

#[derive(Debug)]
pub struct Leaf {
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
            return (Leaf::new(""), Leaf::new(&self.buf.as_ref().clone()));
        }

        if offset >= self.weight() {
            return (Leaf::new(&self.buf.as_ref().clone()), Leaf::new(""));
        }

        let (left, right) = self.buf.split_at(self.start + offset);
        ((Leaf::new(left)), Leaf::new(right))
    }

    fn report(&self, start: usize, end: usize) -> Option<String> {
        if start >= self.start && end <= self.end {
            return Some(self.buf[start..end + 1].to_string());
        }
        None
    }
}

#[derive(Debug)]
pub enum Rope {
    Node(Node),
    Leaf(Leaf),
}

impl Rope {
    pub fn new(s: &str) -> Rope {
        Rope::Leaf(Leaf::new(s))
    }

    fn buf(&self) -> Option<&str> {
        match self {
            Rope::Node(_) => None,
            Rope::Leaf(leaf) => Some(&leaf.buf),
        }
    }

    pub fn index(&self, i: usize) -> Option<char> {
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

    fn join(left: Box<Rope>, right: Box<Rope>) -> Rope {
        Rope::Node(Node {
            weight: left.length(),
            left: Some(left),
            right: Some(right),
        })
    }

    fn split(&mut self, offset: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(leaf) => {
                let (l, r) = leaf.split(offset);
                return (Rope::Leaf(l), Rope::Leaf(r));
            }
            Rope::Node(node) => {
                let w = node.weight;

                // < not <= because w - always length of the string (offset -1)
                if offset < w {
                    let (l, r) = node
                        .left
                        .as_mut()
                        .expect("left child cannot be empty")
                        .split(offset);
                    let r = Rope::join(
                        Box::new(r),
                        node.right.take().expect("right child cannot be empty"),
                    );
                    return (l, r);
                }

                let (l, r) = node
                    .right
                    .as_mut()
                    .expect("right child cannot be empty")
                    .split(offset - w);
                let l = Rope::join(
                    Box::new(l),
                    node.right.take().expect("left child cannot be empty"),
                );
                return (l, r);
            }
        }
    }

    pub fn insert(&mut self, s: &str, offset: usize) -> Rope {
        let (l, r) = self.split(offset);

        let leaf = Rope::new(s);

        let tmp = Rope::join(Box::new(l), Box::new(leaf));
        let res = Rope::join(Box::new(tmp), Box::new(r));
        return res;
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Rope {
        let (l, mut r) = self.split(start);

        let (_, r2) = r.split(end - start + 1);

        Rope::join(Box::new(l), Box::new(r2))
    }

    pub fn report(&self, start: usize, end: usize) -> Option<String> {
        match self {
            Rope::Leaf(leaf) => {
                leaf.report(start, end)
            }
            Rope::Node(node) => {
                let len = end - start + 1;
                if len <= node.weight {
                    return node.left.as_ref()?.report(start, end);
                }
                let l = node.left.as_ref()?.report(start, node.weight - 1)?;
                let r = node.right.as_ref()?.report(0, len - node.weight - 1)?;
                Some(l + &r)
            }
        }
    }
}

impl IntoIterator for Rope {
    type Item = char;
    type IntoIter = RopeIterator;

    fn into_iter(self) -> Self::IntoIter {
	RopeIterator {
	    rope: self,
	    index: 0
	}
    }
}

pub struct RopeIterator {
    rope: Rope,
    index: usize,
}

impl Iterator for RopeIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
	let res = self.rope.index(self.index);
	self.index += 1;
	res
    }
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

    assert_eq!(rope.index(1).unwrap(), 'e');
    assert_eq!(rope.index(0).unwrap(), 'H');
    assert_eq!(rope.index(3).unwrap(), 'l');
    assert_eq!(rope.index(12).unwrap(), '!');
}

#[test]
fn test_rope_split() {
    let mut rope = Rope::new("Hello, World!");
    let (left, right) = rope.split(5);
    assert_eq!(left.buf(), Some("Hello"));
    assert_eq!(right.buf(), Some(", World!"));
}

#[test]
fn test_rope_report() {
    let mut rope = Rope::new("Hello, World!");

    assert_eq!(rope.report(1, 5).unwrap(), "ello,");

    let (left, right) = rope.split(5);
    assert_eq!(left.report(0, 4).unwrap(), "Hello");
    assert_eq!(right.report(0, 7).unwrap(), ", World!");
    assert_eq!(right.report(0, 8), None);
}

#[test]
fn test_rope_insert() {
    let mut rope = Rope::new("Hello, World!");

    let rope = rope.insert(" Cruel", 6);

    assert_eq!(rope.report(0, 18).unwrap(), "Hello, Cruel World!");
}

#[test]
fn test_rope_delete() {
    let mut rope = Rope::new("Hello, World!");
    rope = rope.delete(2, 4);
    assert_eq!(rope.report(0, 9).unwrap(), "He, World!");
}


#[test]
fn test_rope_iterator() {
    let rope = Rope::new("Hello!");
    let mut itr = rope.into_iter();

     assert_eq!(itr.next(), Some('H'));
     assert_eq!(itr.next(), Some('e'));
     assert_eq!(itr.next(), Some('l'));
     assert_eq!(itr.next(), Some('l'));
     assert_eq!(itr.next(), Some('o'));
     assert_eq!(itr.next(), Some('!'));
     assert_eq!(itr.next(), None);
}
