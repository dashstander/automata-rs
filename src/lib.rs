
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Permutation<const N: usize> {
    data: [usize; N]
}


#[derive(Debug)]
struct SymmetricGroup<const N: usize> {
    elements: Vec<Permutation<N>>
}

  
impl<const N: usize> Permutation<N> {
  
    fn new(v: &[usize; N]) -> Self {
      Self { data: *v } 
    }

    fn apply(&self, v: &mut Vec<i32>) {
        let tmp = v.clone();
        for i in 0..N {
            v[i] = tmp[self.data[i]];
        }
    }

    fn make_identity_perm(&self) -> Permutation<N> {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = i;
        }
        Permutation::<N>::new(&data)
    }

    fn multiply(&self, other: &Permutation<N>) -> Permutation<N> {
    
        let mut result = [0; N];
        
        for i in 0..N {
          result[i] = other.data[self.data[i]]; 
        }
    
        Permutation::<N>::new(&result)
    }

    fn pow(&self, n: u32) -> Permutation<N> {
        match n {
            0 => self.make_identity_perm(),
            1 => self.clone(),
            n => {
                let mut tmp = self.clone();
                for i in 1..n {
                    print!{"{}", i}
                    tmp = *self * tmp;
                }
                tmp
            }
        }
    }
}

impl<const N: usize> std::ops::Mul for Permutation<N> {

    type Output = Permutation<N>;
  
    fn mul(self, other: Permutation<N>) -> Permutation<N> {
      
      self.multiply(&other)
    
    }
  
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute() {

        let mut v = vec![1, 2, 3, 4, 5];
        let res = vec![3, 5, 1, 2, 4];

        let p = Permutation::<5>::new(&[2, 4, 0, 1, 3]); 
        p.apply(&mut v);
        assert_eq!(v, res);
    }

    #[test]
    fn multiply() {
        let p1 = Permutation::<6>::new(&[1, 0, 2, 3, 4, 5]);
        let p2 = Permutation::<6>::new(&[5, 4, 3, 2, 1, 0]);

        let res1 = Permutation::<6>::new(&[4, 5, 3, 2, 1, 0]);
        let res2 = Permutation::<6>::new(&[5, 4, 3, 2, 0, 1]);

        assert_eq!(p1 * p2, res1);
        assert_eq!(p2 * p1, res2);
    }


    #[test]
    fn pow() {
        let p1 = Permutation::<6>::new(&[1, 0, 2, 3, 4, 5]);
        let p2 = Permutation::<6>::new(&[5, 4, 3, 2, 1, 0]);
        let p6 = Permutation::<6>::new(&[5, 0, 1, 2, 3, 4]);

        assert_eq!(p1.pow(0), p2.pow(0));
        assert_eq!(p1.pow(1), p1);
        assert_eq!(p1.pow(2), p2.pow(2));
        assert_eq!(p6.pow(6), p5.pow(0));

    }


}
