use ark_bn254::{Fq as F, FqParameters, FrParameters};
use ark_ff::{BigInteger, Field, Fp256, FpParameters, PrimeField};
use ark_std::{One, UniformRand, Zero};
use num_bigint::BigUint;
use ruint::{aliases::U256, uint};

pub type Fr = U256;
pub type FieldElement = Fp256<FrParameters>;

pub const MODULUS: Fr =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);

macro_rules! Fr_mul {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = $a * $b
    }};
}

macro_rules! Fr_add {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = $a + $b
    }};
}

macro_rules! Fr_sub {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = $a - $b
    }};
}

macro_rules! Fr_neg {
    ($o:expr,$a:expr) => {{
        $o = -$a
    }};
}

macro_rules! Fr_inv {
    ($o:expr,$a:expr) => {{
        $o = $a.inverse().unwrap();
    }};
}

macro_rules! Fr_div {
    ($o:expr,$a:expr,$b:expr) => {{
        let tmp = $b.inverse().unwrap();
        $o = $a * tmp;
    }};
}

macro_rules! Fr_square {
    ($o:expr,$a:expr) => {{
        $o = $a * $a
    }};
}

macro_rules! Fr_shl {
    ($o:expr,$a:expr,$n:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let n = Fr_toInt!($n);
        $o = (t1 << n).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_shr {
    ($o:expr,$a:expr,$n:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let n = Fr_toInt!($n);
        $o = (t1 >> n).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_band {
    ($o:expr,$a:expr,$b:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let t2: U256 = $b.try_into().unwrap();
        $o = (t1 & t2).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_bor {
    ($o:expr,$a:expr,$b:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let t2: U256 = $b.try_into().unwrap();
        $o = (t1 | t2).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_bxor {
    ($o:expr,$a:expr,$b:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let t2: U256 = $b.try_into().unwrap();
        $o = (t1 ^ t2).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_bnot {
    ($o:expr,$a:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        $o = (!t1).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_eq {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a == $b).into();
    }};
}

macro_rules! Fr_neq {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a != $b).into();
    }};
}

macro_rules! Fr_lt {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a < $b).into();
    }};
}

macro_rules! Fr_leq {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a <= $b).into();
    }};
}

macro_rules! Fr_gt {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a > $b).into();
    }};
}

macro_rules! Fr_geq {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = ($a >= $b).into();
    }};
}

macro_rules! Fr_isTrue {
    ($a:expr) => {{
        $a.is_one()
    }};
}

macro_rules! Fr_land {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = (!($a * $b).is_zero()).into();
    }};
}

macro_rules! Fr_lor {
    ($o:expr,$a:expr,$b:expr) => {{
        $o = (!($a + $b).is_zero()).into();
    }};
}

macro_rules! Fr_copy {
    ($o:expr,$a:expr) => {{
        unsafe {
            *($o as *mut FieldElement) = $a;
        }
    }};
}

macro_rules! Fr_toInt {
    ($a:expr) => {{
        if (!$a.is_zero()) {
            Into::<BigUint>::into($a).to_u32_digits()[0] as usize
        } else {
            0
        }
    }};
}

//Fr_toInt

pub(crate) use Fr_add;
pub(crate) use Fr_band;
pub(crate) use Fr_bnot;
pub(crate) use Fr_bor;
pub(crate) use Fr_bxor;
pub(crate) use Fr_copy;
pub(crate) use Fr_div;
pub(crate) use Fr_eq;
pub(crate) use Fr_geq;
pub(crate) use Fr_gt;
pub(crate) use Fr_inv;
pub(crate) use Fr_isTrue;
pub(crate) use Fr_land;
pub(crate) use Fr_leq;
pub(crate) use Fr_lor;
pub(crate) use Fr_lt;
pub(crate) use Fr_mul;
pub(crate) use Fr_neg;
pub(crate) use Fr_neq;
pub(crate) use Fr_shl;
pub(crate) use Fr_shr;
pub(crate) use Fr_square;
pub(crate) use Fr_sub;
pub(crate) use Fr_toInt;

pub struct ComponentMemory {
    pub templateId: u32,
    pub templateName: String,
    pub componentName: String,
    pub signalStart: usize,
    pub inputCounter: u32,
    pub idFather: usize,
    pub subcomponents: Vec<usize>,
}

pub struct Context {
    pub componentMemory: Vec<ComponentMemory>,
    pub signalValues: Vec<FieldElement>,
    pub circuitConstants: Vec<FieldElement>,
}
