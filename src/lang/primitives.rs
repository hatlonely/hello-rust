#[cfg(test)]
mod tests {
    #[test]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    fn main() {
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
}
