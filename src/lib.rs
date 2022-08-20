use num_traits::{FromPrimitive, NumOps, Zero};

#[derive(Debug, Clone)]
pub struct VecPolynomial<T> {
    coefficients: Vec<T>
}

#[derive(Debug, Clone)]
pub struct FixedPolynomial<T, const N: usize> {
    coefficients: [T; N]
}

pub trait Polynomial<T: Copy + NumOps + Zero + FromPrimitive> {
    fn coefficients(&self) -> &[T];

    fn evaluate(&self, x: T) -> T {
        // use Horner schema to evaluate
        self.coefficients().iter().rfold(T::zero(), |acc, &a| acc*x + a)
    }

    fn evaluate_derivative(&self, x: T) -> T {
        // use Horner schema to evaluate
        let degree = self.coefficients().len();
        self.coefficients().iter().zip(degree..0).rfold(T::zero(), |acc, (&a, i)| {
            acc*x + T::from_usize(i).unwrap() * a
        })
    }
}

impl<T: Copy + NumOps + Zero + FromPrimitive> Polynomial<T> for VecPolynomial<T> {
    fn coefficients(&self) -> &[T] {
        &self.coefficients
    }
}

impl<T: Copy + NumOps + Zero + FromPrimitive, const N: usize> Polynomial<T> for FixedPolynomial<T, N> {
    fn coefficients(&self) -> &[T] {
        &self.coefficients
    }
}


pub fn it_works_vec(p: &VecPolynomial<f64>, x: f64) -> f64 {
    let result = p.evaluate_derivative(x);
    result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_vec() {
        let p = VecPolynomial::<i32> { coefficients: vec![-1, 2, 3] };
        let result = p.evaluate(42);
        assert_eq!(result, 3*42*42 + 2*42 - 1);
    }

    #[test]
    fn it_works_fixed() {
        let p = FixedPolynomial { coefficients: [-1, 2, 3] };
        let result = p.evaluate(42);
        assert_eq!(result, 3*42*42 + 2*42 - 1);
    }
}
