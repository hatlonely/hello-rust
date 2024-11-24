// https://doc.rust-lang.org/rust-by-example/std/vec.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_vector_create() {
        let vec1 = vec![1, 2, 3];
        assert_eq!(vec1, vec![1, 2, 3]);

        let vec2 = vec![0; 3];
        assert_eq!(vec2, vec![0, 0, 0]);

        let vec3 = vec![1; 3];
        assert_eq!(vec3, vec![1, 1, 1]);

        let mut vec4 = vec![0; 3];
        vec4.fill(4);
        assert_eq!(vec4, vec![4, 4, 4]);

        let vec5 = vec1.repeat(2);
        assert_eq!(vec5, vec![1, 2, 3, 1, 2, 3]);

        let mut vec6: Vec<i32> = Vec::with_capacity(3);
        vec6.push(1);
        vec6.push(2);
        vec6.push(3);
        assert_eq!(vec6, vec![1, 2, 3]);
    }

    #[test]
    fn test_vector_get() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
        assert_eq!(vec.is_empty(), false);
        assert_eq!(vec.first(), Some(&1));
        assert_eq!(vec.last(), Some(&3));
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));
        assert_eq!(vec.get(3), None);
        assert_eq!(vec.contains(&1), true);
    }

    #[test]
    fn test_vector_modify() {
        let mut vec = vec![1, 2, 3];

        vec.push(4);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        vec.pop();
        assert_eq!(vec, vec![1, 2, 3]);

        vec.insert(1, 5);
        assert_eq!(vec, vec![1, 5, 2, 3]);

        vec.remove(1);
        assert_eq!(vec, vec![1, 2, 3]);

        vec.swap(0, 2);
        assert_eq!(vec, vec![3, 2, 1]);

        vec.reverse();
        assert_eq!(vec, vec![1, 2, 3]);

        vec.sort();
        assert_eq!(vec, vec![1, 2, 3]);

        vec.sort_by(|a, b| b.cmp(a));
        assert_eq!(vec, vec![3, 2, 1]);

        let mut vec = vec.repeat(2);
        assert_eq!(vec, vec![3, 2, 1, 3, 2, 1]);

        vec.sort();
        assert_eq!(vec, vec![1, 1, 2, 2, 3, 3]);

        vec.dedup();
        assert_eq!(vec, vec![1, 2, 3]);

        use std::iter;
        vec.extend(iter::once(4));
        assert_eq!(vec, vec![1, 2, 3, 4]);

        vec.extend(vec![5, 6]);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);

        vec.clear();
        assert_eq!(vec.is_empty(), true);
    }

    #[test]
    fn test_vector_join() {
        let vec = vec![1, 2, 3];
        let joined = vec
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        assert_eq!(joined, "1, 2, 3");
    }
}
