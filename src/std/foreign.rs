// https://doc.rust-lang.org/rust-by-example/std_misc/ffi.html

#[cfg(test)]
mod tests {
    #[test]
    fn test_foreign() {
        use std::fmt;

        #[cfg(target_family = "unix")]
        #[link(name = "m")]
        extern "C" {
            fn csqrtf(z: Complex) -> Complex;
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Complex {
            re: f32,
            im: f32,
        }

        impl fmt::Debug for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.im < 0. {
                    write!(f, "{}-{}i", self.re, -self.im)
                } else {
                    write!(f, "{}+{}i", self.re, self.im)
                }
            }
        }

        let z = Complex { re: -1., im: 0. };
        let z_sqrt = unsafe { csqrtf(z) };
        print!("the square root of {:?} is {:?}", z, z_sqrt);
    }
}
