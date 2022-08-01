
mod macros;
use std::env;
use crate::macros::*;
use ark_bn254::Fr as F;
use ark_ff::Zero;
use ark_std::One;
use ark_ff::Field;
use num_bigint::BigUint;
use ruint::aliases::U256;
use std::str::FromStr;
use ark_ff::BigInteger256;
use ark_ff::FromBytes;
const _functionTable : [fn(usize, &mut Context);1] = [
Num2Bits_0_run ];
fn get_main_input_signal_start() -> usize {return 9;}
fn get_main_input_signal_no() -> usize {return 1;}
fn get_total_signal_no() -> usize {return 10;}
fn get_number_of_components() -> usize {return 1;}
fn get_size_of_input_hashmap() -> usize {return 256;}
fn get_size_of_witness() -> usize {return 9;}
fn get_size_of_constants() -> usize {return 4;}
fn get_size_of_io_map() -> usize {return 0;}
fn Num2Bits_0_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize){
ctx.componentMemory[coffset].templateId = 0;
ctx.componentMemory[coffset].templateName = "Num2Bits".to_string();
ctx.componentMemory[coffset].signalStart = soffset;
ctx.componentMemory[coffset].inputCounter = 1;
ctx.componentMemory[coffset].componentName = componentName;
ctx.componentMemory[coffset].idFather = componentFather;
ctx.componentMemory[coffset].subcomponents = vec![0;0];
}
fn Num2Bits_0_run(ctx_index: usize, ctx: &mut Context){
let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
let myFather = ctx.componentMemory[ctx_index].idFather;
let myId = ctx_index;
let mut expaux = vec![F::zero(); 7];
let mut lvar = vec![F::zero(); 3];
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[0]);
}
{
let aux_dest = &lvar[1] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[1]);
}
{
let aux_dest = &lvar[2] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[1]);
}
Fr_lt!(expaux[0],lvar[2],ctx.circuitConstants[0]); // line circom 8
while Fr_isTrue!(expaux[0]) {
{
let aux_dest = &ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[2])) + 0)] as *const FieldElement;
Fr_shr!(expaux[1],ctx.signalValues[mySignalStart + 8],lvar[2]); // line circom 9
Fr_band!(expaux[0],expaux[1],ctx.circuitConstants[2]); // line circom 9
Fr_copy!(aux_dest,expaux[0]);
}
Fr_sub!(expaux[3],ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[2])) + 0)],ctx.circuitConstants[2]); // line circom 10
Fr_mul!(expaux[1],ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[2])) + 0)],expaux[3]); // line circom 10
Fr_eq!(expaux[0],expaux[1],ctx.circuitConstants[1]); // line circom 10
{
let aux_dest = &lvar[1] as *const FieldElement;
Fr_pow!(expaux[4],ctx.circuitConstants[3],lvar[2]); // line circom 11
Fr_mul!(expaux[2],ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[2])) + 0)],expaux[4]); // line circom 11
Fr_add!(expaux[0],lvar[1],expaux[2]); // line circom 11
Fr_copy!(aux_dest,expaux[0]);
}
{
let aux_dest = &lvar[2] as *const FieldElement;
Fr_add!(expaux[0],lvar[2],ctx.circuitConstants[2]); // line circom 8
Fr_copy!(aux_dest,expaux[0]);
}
Fr_lt!(expaux[0],lvar[2],ctx.circuitConstants[0]); // line circom 8
}
Fr_eq!(expaux[0],lvar[1],ctx.signalValues[mySignalStart + 8]); // line circom 14
}
fn run(ctx: &mut Context){
Num2Bits_0_create(1,0,ctx,"main".to_string(),0);
Num2Bits_0_run(0,ctx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut signalValues = vec![F::zero(); get_total_signal_no() as usize];
    signalValues[0] = F::one(); // 1 by convention

    for (i, w) in args.into_iter().skip(1).enumerate() {
        signalValues[get_main_input_signal_start() + i] = F::from_str(&w).unwrap();
    }

    let mut circuitConstants = vec![F::zero(); get_size_of_constants() as usize];
    circuitConstants[0] = F::from(8);
circuitConstants[1] = F::from(0);
circuitConstants[2] = F::from(1);
circuitConstants[3] = F::from(2);


    let mut componentMemory = Vec::new();
    for _ in 0..get_number_of_components() {
        componentMemory.push(ComponentMemory {
            templateId: 0,
            templateName: "".to_string(),
            componentName: "".to_string(),
            signalStart: 0,
            inputCounter: 0,
            idFather: 0,
            subcomponents: vec![0; 0],
        });
    }

    let mut ctx = crate::Context {
        componentMemory,
        signalValues,
        circuitConstants,
        templateInsId2IOSignalInfo: get_btree(),
    };

    run(&mut ctx);

    for i in ctx.signalValues {
        let bi : BigUint = i.into();
        println!("{} {}", i, bi);
    }
}
