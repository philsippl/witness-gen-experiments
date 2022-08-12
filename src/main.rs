
mod macros;
use std::env;
use crate::macros::*;
use num_bigint::BigUint;
use ruint::Uint;
use ruint::uint;
use std::str::FromStr;
use ark_bn254::Fr as F;
use ark_ff::BigInteger256;
use ark_ff::FromBytes;
use ruint::aliases::U64;
const _functionTable : [fn(usize, &mut Context);2] = [
Test_0_run,
Semaphore_1_run ];
fn get_main_input_signal_start() -> usize {return 2;}
fn get_main_input_signal_no() -> usize {return 1;}
fn get_total_signal_no() -> usize {return 5;}
fn get_number_of_components() -> usize {return 2;}
fn get_size_of_input_hashmap() -> usize {return 256;}
fn get_size_of_witness() -> usize {return 2;}
fn get_size_of_constants() -> usize {return 4;}
fn get_size_of_io_map() -> usize {return 0;}
fn Test_0_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize){
ctx.componentMemory[coffset].templateId = 0;
ctx.componentMemory[coffset].templateName = "Test".to_string();
ctx.componentMemory[coffset].signalStart = soffset;
ctx.componentMemory[coffset].inputCounter = 1;
ctx.componentMemory[coffset].componentName = componentName;
ctx.componentMemory[coffset].idFather = componentFather;
ctx.componentMemory[coffset].subcomponents = vec![0;0];
}
fn Test_0_run(ctx_index: usize, ctx: &mut Context){
let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
let myFather = ctx.componentMemory[ctx_index].idFather;
let myId = ctx_index;
let mut expaux = vec![uint!(0_U256); 4];
let mut lvar = vec![uint!(0_U256); 0];
{
let aux_dest = &ctx.signalValues[mySignalStart + 0] as *const FieldElement;
Fr_mul!(expaux[1],ctx.signalValues[mySignalStart + 1],ctx.circuitConstants[0]); // line circom 7
Fr_add!(expaux[0],expaux[1],ctx.circuitConstants[1]); // line circom 7
Fr_copy!(aux_dest,expaux[0]);
}
}
fn Semaphore_1_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize){
ctx.componentMemory[coffset].templateId = 1;
ctx.componentMemory[coffset].templateName = "Semaphore".to_string();
ctx.componentMemory[coffset].signalStart = soffset;
ctx.componentMemory[coffset].inputCounter = 1;
ctx.componentMemory[coffset].componentName = componentName;
ctx.componentMemory[coffset].idFather = componentFather;
ctx.componentMemory[coffset].subcomponents = vec![0;5];
}
fn Semaphore_1_run(ctx_index: usize, ctx: &mut Context){
let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
let myFather = ctx.componentMemory[ctx_index].idFather;
let myId = ctx_index;
let mut expaux = vec![uint!(0_U256); 2];
let mut lvar = vec![uint!(0_U256); 1];
{
let aux_dest = &lvar[0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[2]);
}
{
let aux_create = 0;
let mut aux_cmp_num = 0+ctx_index+1;
let mut csoffset = mySignalStart+2;
let aux_dimensions: Vec<usize> = vec![5];
let aux_positions: Vec<usize>= vec![0];
for i_aux in  0..1 {
let i = aux_positions[i_aux];
let new_cmp_name = "mims".to_owned()+&ctx.generate_position_array(&aux_dimensions, 1, i);
ctx.componentMemory[ctx_index].subcomponents[aux_create+i] = aux_cmp_num;
Test_0_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
csoffset += 2 ;
aux_cmp_num += 1;
}
}
{
let cmp_index_ref = 0;
{
let aux_dest = &ctx.signalValues[ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref]].signalStart + 1] as *const FieldElement;
Fr_copy!(aux_dest,ctx.signalValues[mySignalStart + 1]);
}

let idx=ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref];
ctx.componentMemory[idx].inputCounter -= 1;
if !((ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref]].inputCounter)==1){
Test_0_run(ctx.componentMemory[ctx_index].subcomponents[cmp_index_ref],ctx);
}
}
{
let aux_dest = &ctx.signalValues[mySignalStart + 0] as *const FieldElement;
Fr_copy!(aux_dest,ctx.signalValues[ctx.componentMemory[ctx.componentMemory[ctx_index].subcomponents[0]].signalStart + 0]);
}
}
fn run(ctx: &mut Context){
Semaphore_1_create(1,0,ctx,"main".to_string(),0);
Semaphore_1_run(0,ctx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut signalValues = vec![uint!(0_U256); get_total_signal_no() as usize];
    signalValues[0] = uint!(1_U256); // 1 by convention

    for (i, w) in args.into_iter().skip(1).enumerate() {
        signalValues[get_main_input_signal_start() + i] = Uint::from_str(&w).unwrap();
    }

    let circuitConstants = get_constants();

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
