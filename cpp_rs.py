import struct
import re
import os
import sys

CIRCUIT_PATH = sys.argv[1]
CIRCUIT_NAME = sys.argv[2]

OUT_FILE = "src/main.rs"

GETTERS = {}
HEADER = """
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
"""
FOOTER = """
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut signalValues = vec![F::zero(); get_total_signal_no() as usize];
    signalValues[0] = F::one(); // 1 by convention

    for (i, w) in args.into_iter().skip(1).enumerate() {
        signalValues[2 + i] = F::from_str(&w).unwrap();
    }

    let mut circuitConstants = vec![F::zero(); get_size_of_constants() as usize];
    // @TRANSPILER_CONSTANTS

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
"""

def interpret(line):
    line = line.replace("->", ".")
        
    # skip comments
    if line.startswith("//") or line.startswith("assert(") or "std::cout <<" in line:
        return ""

    if line.startswith("uint get_"):
        line = line.replace("uint", "fn")
        line = line.replace("()", "() -> usize")
        val = re.findall(r'\d+', line)[0]
        GETTERS[line.split()[1]] = int(val)
    
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

    if "_create(" in line:
        line = line.replace("\",", "\".to_string(),")

    # replace run function 
    line = line.replace("void run(Circom_CalcWit* ctx){", "fn run(ctx: &mut Context){")

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
    

fout = open(OUT_FILE, "w")
fout.write(HEADER)

with open(f"{CIRCUIT_PATH}/{CIRCUIT_NAME}_cpp/{CIRCUIT_NAME}.cpp") as fin:
    start = False
    for line in fin:
        line = line.rstrip()
        
        # ignore everything until here
        if "uint get_main_input_signal_start()" in line:
            start = True
            
        if not start:
            continue
            
        line = interpret(line)
        if line == "":
            continue
            
        fout.write(line + "\n")
        
# modify footer
with open(f"{CIRCUIT_PATH}/{CIRCUIT_NAME}_cpp/{CIRCUIT_NAME}.dat", "rb") as f:
    f.read(GETTERS["get_size_of_input_hashmap()"]*24) # ignore
    f.read(GETTERS["get_size_of_witness()"]*8)  # ignore
    
    # we're only here for the constants:
    constants_code = ""
    for i in range(0,GETTERS["get_size_of_constants()"]):
        sv, typ = struct.unpack('<iI', bytearray(f.read(8)))
        long_bytes = list(bytearray(f.read(32)))
        is_long = bool(typ & 0x80000000)
        # print(f"is_long = {is_long}, short_val = {sv}, type = {typ}, long_val = {long_bytes}")
        if not is_long:
            constants_code += f"circuitConstants[{i}] = F::from({sv});\n"
        else:
            constants_code += f"circuitConstants[{i}] = F::new(BigInteger256::read(&({long_bytes} as [u8; 32])[..]).unwrap());\n"

FOOTER = FOOTER.replace("// @TRANSPILER_CONSTANTS", constants_code)

fout.write(FOOTER)
fout.close()

os.system(f"cargo fmt -- {OUT_FILE}")