use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

// impl Add for Complex {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         let real = self.real + rhs.real;
//         let imagine = self.imagine + rhs.imagine;
//         Self::new(real, imagine)
//     }
// }

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}

impl Add<&str> for &Complex {
    type Output = Complex;

    fn add(self, rhs: &str) -> Self::Output {
        if let Ok(v) = rhs.parse::<f64>() {
            let real = self.real + v;
            return Complex::new(real, self.imagine);
        }
        Complex::new(self.real, self.imagine)
    }
}

fn main() {
    {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(1 as f64, 3.2);
        println!("{:?}", &c1 + &c2);
    }
    {
        let c1 = Complex::new(1.0, 2.0);
        println!("{:?}", &c1 + 5.9);
    }
    {
        let c1 = Complex::new(1.0, 2.0);
        println!("{:?}", &c1 + "7.6");
    }
}
