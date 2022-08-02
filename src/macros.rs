use ark_bn254::{Fq as F, FqParameters, FrParameters};
use ark_ff::{BigInteger, Field, Fp256, FpParameters, PrimeField};
use ark_std::{One, UniformRand, Zero};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};
use num_bigint::BigUint;
use ruint::{aliases::U256, uint};
use std::collections::BTreeMap;
use core::include_bytes;

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

macro_rules! Fr_copyn {
    ($o:expr,$a:expr, $n:expr) => {{
        unsafe {
            for i in 0..$n {
                *(($o as *mut FieldElement).add(i)) =
                    *((($a as *const FieldElement) as *mut FieldElement).add(i));
            }
        }
    }};
}

macro_rules! Fr_toInt {
    ($a:expr) => {{
        if (!$a.is_zero()) {
            let t1: U256 = $a.try_into().unwrap();
            t1.as_limbs()[0] as usize
        } else {
            0
        }
    }};
}

macro_rules! Fr_mod {
    ($o:expr,$a:expr,$n:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let t2: U256 = $n.try_into().unwrap();
        $o = t1.reduce_mod(t2).try_into().unwrap();
    }};
}

macro_rules! Fr_pow {
    ($o:expr,$a:expr,$n:expr) => {{
        let n = Fr_toInt!($n);
        let t1: U256 = $a.try_into().unwrap();
        $o = t1.pow(n).reduce_mod(MODULUS).try_into().unwrap();
    }};
}

macro_rules! Fr_idiv {
    ($o:expr,$a:expr,$b:expr) => {{
        let t1: U256 = $a.try_into().unwrap();
        let t2: U256 = $b.try_into().unwrap();
        $o = (t1  / t2).try_into().unwrap();
    }};
}

macro_rules! X_dec {
    ($o:expr) => {{
        $o -= 1;
        $o
    }};
}

//TODO: Fr_mod, Fr_idiv, Fr_pow

pub(crate) use Fr_add;
pub(crate) use Fr_band;
pub(crate) use Fr_bnot;
pub(crate) use Fr_bor;
pub(crate) use Fr_bxor;
pub(crate) use Fr_copy;
pub(crate) use Fr_copyn;
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
pub(crate) use Fr_mod;
pub(crate) use Fr_pow;
pub(crate) use Fr_idiv;
pub(crate) use X_dec;

use crate::{get_size_of_input_hashmap, get_size_of_io_map};

pub struct ComponentMemory {
    pub templateId: usize,
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
    pub templateInsId2IOSignalInfo: TemplateInstanceIOMap,
}

impl Context {
    pub fn generate_position_array(
        &self,
        dimensions: &Vec<usize>,
        size_dimensions: usize,
        index: usize,
    ) -> String {
        let mut positions: String = "".to_string();
        let mut index = index;
        let x = 1u32 != 0;
        for i in 0..size_dimensions {
            let last_pos = index % dimensions[size_dimensions - 1 - i];
            index /= dimensions[size_dimensions - 1 - i];
            let new_pos = format!("[{}]", last_pos);
            positions = new_pos + &positions;
        }
        positions
    }
}

/////////////////
#[derive(Debug)]
pub struct IODef {
    pub code: usize,
    pub offset: usize,
    pub lengths: Vec<usize>,
}

pub type InputOutputList = Vec<IODef>;
pub type TemplateInstanceIOMap = BTreeMap<usize, InputOutputList>;

const DAT_BYTES: &[u8] = include_bytes!("tmp.dat");

pub fn get_btree() -> TemplateInstanceIOMap {
    let mut bytes = DAT_BYTES;
    let mut indices = vec![0usize; get_size_of_io_map()];
    let mut btree: TemplateInstanceIOMap = BTreeMap::new();

    (0..get_size_of_io_map()).for_each(|i| {
        let t32 = bytes.read_u32::<LittleEndian>().unwrap() as usize;
        indices[i] = t32;
    });
    
    (0..get_size_of_io_map()).for_each(|i| {
        let l32 = bytes.read_u32::<LittleEndian>().unwrap() as usize;
        let mut io_list: InputOutputList = vec![];

        (0..l32).for_each(|_j| {
            let offset = bytes.read_u32::<LittleEndian>().unwrap() as usize;
            let len = bytes.read_u32::<LittleEndian>().unwrap() as usize + 1;
    
            let mut lengths = vec![0usize; len];

            (1..len).for_each(|k| {
                lengths[k] = bytes.read_u32::<LittleEndian>().unwrap() as usize;
            });
            
            io_list.push(IODef {
                code: 0,
                offset,
                lengths,
            });
        });
        btree.insert(indices[i], io_list);
    });
    btree
}