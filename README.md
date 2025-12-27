# trees-rs

LinksPlatform's Platform.Trees Rust Library.

This library provides low-level tree and linked list data structure traits for the [Links Platform](https://github.com/linksplatform) ecosystem in Rust.

## Overview

`platform-trees` is a Rust implementation of tree and linked list methods used by the Links Platform. It provides generic traits that can be implemented for various storage backends, enabling efficient tree-based data structures with size-balanced binary (SZB) tree algorithms.

## Features

### Tree Structures
- **`SzbTree`** - Size-balanced binary tree trait with core operations:
  - Node navigation (`get_left`, `get_right`, `get_next`, `get_previous`)
  - Tree rotations (`left_rotate`, `right_rotate`)
  - Size management (`get_size`, `fix_size`, `inc_size`, `dec_size`)
  - Tree queries (`contains`, `get_leftest`, `get_rightest`)

- **`NoRecurSzbTree`** - Non-recursive size-balanced tree trait extending `SzbTree`:
  - Iterative `attach` and `detach` operations
  - Avoids stack overflow on deep trees
  - Maintains tree balance during modifications

### Linked List Structures
- **`LinkedList`** - Base doubly-linked list trait with `get_previous`, `get_next`, `set_previous`, `set_next`

- **`AbsoluteLinkedList`** - Linked list with absolute positioning:
  - Direct access to `first` and `last` elements
  - Size tracking

- **`RelativeLinkedList`** - Linked list with head-relative positioning:
  - Multiple independent lists sharing storage
  - Head parameter for list identification

- **`AbsoluteCircularLinkedList`** - Circular doubly-linked list with absolute positioning:
  - `attach_before`, `attach_after`, `attach_as_first`, `attach_as_last`
  - `detach` operation

- **`RelativeCircularLinkedList`** - Circular doubly-linked list with head-relative positioning:
  - All circular list operations with head parameter
  - Supports multiple circular lists in shared storage

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
platform-trees = "0.1.0-beta.1"
```

### Example: Implementing SzbTree

```rust
use platform_trees::{SzbTree, NoRecurSzbTree};
use platform_data::LinkType;

// Define your tree node storage
struct MyTreeStorage {
    nodes: Vec<Node>,
}

struct Node {
    left: usize,
    right: usize,
    size: usize,
    // ... your data
}

// Implement the SzbTree trait for your storage type
impl SzbTree<usize> for MyTreeStorage {
    unsafe fn get_mut_left_reference(&mut self, node: usize) -> *mut usize {
        &mut self.nodes[node].left
    }

    unsafe fn get_mut_right_reference(&mut self, node: usize) -> *mut usize {
        &mut self.nodes[node].right
    }

    unsafe fn get_left_reference(&self, node: usize) -> *const usize {
        &self.nodes[node].left
    }

    unsafe fn get_right_reference(&self, node: usize) -> *const usize {
        &self.nodes[node].right
    }

    unsafe fn get_left(&self, node: usize) -> usize {
        self.nodes[node].left
    }

    unsafe fn get_right(&self, node: usize) -> usize {
        self.nodes[node].right
    }

    unsafe fn get_size(&self, node: usize) -> usize {
        self.nodes[node].size
    }

    unsafe fn set_left(&mut self, node: usize, left: usize) {
        self.nodes[node].left = left;
    }

    unsafe fn set_right(&mut self, node: usize, right: usize) {
        self.nodes[node].right = right;
    }

    unsafe fn set_size(&mut self, node: usize, size: usize) {
        self.nodes[node].size = size;
    }

    unsafe fn first_is_to_the_left_of_second(&self, first: usize, second: usize) -> bool {
        first < second
    }

    unsafe fn first_is_to_the_right_of_second(&self, first: usize, second: usize) -> bool {
        first > second
    }
}

// Implement NoRecurSzbTree to get attach/detach operations
impl NoRecurSzbTree<usize> for MyTreeStorage {}
```

### Example: Implementing LinkedList

```rust
use platform_trees::{LinkedList, AbsoluteLinkedList, AbsoluteCircularLinkedList};
use platform_data::LinkType;

struct MyListStorage {
    elements: Vec<ListElement>,
    first: usize,
    last: usize,
    size: usize,
}

struct ListElement {
    prev: usize,
    next: usize,
}

impl LinkedList<usize> for MyListStorage {
    fn get_previous(&self, element: usize) -> usize {
        self.elements[element].prev
    }

    fn get_next(&self, element: usize) -> usize {
        self.elements[element].next
    }

    fn set_previous(&mut self, element: usize, previous: usize) {
        self.elements[element].prev = previous;
    }

    fn set_next(&mut self, element: usize, next: usize) {
        self.elements[element].next = next;
    }
}

impl AbsoluteLinkedList<usize> for MyListStorage {
    fn get_first(&self) -> usize { self.first }
    fn get_last(&self) -> usize { self.last }
    fn get_size(&self) -> usize { self.size }

    fn set_first(&mut self, element: usize) { self.first = element; }
    fn set_last(&mut self, element: usize) { self.last = element; }
    fn set_size(&mut self, size: usize) { self.size = size; }
}

// Now AbsoluteCircularLinkedList methods are available
impl AbsoluteCircularLinkedList<usize> for MyListStorage {}
```

## API Reference

### Tree Traits

| Trait | Description |
|-------|-------------|
| `SzbTree<T>` | Base trait for size-balanced binary trees with rotation and navigation operations |
| `NoRecurSzbTree<T>` | Extension trait providing iterative attach/detach without recursion |

### List Traits

| Trait | Description |
|-------|-------------|
| `LinkedList<T>` | Base trait for doubly-linked list navigation |
| `AbsoluteLinkedList<T>` | Linked list with global first/last/size |
| `RelativeLinkedList<T>` | Linked list with head-relative first/last/size |
| `AbsoluteCircularLinkedList<T>` | Circular list operations with absolute positioning |
| `RelativeCircularLinkedList<T>` | Circular list operations with relative positioning |

## Dependencies

- [platform-data](https://github.com/linksplatform/Data) - LinksPlatform's core data traits (provides `LinkType`)
- [funty](https://crates.io/crates/funty) - Fundamental type unification

## Related Projects

- [linksplatform/Collections.Methods](https://github.com/linksplatform/Collections.Methods) - C#/C++ implementation of these methods
- [linksplatform/Data.Doublets](https://github.com/linksplatform/Data.Doublets) - Doublets data structure using these tree methods
- [linksplatform/mem-rs](https://github.com/linksplatform/mem-rs) - Memory management for Links Platform Rust libraries

## License

This project is released into the public domain under the [Unlicense](LICENSE).

## Support

Ask questions at [stackoverflow.com/tags/links-platform](https://stackoverflow.com/tags/links-platform) (or with tag `links-platform`) to get our free support.

You can also get real-time support on [our official Discord server](https://discord.gg/eEXJyjWv5e).
