mod macros;
use crate::macros::*;
use ark_bn254::Fq as F;
use ark_ff::Field;
use ark_ff::Zero;
use ark_std::One;
use num_bigint::BigUint;
use ruint::aliases::U256;
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
    let mut expaux = vec![F::zero(); 5];
    let mut lvar = vec![F::zero(); 1];
    {
        let aux_dest = &lvar[0] as *const FieldElement;
        Fr_copy!(aux_dest, ctx.circuitConstants[0]);
    }
    {
        let aux_dest = &ctx.signalValues[mySignalStart + 3] as *const FieldElement;

        Fr_sub!(
            expaux[2],
            ctx.signalValues[mySignalStart + 1],
            ctx.circuitConstants[1]
        ); // line circom 32
        Fr_div!(expaux[0], ctx.circuitConstants[1], expaux[2]); // line circom 32
        Fr_copy!(aux_dest, expaux[0]);
    }

    // for &i in &ctx.signalValues {
    //     println!("> {}", i);
    // }
    // println!("-------");

    Fr_sub!(
        expaux[2],
        ctx.signalValues[mySignalStart + 1],
        ctx.circuitConstants[1]
    ); // line circom 33
    Fr_mul!(expaux[1], expaux[2], ctx.signalValues[mySignalStart + 3]); // line circom 33
    Fr_eq!(expaux[0], expaux[1], ctx.circuitConstants[1]); // line circom 33

    {
        let aux_dest = &ctx.signalValues[mySignalStart + 4] as *const FieldElement;
        Fr_sub!(
            expaux[2],
            ctx.signalValues[mySignalStart + 2],
            ctx.circuitConstants[1]
        ); // line circom 35

        Fr_div!(expaux[0], ctx.circuitConstants[1], expaux[2]); // line circom 35
        Fr_copy!(aux_dest, expaux[0]);
    }
    Fr_sub!(
        expaux[2],
        ctx.signalValues[mySignalStart + 2],
        ctx.circuitConstants[1]
    ); // line circom 36
    Fr_mul!(expaux[1], expaux[2], ctx.signalValues[mySignalStart + 4]); // line circom 36
    Fr_eq!(expaux[0], expaux[1], ctx.circuitConstants[1]); // line circom 36
    {
        let aux_dest = &ctx.signalValues[mySignalStart + 0] as *const FieldElement;
        Fr_mul!(
            expaux[0],
            ctx.signalValues[mySignalStart + 1],
            ctx.signalValues[mySignalStart + 2]
        ); // line circom 38
        Fr_copy!(aux_dest, expaux[0]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ComponentMemory, Multiplier_0_create, Multiplier_0_run};
    use ark_bn254::Fq as F;
    use ark_ff::{Field, One, PrimeField, Zero};

    #[test]
    fn xxx_test() {
        let n = F::from(1);

        let mut signalValues = vec![F::zero(); 140];

        signalValues[0] = F::one();
        signalValues[2] = F::from(7);
        signalValues[3] = F::from(9);

        let circuitConstants = vec![F::from(64), F::from(1)];

        let mut componentMemory = Vec::new();
        for _ in 0..10 {
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

        Multiplier_0_create(1, 0, &mut ctx, "main".to_string(), 0);
        Multiplier_0_run(0, &mut ctx);

        println!("{}", F::from(2).inverse().unwrap() * F::from(2));
        for i in ctx.signalValues {
            println!("{}", i);
        }

        // let x = F::from(9) - F::from(1);
        // let y = F::from(8).inverse().unwrap();

        let x = F::from(2);

        println!("{}", x.inverse().unwrap() * F::from(1));
    }
}

fn main() {}
