struct BigNum {
    v: Vec<i32>
}

impl BigNum {
    // Digits are stored in reverse order (small digits first): [1,2,3] is 321 in base ten
    pub fn new(v: Vec<i32>) -> BigNum {
        BigNum{
            v
        }
    }

    pub fn add(&self, b:&BigNum) -> BigNum {
        let mut v = Vec::new();
        let n = self.v.len();
        let m = b.v.len();
        let p = n.max(m);
        v.resize(p, 0);
        let mut carry = 0;
        for i in 0..p {
            let mut t = carry;
            if i < n { t += self.v[i]; }
            if i < m { t += b.v[i]; }
            v[i] = t % 10;
            carry = t/10;
        }
        if carry > 0 { v.push(1); }
        BigNum::new(v)
    }
}

#[test]
fn instruction_push_nominal() {
    let a = BigNum::new(vec![3,2,1]); // 123
    let b = BigNum::new(vec![4,5,6]); // 654
    let c = a.add(&b);
    assert_eq!(vec![7,7,7], c.v);
}

#[test]
fn instruction_push_nominal2() {
    let a = BigNum::new(vec![2,1]);   // 12
    let b = BigNum::new(vec![4,3,7]);   // 732
    let c = a.add(&b);
    assert_eq!(vec![6,4,7], c.v);
}