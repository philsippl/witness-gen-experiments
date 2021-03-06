mod macros;
use crate::macros::*;
use ark_bn254::Fr as F;
use ark_ff::BigInteger256;
use ark_ff::Field;
use ark_ff::FromBytes;
use ark_ff::Zero;
use ark_std::One;
use num_bigint::BigUint;
use ruint::aliases::U256;
use std::env;
use std::str::FromStr;
fn get_main_input_signal_start() -> usize {
    return 2;
}
fn get_main_input_signal_no() -> usize {
    return 2;
}
fn get_total_signal_no() -> usize {
    return 4;
}
fn get_number_of_components() -> usize {
    return 1;
}
fn get_size_of_input_hashmap() -> usize {
    return 256;
}
fn get_size_of_witness() -> usize {
    return 4;
}
fn get_size_of_constants() -> usize {
    return 4;
}
fn get_size_of_io_map() -> usize {
    return 0;
}
fn Multiplier_0_create(
    soffset: usize,
    coffset: usize,
    ctx: &mut Context,
    componentName: String,
    componentFather: usize,
) {
    ctx.componentMemory[coffset].templateId = 0;
    ctx.componentMemory[coffset].templateName = "Multiplier".to_string();
    ctx.componentMemory[coffset].signalStart = soffset;
    ctx.componentMemory[coffset].inputCounter = 2;
    ctx.componentMemory[coffset].componentName = componentName;
    ctx.componentMemory[coffset].idFather = componentFather;
    ctx.componentMemory[coffset].subcomponents = vec![0; 0];
}
fn Multiplier_0_run(ctx_index: usize, ctx: &mut Context) {
    let mySignalStart = ctx.componentMemory[ctx_index].signalStart;
    let myTemplateName = ctx.componentMemory[ctx_index].templateName.clone();
    let myComponentName = ctx.componentMemory[ctx_index].componentName.clone();
    let myFather = ctx.componentMemory[ctx_index].idFather;
    let myId = ctx_index;
    let mut expaux = vec![F::zero(); 7];
    let mut lvar = vec![F::zero(); 0];
    {
        let aux_dest = &ctx.signalValues[mySignalStart + 0] as *const FieldElement;
        Fr_mul!(
            expaux[4],
            ctx.signalValues[mySignalStart + 1],
            ctx.signalValues[mySignalStart + 2]
        ); // line circom 8
        Fr_add!(expaux[3], expaux[4], ctx.circuitConstants[0]); // line circom 8
        Fr_add!(expaux[2], expaux[3], ctx.circuitConstants[1]); // line circom 8
        Fr_add!(expaux[1], expaux[2], ctx.circuitConstants[2]); // line circom 8
        Fr_add!(expaux[0], expaux[1], ctx.circuitConstants[3]); // line circom 8
        Fr_copy!(aux_dest, expaux[0]);
    }
}
fn run(ctx: &mut Context) {
    Multiplier_0_create(1, 0, ctx, "main".to_string(), 0);
    Multiplier_0_run(0, ctx);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut signalValues = vec![F::zero(); get_total_signal_no() as usize];
    signalValues[0] = F::one(); // 1 by convention

    for (i, w) in args.into_iter().skip(1).enumerate() {
        signalValues[2 + i] = F::from_str(&w).unwrap();
    }

    let mut circuitConstants = vec![F::zero(); get_size_of_constants() as usize];
    circuitConstants[0] = F::new(
        BigInteger256::read(
            &([
                78, 251, 199, 166, 68, 81, 80, 228, 188, 81, 41, 162, 85, 151, 51, 120, 57, 48, 34,
                199, 214, 52, 146, 35, 235, 131, 66, 75, 114, 91, 172, 9,
            ] as [u8; 32])[..],
        )
        .unwrap(),
    );
    circuitConstants[1] = F::from(1337);
    circuitConstants[2] = F::from(129);
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
    };

    run(&mut ctx);

    for i in ctx.signalValues {
        println!("{}", i);
    }
}
