use ruint::{aliases::U256, uint};
use ark_ff::{PrimeField, FpParameters, BigInteger, Field, Fp256};
use ark_bn254::{Fq as F, FqParameters};
use ark_std::{One, Zero, UniformRand};

pub type Fr = U256;
pub type FieldElement = Fp256<FqParameters>;

pub const MODULUS: Fr =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);


macro_rules! Fr_mul{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = $a * $b
        }
    }
}

macro_rules! Fr_add{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = $a + $b
        }
    }
}

macro_rules! Fr_sub{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = $a - $b
        }
    }
}

macro_rules! Fr_neg{
    ($o:expr,$a:expr)=>{
        {
            $o = -$a
        }
    }
}

macro_rules! Fr_inv{
    ($o:expr,$a:expr)=>{
        {
            $o = $a.inverse().unwrap();
        }
    }
}

macro_rules! Fr_square{
    ($o:expr,$a:expr)=>{
        {
            $o = $a * $a
        }
    }
}

macro_rules! Fr_shl{
    ($o:expr,$a:expr,$n:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            $o = (t1 << $n).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_shr{
    ($o:expr,$a:expr,$n:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            $o = (t1 >> $n).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_band{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            let t2: U256 = $b.try_into().unwrap();
            $o = (t1 & t2).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_bor{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            let t2: U256 = $b.try_into().unwrap();
            $o = (t1 | t2).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_bxor{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            let t2: U256 = $b.try_into().unwrap();
            $o = (t1 ^ t2).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_bnot{
    ($o:expr,$a:expr)=>{
        {
            let t1: U256 = $a.try_into().unwrap();
            $o = (!t1).reduce_mod(MODULUS).try_into().unwrap();
        }
    }
}

macro_rules! Fr_eq{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a == $b);
        }
    }
}

macro_rules! Fr_neq{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a != $b);
        }
    }
}

macro_rules! Fr_lt{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a < $b);
        }
    }
}

macro_rules! Fr_leq{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a <= $b);
        }
    }
}

macro_rules! Fr_gt{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a > $b);
        }
    }
}

macro_rules! Fr_geq{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = ($a >= $b);
        }
    }
}

macro_rules! Fr_isTrue{
    ($o:expr,$a:expr)=>{
        {
            $o = $a.is_one().into();
        }
    }
}

macro_rules! Fr_land{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = (!($a * $b).is_zero()).into();
        }
    }
}

macro_rules! Fr_lor{
    ($o:expr,$a:expr,$b:expr)=>{
        {
            $o = (!($a + $b).is_zero()).into();
        }
    }
}

macro_rules! Fr_copy{
    ($o:expr,$a:expr)=>{
        {
            $o = $a;
        }
    }
}

struct ComponentMemory {
    pub templateId: u32,
    pub templateName: String,
    pub componentName: String,
    pub signalStart: usize,
    pub inputCounter: u32, 
    pub idFather: usize,
    pub subcomponents: Vec<u32>,
}

struct Context {
    pub componentMemory: Vec<ComponentMemory>,
    pub signalValues: Vec<FieldElement>,
    pub circuitConstants: Vec<FieldElement>,
}

fn main() {

    let x1 = uint!(1_u32);
    let x2 = uint!(10_u32);
    
    let y = x1 & x2;
    dbg!(y);

    unsafe {
    let witness_size = 6;
    let mut signal_values = [F::one()*F::from(5_u32); 6]; 
    let mut expaux = [F::zero(); 5];

    let aux_dest = &mut signal_values[0];
    Fr_mul!(expaux[0], signal_values[1], signal_values[2]);
    *aux_dest = expaux[0];
    dbg!(signal_values);

    }



}

fn Multiplier_1_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize) {
    ctx.componentMemory[coffset].templateId = 1;
    ctx.componentMemory[coffset].templateName = "Multiplier".to_string();
    ctx.componentMemory[coffset].signalStart = soffset;
    ctx.componentMemory[coffset].inputCounter = 2;
    ctx.componentMemory[coffset].componentName = componentName;
    ctx.componentMemory[coffset].idFather = componentFather;
    ctx.componentMemory[coffset].subcomponents = [2].to_vec();
}

fn Multiplier_0_run(ctx_index: usize, ctx: &Context) {
    let signalValues = &ctx.signalValues;
    let mySignalStart = &ctx.componentMemory[ctx_index].signalStart;
    let myTemplateName = &ctx.componentMemory[ctx_index].templateName;
    let myComponentName = &ctx.componentMemory[ctx_index].componentName;
    let myFather = &ctx.componentMemory[ctx_index].idFather;
    let myId = ctx_index;
    let mySubcomponents = &ctx.componentMemory[ctx_index].subcomponents;
    let circuitConstants = &ctx.circuitConstants;
    let mut expaux = [F::zero(); 3];
    let mut lvar = [F::zero(); 0];
    let sub_component_aux: u32;
    
    // let aux_dest = &mut signalValues[mySignalStart + 0];
    // Fr_mul!(expaux[0], signalValues[mySignalStart + 1], signalValues[mySignalStart + 2]);
    // *aux_dest = expaux[0];
}

// PFrElement aux_dest = &signalValues[mySignalStart + 0];
// // load src
// Fr_mul(&expaux[0],&signalValues[mySignalStart + 1],&signalValues[mySignalStart + 2]); // line circom 8
// // end load src
// Fr_copy(aux_dest,&expaux[0]);
// }
// }