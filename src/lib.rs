//! These wrappers enable the [“fast-math”][1] flags for the operations
//! where there are intrinsics for this (add, sub, mul, div, rem).
//! The wrappers exist so that we have a quick & easy way **to experiment**
//! with fast math flags and further that feature in Rust.
//!
//! Note that as of this writing, the Rust instrinsics use the “fast” flag
//! documented in the langref; this enables all the float flags.
//!
//! This crate does not use `![always(inline)]`, so for best performance,
//! compile your crate with `lto=true`.
//!
//! [1]: http://llvm.org/docs/LangRef.html#fast-math-flags
//!
#![feature(core_intrinsics)]
#![feature(type_ascription)]

use num_traits::{Float,One,Zero};

use std::cmp::Ordering;
use std::intrinsics;
use std::iter::Sum;
use std::mem;
use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Neg,
    Div,
    DivAssign,
    Rem,
    RemAssign,
};

/// “fast-math” wrapper for f32 and f64.
///
/// The `Fast` type enforces no invariant and can hold any f32, f64 values.
/// Type synonyms `F32` and `F64` are provided for convience.
/// Values can be unwrapped with `.0`.
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Fast<F>(pub F);

pub type F32 = Fast<f32>;
pub type F64 = Fast<f64>;

/// Convenience function for wrapping a floating point value
pub fn fa<F:Float>(x: F) -> Fast<F> {
    Fast(x)
}

macro_rules! impl_op {
    ($($name:ident, $method:ident, $intrins:ident;)*) => {
        $(
            // Fast<F> + F
            impl<F:Float> $name<F> for Fast<F> {
                type Output = Self;
                fn $method(self, rhs: F) -> Self::Output {
                    unsafe {
                        fa(intrinsics::$intrins(self.0, rhs))
                    }
                }
            }
            
            // F + Fast<F>
            impl $name<F32> for f32 {
                type Output = F32;
                fn $method(self, rhs: F32) -> Self::Output {
                    unsafe {
                        fa(intrinsics::$intrins(self, rhs.0))
                    }
                }
            }
            
            impl $name<F64> for f64 {
                type Output = F64;
                fn $method(self, rhs: F64) -> Self::Output {
                    unsafe {
                        fa(intrinsics::$intrins(self, rhs.0))
                    }
                }
            }
            
            // Fast<F> + Fast<F>
            impl<F:Float> $name for Fast<F> {
                type Output = Self;
                fn $method(self, rhs: Self) -> Self::Output {
                    unsafe {
                        fa(intrinsics::$intrins(self.0, rhs.0))
                    }
                }
            }
        )*

    }
}

impl_op! {
    Add, add, fadd_fast;
    Div, div, fdiv_fast;
    Mul, mul, fmul_fast;
    Rem, rem, frem_fast;
    Sub, sub, fsub_fast;
}

macro_rules! impl_assignop {
    ($($name:ident, $method:ident, $intrins:ident;)*) => {
        $(
            // Fast<F> += Fast<F>
            impl<F:Float> $name for Fast<F> {
                fn $method(&mut self, rhs: Self) {
                    unsafe {
                        (*self).0 = intrinsics::$intrins(self.0, rhs.0)
                    }
                }
            }

            // Fast<F> += F
            impl<F:Float> $name<F> for Fast<F> {
                fn $method(&mut self, rhs: F) {
                    unsafe {
                        (*self).0 = intrinsics::$intrins(self.0, rhs)
                    }
                }
            }
        )*
    }
}

impl_assignop! {
    AddAssign, add_assign, fadd_fast;
    DivAssign, div_assign, fdiv_fast;
    MulAssign, mul_assign, fmul_fast;
    RemAssign, rem_assign, frem_fast;
    SubAssign, sub_assign, fsub_fast;
}

impl<F:Float> Neg for Fast<F> {
    type Output = Self;

    fn neg(self) -> Self {
        fa(-self.0)
    }
}

impl<F:Float> One for Fast<F> {
    fn one() -> Self { fa(<_>::one()) }
    fn is_one(&self) -> bool { self.0.is_one() }
}

