
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
const _functionTable : [fn(usize, &mut Context);2] = [
Ark_0_run,
Multiplier_1_run ];
fn get_main_input_signal_start() -> usize {return 11;}
fn get_main_input_signal_no() -> usize {return 2;}
fn get_total_signal_no() -> usize {return 33;}
fn get_number_of_components() -> usize {return 2;}
fn get_size_of_input_hashmap() -> usize {return 256;}
fn get_size_of_witness() -> usize {return 11;}
fn get_size_of_constants() -> usize {return 4;}
fn get_size_of_io_map() -> usize {return 0;}
fn Ark_0_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize){
ctx.componentMemory[coffset].templateId = 0;
ctx.componentMemory[coffset].templateName = "Ark".to_string();
ctx.componentMemory[coffset].signalStart = soffset;
ctx.componentMemory[coffset].inputCounter = 10;
ctx.componentMemory[coffset].componentName = componentName;
ctx.componentMemory[coffset].idFather = componentFather;
ctx.componentMemory[coffset].subcomponents = vec![0;0];
}
fn Ark_0_run(ctx_index: usize, ctx: &mut Context){
let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
let myFather = ctx.componentMemory[ctx_index].idFather;
let myId = ctx_index;
let mut expaux = vec![F::zero(); 3];
let mut lvar = vec![F::zero(); 2];
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[0]);
}
{
let aux_dest = &lvar[1] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[1]);
}
Fr_lt!(expaux[0],lvar[1],ctx.circuitConstants[0]); // line circom 7
while Fr_isTrue!(expaux[0]) {
{
let aux_dest = &ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[1])) + 0)] as *const FieldElement;
Fr_mul!(expaux[0],ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[1])) + 10)],ctx.circuitConstants[2]); // line circom 8
Fr_copy!(aux_dest,expaux[0]);
}
{
let aux_dest = &lvar[1] as *const FieldElement;
Fr_add!(expaux[0],lvar[1],ctx.circuitConstants[3]); // line circom 7
Fr_copy!(aux_dest,expaux[0]);
}
Fr_lt!(expaux[0],lvar[1],ctx.circuitConstants[0]); // line circom 7
}
}
fn Multiplier_1_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize){
ctx.componentMemory[coffset].templateId = 1;
ctx.componentMemory[coffset].templateName = "Multiplier".to_string();
ctx.componentMemory[coffset].signalStart = soffset;
ctx.componentMemory[coffset].inputCounter = 2;
ctx.componentMemory[coffset].componentName = componentName;
ctx.componentMemory[coffset].idFather = componentFather;
ctx.componentMemory[coffset].subcomponents = vec![0;1];
}
fn Multiplier_1_run(ctx_index: usize, ctx: &mut Context){
let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
let myFather = ctx.componentMemory[ctx_index].idFather;
let myId = ctx_index;
let mut expaux = vec![F::zero(); 3];
let mut lvar = vec![F::zero(); 1];
{
let aux_create = 0;
let mut aux_cmp_num = 0+ctx_index+1;
let mut csoffset = mySignalStart+12;
for i in  0..1 {
let new_cmp_name = "ark".to_string();
ctx.componentMemory[ctx_index].subcomponents[aux_create+i] = aux_cmp_num;
Ark_0_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
csoffset += 20 ;
aux_cmp_num += 1;
}
}
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[1]);
}
Fr_lt!(expaux[0],lvar[0],ctx.circuitConstants[0]); // line circom 20
while Fr_isTrue!(expaux[0]) {
{
let cmp_index_ref = 0;
{
let aux_dest = &ctx.signalValues[ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt!(lvar[0])) + 10)] as *const FieldElement;
Fr_copy!(aux_dest,ctx.signalValues[mySignalStart + 10]);
}
if !(ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref]].inputCounter)==1 {
Ark_0_run(ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref],ctx);
}
}
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_add!(expaux[0],lvar[0],ctx.circuitConstants[3]); // line circom 20
Fr_copy!(aux_dest,expaux[0]);
}
Fr_lt!(expaux[0],lvar[0],ctx.circuitConstants[0]); // line circom 20
}
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[1]);
}
Fr_lt!(expaux[0],lvar[0],ctx.circuitConstants[0]); // line circom 24
while Fr_isTrue!(expaux[0]) {
{
let aux_dest = &ctx.signalValues[mySignalStart + ((1 * Fr_toInt!(lvar[0])) + 0)] as *const FieldElement;
Fr_copy!(aux_dest,ctx.signalValues[ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[0]].signalStart + ((1 * Fr_toInt!(lvar[0])) + 0)]);
}
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_add!(expaux[0],lvar[0],ctx.circuitConstants[3]); // line circom 24
Fr_copy!(aux_dest,expaux[0]);
}
Fr_lt!(expaux[0],lvar[0],ctx.circuitConstants[0]); // line circom 24
}
}
fn run(ctx: &mut Context){
Multiplier_1_create(1,0,ctx,"main".to_string(),0);
Multiplier_1_run(0,ctx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut signalValues = vec![F::zero(); get_total_signal_no() as usize];
    signalValues[0] = F::one(); // 1 by convention

    for (i, w) in args.into_iter().skip(1).enumerate() {
        signalValues[get_main_input_signal_start() + i] = F::from_str(&w).unwrap();
    }

    let mut circuitConstants = vec![F::zero(); get_size_of_constants() as usize];
    circuitConstants[0] = F::from(10);
circuitConstants[1] = F::from(0);
circuitConstants[2] = F::from(2);
circuitConstants[3] = F::from(1);


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
