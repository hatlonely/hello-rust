#[cfg(test)]
mod tests {
    #[test]
    fn structures() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let person1 = Person {
            name: String::from("Alice"),
            age: 30,
        };

        println!("person: {:?}", person1);
        println!("person.name: {}", person1.name);
        println!("person.age: {}", person1.age);

        impl Default for Person {
            fn default() -> Self {
                Person {
                    name: String::from("John Doe"),
                    age: 20,
                }
            }
        }

        let person2 = Person::default();
        println!("default_person: {:?}", person2);

        let person3 = Person {
            name: String::from("Alice"),
            ..Person::default()
        };
        println!("person: {:?}", person3);

        let person4 = Person {
            name: String::from("Bob"),
            ..person1
        };
        println!("person: {:?}", person4);

        let Person { name, age } = person4;
        println!("name: {}", name);
        println!("age: {}", age);
    }

    #[test]
    fn enums() {
        enum WebEvent {
            PageLoad,
            PageUnload,
            KeyPress(char),
            Paste(String),
            Click { x: i64, y: i64 },
        }

        let pressed = WebEvent::KeyPress('x');
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                WebEvent::KeyPress(c) => println!("pressed '{}'", c),
                WebEvent::Paste(s) => println!("pasted \"{}\"", s),
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}", x, y);
                }
            }
        }

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);
    }

    #[test]
    fn constants() {
        const PI: f32 = 3.14159;
        println!("PI: {}", PI);

        static mut G: i32 = 10;
        unsafe {
            println!("G: {}", G);
            G = 20;
            println!("G: {}", G);
        }
    }
}
