pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod matrix {
    pub struct Matrix {
        length: usize,
        width: usize,
    }

    impl Matrix {
        pub fn can_hold(&self, other: &Matrix) -> bool {
            self.length >= other.length && self.width >= other.width
        }
        pub fn new(length: usize, width: usize) -> Matrix {
            if length >= 100 {
                panic!("超出了尺寸限制")
            }
            if width >= 100 {
                panic!("超出了尺寸限制")
            }
            Matrix { length, width }
        }
        pub fn copy(&self) -> Result<(), String> {
            if self.length >= 50 || self.width >= 50 {
                Err(String::from("err message"))
            }else {
                Ok(())   
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        let i = 2;
        assert_ne!(i, 1)
    }

    #[test]
    fn matrix_test() {
        let one = Matrix::new(10, 20);
        let two = Matrix::new(5, 5);
        assert!(one.can_hold(&two), "测试不通过")
    }

    #[test]
    #[should_panic]
    fn matrix_create_test() {
        let one = Matrix::new(200, 20);
    }

    #[test]
    fn matrix_copy_test()->Result<(),String>{
        let one = Matrix::new(30, 30);
        let r = one.copy();
        return r;
    }
}
