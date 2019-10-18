## Rope

[Rope](https://en.wikipedia.org/wiki/Rope_(data_structure)) is a cool data structure to work with very long string(say text editor).

A rope is a binary tree where each leaf (end node) holds a string and a length (also known as a "weight"), and each node further up the tree holds the sum of the lengths of all the leaves in its left subtree. A node with two children thus divides the whole string into two parts: the left subtree stores the first part of the string, the right subtree stores the second part of the string, and node's weight is the sum of the left child's weight along with all of the nodes contained in its subtree.


![Rope Image](https://upload.wikimedia.org/wikipedia/commons/thumb/2/20/Vector_Rope_index.svg/853px-Vector_Rope_index.svg.png)


## Usage

```rust

fn main() {
	// Inserting.
	let mut rope = Rope::new("Hello, World!");
	let rope = rope.insert(" Cruel", 6);
	assert_eq!(rope.report(0, 18).unwrap(), "Hello, Cruel World!");
	
	// Deleting
	let mut rope = Rope::new("Hello, World!");
	rope = rope.delete(2, 4);
	assert_eq!(rope.report(0, 9).unwrap(), "He, World!");

	// Iterator
	let rope = Rope::new("Hello!");
	for c in rope.into_iter() {
		println!("{}", c);
	}
}
```

## TODO

- [ ] Current rope implmentation becomes unbalanced too soon. Need to balance the tree
after every `split` and `join`.
- [ ] Cleanup a bit for proper error handling.
- [ ] Current iterator works in O(nlogn). Improve it!
