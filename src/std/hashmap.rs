// https://doc.rust-lang.org/rust-by-example/std/hash.html

#[cfg(test)]
mod tests {

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let mut contacts = HashMap::new();
        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        println!("{:?}", contacts);
        println!("{:?}", contacts.get("Daniel"));
        println!("{:?}", contacts["Daniel"]);

        assert_eq!(contacts.get("Daniel"), Some(&"798-1364"));
        assert_eq!(contacts["Daniel"], "798-1364");

        contacts.remove("Daniel");

        for (contact, &number) in contacts.iter() {
            println!("Calling {} at {}", contact, number);
        }
    }

    #[test]
    fn test_hashset() {
        use std::collections::HashSet;

        let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();

        assert!(a.contains(&3));
        let ok = a.insert(4);
        assert!(ok);
        assert!(a.contains(&4));

        let b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
        assert!(a != b);

        let c = a.union(&b).collect::<HashSet<&i32>>();
        assert_eq!(c, vec![&1, &2, &3, &4].into_iter().collect());
    }
}