impl<F:Float> Zero for Fast<F> {
    fn zero() -> Self { fa(<_>::zero()) }
    fn is_zero(&self) -> bool { self.0.is_zero() }
}

use std::fmt;
macro_rules! impl_format {
    ($($name:ident)+) => {
        $(
        impl<F: fmt::$name> fmt::$name for Fast<F> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        )+
    }
}

impl_format!(Debug Display LowerExp UpperExp);

impl<F:Float> Eq for Fast<F> {}

impl<F:Float> Ord for Fast<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}


impl<F:Float> Fast<F> {
    pub fn abs(self) -> Self { fa(self.0.abs()) }
    pub fn ceil(self) -> Self { fa(self.0.ceil()) }
    pub fn exp(self) -> Self { fa(self.0.exp()) }
    pub fn floor(self) -> Self { fa(self.0.floor()) }
    pub fn ln(self) -> Self { fa(self.0.ln()) }
    pub fn mul_add(self, a: Self, b: Self) -> Self { fa(self.0.mul_add(a.0, b.0)) }
    pub fn powi(self, n: i32) -> Self { fa(self.0.powi(n)) }
    pub fn powf(self, n: F) -> Self { fa(self.0.powf(n)) }
    pub fn round(self) -> Self { fa(self.0.round()) }
    pub fn trunc(self) -> Self { fa(self.0.trunc()) }
    
}

impl F32 {
    /// Casts a wrapped f32 to a wrapped f64.
    pub fn as_64(self) -> F64 {
        fa(self.0 as f64)
    }
    
    pub fn fastexp(self) -> Self {
        let mut y = 1.0 + self*0.00390625;
        y *= y; y *= y; y *= y; y *= y;
        y *= y; y *= y; y *= y; y *= y;
        y
    }

    #[allow(overflowing_literals)]
    pub fn fastln(self) -> Self { unsafe {
        let selfi = mem::transmute(self):i32;
        let e = (selfi - 0x3f2aaaab) & 0xff800000;
        let m = mem::transmute(selfi - e):Self;
        let i = fa(e as f32) * 1.19209290e-7;
        let f = m - 1.0;
        let s = f*f;
        let mut r = fa(0.230836749).mul_add(f, fa(-0.279208571));
        let t = fa(0.331826031).mul_add(f, fa(-0.498910338));
        r = r.mul_add(s, t);
        r = r.mul_add(s, f);
        i.mul_add(fa(0.693147182), r)
    } }

    pub fn sqrt(self) -> Self {
        fa(unsafe { intrinsics::sqrtf32(self.0) })
    }
}

impl F64 {
    /// Casts a wrapped f64 to a wrapped f32.
    pub fn as_32(self) -> F32 {
        fa(self.0 as f32)
    }

    pub fn fastexp(self) -> Self {
        let mut y = 1.0 + self*0.00390625;
        y *= y; y *= y; y *= y; y *= y;
        y *= y; y *= y; y *= y; y *= y;
        y
    }

    pub fn fastln(self) -> Self {
        self.as_32().fastln().as_64()
    }

    pub fn sqrt(self) -> Self {
        fa(unsafe { intrinsics::sqrtf64(self.0) })
    }
}

impl<'a> Sum<&'a F32> for F32 {
    fn sum<I:Iterator<Item=&'a F32>>(iter: I) -> F32 {
        iter.fold(fa(0.0), |a,b| a + *b)
    }
}

impl<'a> Sum<&'a F64> for F64 {
    fn sum<I:Iterator<Item=&'a F64>>(iter: I) -> F64 {
        iter.fold(fa(0.0), |a,b| a + *b)
    }
}

impl<F:Float> PartialEq<F> for Fast<F> {
    fn eq(&self, other: &F) -> bool {
        self.0 == *other
    }
}

impl<F:Float> PartialOrd<F> for Fast<F> {
    fn partial_cmp(&self, other: &F) -> Option<Ordering> {
        Some(self.cmp(&fa(*other)))
    }
}
