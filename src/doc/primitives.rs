#[cfg(test)]
mod tests {
    #[test]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn literals() {
        let ok: bool = true;
        let b = false;

        let f: f32 = 1.0;
        let d: f64 = 1.0;
        let i: i32 = 1;
        let u: u32 = 1;

        let i64 = 1i64;
        let u64 = 1u64;

        let c: char = 'c';
        let s: &str = "string";

        let t = (1, 2);
        let a = [1, 2, 3];

        let v = vec![1, 2, 3];

        let o = Some(1);
        let n: Option<i32> = None;

        let e: Result<i32, &str> = Ok(1);
        let r: Result<i32, &str> = Err("error");

        let mut x = 1;
        x = 2;
    }

    #[test]
    fn tuples() {
        let t = (1u8, 2u32);
        println!("t: {:?}", t);
        println!("t.0: {:?}", t.0);
        println!("t.1: {:?}", t.1);

        let tt = ((1, 2), (3, 4));
        println!("tt: {:?}", tt);
        println!("tt.0.0: {:?}", tt.0 .0);
        println!("tt.0.1: {:?}", tt.0 .1);
        println!("tt.1.0: {:?}", tt.1 .0);
        println!("tt.1.1: {:?}", tt.1 .1);

        let (a, b) = t;
        println!("a: {:?}", a);
        println!("b: {:?}", b);
    }

    #[test]
    fn arrays() {
        let a = [1, 2, 3];
        println!("a: {:?}", a);
        println!("a[0]: {:?}", a[0]);
        println!("a[1]: {:?}", a[1]);
        println!("a[2]: {:?}", a[2]);
        println!("a.len(): {:?}", a.len());

        for i in 0..a.len() {
            println!("a[{}]: {:?}", i, a[i]);
        }

        let mut m = [0; 3];
        println!("m: {:?}", m);
        m[0] = 1;
        m[1] = 2;
        m[2] = 3;
        println!("m: {:?}", m);
    }

    #[test]
    fn slices() {
        let a = [1, 2, 3, 4, 5];
        let s = &a[1..4];
        println!("s: {:?}", s);
        println!("s[0]: {:?}", s[0]);
        println!("s[1]: {:?}", s[1]);
        println!("s[2]: {:?}", s[2]);

        for i in 0..a.len() {
            match s.get(i) {
                Some(x) => println!("a[{}]: {:?}", i, x),
                None => println!("a[{}]: None", i), // 超过范围的索引返回 None
            }
        }
    }
}
