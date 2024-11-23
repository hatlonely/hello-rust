// https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html

#[cfg(test)]
mod tests {
    #[test]
    fn functions() {
        fn add<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<Output = T>,
        {
            a + b
        }

        println!("add(1, 2): {}", add(1, 2));
        println!("add(1.1, 2.2): {}", add(1.1, 2.2));
    }

    #[test]
    fn implementation() {
        struct Node<T> {
            value: T,
            next: Option<Box<Node<T>>>,
        }

        struct List<T> {
            head: Option<Box<Node<T>>>,
        }

        impl<T> List<T> {
            fn new() -> Self {
                List { head: None }
            }

            fn push(&mut self, value: T) {
                let new_node = Box::new(Node {
                    value,
                    next: self.head.take(),
                });
                self.head = Some(new_node);
            }

            fn pop(&mut self) -> Option<T> {
                self.head.take().map(|node| {
                    self.head = node.next;
                    node.value
                })
            }
        }

        let mut list = List::new();
        list.push(1);
        list.push(2);

        println!("list.pop(): {:?}", list.pop());
        println!("list.pop(): {:?}", list.pop());
        println!("list.pop(): {:?}", list.pop());
    }
}
