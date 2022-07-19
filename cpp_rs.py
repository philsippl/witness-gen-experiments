header = """
mod macros;
use crate::macros::*;
use ark_bn254::Fq as F;
use ark_ff::Zero;
use ark_std::One;
use ark_ff::Field;
use num_bigint::BigUint;
use ruint::aliases::U256;
"""

footer = """
#[cfg(test)]
mod tests {
    use crate::{ComponentMemory, Multiplier_1_create, Multiplier_1_run};
    use ark_bn254::Fq as F;
    use ark_ff::{One, Zero, Field};

    #[test]
    fn xxx_test() {
        let n = F::from(1);

        let mut signalValues = vec![F::zero(); 140];

        signalValues[0] = F::one();
        signalValues[2] = F::from(2);
        signalValues[3] = F::from(3);

        let circuitConstants = vec![F::from(64), F::from(0), F::from(1)];

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

        Multiplier_1_create(1, 0, &mut ctx, "main".to_string(), 0);
        Multiplier_1_run(0, &mut ctx);

        println!("{}", F::from(2).inverse().unwrap() * F::from(2));
        for i in ctx.signalValues {
            println!("{}", i);
        }
    }
}

fn main() {}

"""

def interpret(line):
    line = line.replace("->", ".")
        
    # skip comments
    if line.startswith("//") or line.startswith("assert(") or "std::cout <<" in line:
        return ""
    
    # replace create
    template = "_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather)"
    if template in line:
        line = line.replace(template, "_create(soffset: usize, coffset: usize, ctx: &mut Context, componentName: String, componentFather: usize)") 
        line = line.replace("void", "fn")
    
    # replace run
    template = "_run(uint ctx_index,Circom_CalcWit* ctx)"
    if template in line:
        line = line.replace(template, "_run(ctx_index: usize, ctx: &mut Context)") 
        line = line.replace("void", "fn")
    
    if ("templateName" in line or "new_cmp_name" in line) and "\";" in line:
        line = line.replace("\";", "\".to_string();")
    
    if "new uint[" in line:
        line = line.replace("new uint[", "vec![0;")
    
    if line.startswith("FrElement*") or line.startswith("u64") or line.startswith("std::string") or line.startswith("uint") or line.startswith("u32*") or line.startswith("std::string*") or line.startswith("FrElement") or line.startswith("int"):
        line = "let " + line.split(" ", 1)[1]  
        
    if line.startswith("for (uint i ="):
        line = line.replace("for (uint i =", "for i in ")
        line = line.replace("; i < ", "..")
        line = line.replace("; i++)", "")
        
    if line.startswith("PFrElement aux_dest"):
        line = line.replace("PFrElement", "let")
        line = line.replace(";", " as *const FieldElement;")
        
    if line.startswith("Fr_"):
        line = line.replace("&", "")
        line = line.replace("(", "!(", 1)
        
    if "let aux_cmp_num" in line:
        line = line.replace("aux_cmp_num", "mut aux_cmp_num")
    
    if "let csoffset" in line:
        line = line.replace("csoffset", "mut csoffset")
        
    if line.startswith("let signalValues = ctx.signalValues;"):
        line = ""
        
    if line.startswith("let mySubcomponents = ctx.componentMemory[ctx_index].subcomponents;"):
        line = ""
        
    if line.startswith("let circuitConstants = ctx.circuitConstants;"):
        line = ""
        
    if line.startswith("let expaux"):
        line = line.replace("let expaux", "let mut expaux")
        line = line.replace("[", " = vec![F::zero(); ")
        
    if line.startswith("let lvar"):
        line = line.replace("let lvar", "let mut lvar")
        line = line.replace("[", " = vec![F::zero(); ")
        
    if line.startswith("while("):
        line = line.replace("while(", "while ")
        line = line.replace("){", " {")
        
    line = line.replace("ctx.signalValues", "CTX_SIGNALVALUES")
    line = line.replace("signalValues", "ctx.signalValues")
    line = line.replace("CTX_SIGNALVALUES", "ctx.signalValues")
    
    line = line.replace("mySubcomponents", "ctx.componentMemory[ctx_index].subcomponents")
    line = line.replace("circuitConstants", "ctx.circuitConstants")
    
    line = line.replace("ctx.componentMemory[ctx_index].templateName", "ctx.componentMemory[ctx_index].templateName.clone()")
    line = line.replace("ctx.componentMemory[ctx_index].componentName", "ctx.componentMemory[ctx_index].componentName.clone()")
    
    line = line.replace("Fr_isTrue(&", "Fr_isTrue!(")
    line = line.replace("Fr_toInt(&", "Fr_toInt!(")
    line = line.replace("Fr_toInt(", "Fr_toInt!(")
    
    # TODO
    if "let sub_component_aux;" in line:
        line = ""
        
    if line.startswith("let listOfTemplateMessages"):
        line = ""

    return line
    

fout = open(f"src/abc.rs", "w")

fout.write(header)

with open(f"circuit/test_cpp/test.cpp") as fin:
    start = False
    for line in fin:
        line = line.rstrip()
        
        # ignore everything until here
        if line == "// function declarations":
            start = True
            
        if not start:
            continue
            
        line = interpret(line)
        if line == "":
            continue
            
        fout.write(line + "\n")
        
fout.write(footer)
fout.close()

# TODO: run formatter on generated code