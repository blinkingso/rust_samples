//! Chapter 12. Operator Overloading.
use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{Add, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Complex<T> {
    ///Real portion of the complex number;
    re: T,
    /// Imaginary portion of the complext number
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let (re, im) = (self.re, self.im);
        let (r_re, r_im) = (rhs.re, rhs.im);
        Complex {
            re: re * r_re - im * r_im,
            im: re * r_im + r_re * im,
        }
    }
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Complex {
            im: T::default(),
            re: T::default(),
        }
    }
}

impl<T> PartialEq<Complex<T>> for Complex<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: ToString + Display> ToString for Complex<T> {
    fn to_string(&self) -> String {
        format!("{} + {}i", self.re, self.im)
    }
}

impl<T: Eq> Eq for Complex<T> {}

#[test]
fn test_complex() {
    let zero = Complex::<isize>::default();
    let one = Complex { re: 1, im: 10 };
    let res = zero + one;
    let res = res + one;
    let res = res * Complex { re: 1, im: 10 };
    println!("res is : {}", res.to_string());

    assert_eq!(Complex { re: 1, im: -10 }, Complex { re: 1, im: -10 });
    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
}

#[derive(Debug, PartialEq)]
pub struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(std::cmp::Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> Interval<T> {
    pub fn new(lower: T, upper: T) -> Option<Interval<T>> {
        if lower > upper {
            return None;
        }

        Some(Interval { lower, upper })
    }
}

#[test]
fn test_interval() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 32,
            upper: 40
        }
    );

    let left = Interval::<u32>::new(10, 20).unwrap();
    let right = Interval::<u32>::new(30, 80).unwrap();
    assert!(left < right);
    assert!(!(left >= right));

    let middle = Interval::<u32>::new(20, 60).unwrap();
    let mut intervals = [left, right, middle];
    intervals.sort_by_key(|i| i.lower);
    println!("{:?}", intervals);
    intervals.sort_by_key(|i| Reverse(i.lower));
    println!("{:?}", intervals);
}

#[test]
fn test_index() {
    let mut m = HashMap::new();
    m.insert("SHI", 10);
    m.insert("BAI", 100);
    m.insert("QIAN", 1000);
    m.insert("WAN", 10000);
    m.insert("YI", 1_0000_0000);

    assert_eq!(*m.index("YI"), 1_0000_0000);
    assert_eq!(*m.index("QIAN"), 1_000);

    let mut desserts: Vec<&'static str> = vec!["Java", "Sona"];
    *desserts.index_mut(0) = "C++";
    println!("desserts: {:?}", desserts);

    {
        desserts[0] = "Python";
        *desserts.index_mut(0) = "C++";
        println!("desserts: {:?}", desserts);
    }
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

#[test]
fn test_image() {
    let image = Image {
        width: 4,
        pixels: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let row = image[1][1];
    println!("row[1][1] is {}", row);
}
