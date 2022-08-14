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
use num_bigint::BigUint;
use ruint::Uint;
use ruint::uint;
use std::str::FromStr;
use ark_bn254::Fr as F;
use ark_ff::BigInteger256;
use ark_ff::FromBytes;
use ruint::aliases::U64;
"""
FOOTER = """
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

    if line.startswith("Circom_TemplateFunction _functionTable["):
        line = line.replace("Circom_TemplateFunction _functionTable[", "const _functionTable : [fn(usize, &mut Context);")
        line = line.replace("{", "[")

    if line.endswith("_run };"):
        line = line.replace("_run };", "_run ];")
    
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

    template = "(Circom_CalcWit* ctx,FrElement* lvar,uint componentFather,FrElement* destination,int destination_size)"
    if template in line:
        line = line.replace(template, "(ctx: &mut Context, lvar: &mut Vec<FieldElement>, componentFather: usize, destination: *const FieldElement, destination_size: usize)") 
        line = line.replace("void", "fn")

    if line.startswith("Fr_copy(&lvarcall"):
        line = line.replace("Fr_copy(&", "")
        line = line.replace(",&", " = ")
        line = line.replace(");", ";")

    if ("templateName" in line or "new_cmp_name" in line) and "\";" in line:
        line = line.replace("\";", "\".to_string();")
    
    if "new uint[" in line:
        line = line.replace("new uint[", "vec![0;")

    if line.startswith("FrElement*") or line.startswith("u64") or line.startswith("std::string") or line.startswith("uint") or line.startswith("u32*") or line.startswith("std::string*") or line.startswith("FrElement") or line.startswith("int"):
        line = "let " + line.split(" ", 1)[1]  

    if line.startswith("let aux_dimensions["):
        val = re.findall(r'\d+', line)[0]
        line = line.replace(f"[{val}]", ": Vec<usize>")
        line = line.replace("{", "vec![")
        line = line.replace("};", "];")

    if line.startswith("let map_index_aux["):
        val = re.findall(r'\d+', line)[0]
        if "{" in line:
            line = line.replace(f"[{val}]", ": Vec<usize>")
            line = line.replace("{", "vec![")
            line = line.replace("};", "];")
        else: 
            line = line.replace("let map_index_aux[", "let mut map_index_aux[")
            line = line.replace(f"[{val}]", f" = vec![0usize;{val}]")

    if line.startswith("let aux_positions ["):
        val = re.findall(r'\d+', line)[0]
        line = line.replace(f" [{val}]", ": Vec<usize>")
        line = line.replace("{", "vec![")
        line = line.replace("};", "];")
        
    if line.startswith("for (uint i ="):
        line = line.replace("for (uint i =", "for i in ")
        line = line.replace("; i < ", "..")
        line = line.replace("; i++)", "")

    if line.startswith("for (uint i_aux ="):
        line = line.replace("for (uint i_aux =", "for i_aux in ")
        line = line.replace("; i_aux < ", "..")
        line = line.replace("; i_aux++)", "")
        
    if line.startswith("PFrElement aux_dest"):
        line = line.replace("PFrElement", "let")
        line = line.replace(";", " as *const FieldElement;")
        
    if line.startswith("Fr_"):
        if not line.startswith("Fr_copyn"):
            line = line.replace("&", "")
        else:
            line = line.replace("(&", "(&mut ")

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
        line = line.replace("[", " = vec![uint!(0_U256); ")
        
    if line.startswith("let lvar"):
        line = line.replace("let lvar", "let mut lvar")
        line = line.replace("[", " = vec![uint!(0_U256); ")
        
    if line.startswith("while("):
        line = line.replace("while(", "while ")
        line = line.replace("){", " {")

    if line.startswith("if("):
        line = line.replace("if(", "if ")
        if not "Fr_isTrue" in line:
            line = line.replace("){", "==1{")
        else:
            line = line.replace("){", "{")

        var = ""
        if "(--" in line: 
            start = line.find("(--")
            end = line.find(")")
            var = line[start+3:end]

            start = line.find("[")+1
            end = line.find("]")+1
            idx = line[start:end]

            line = line.replace("(--", "(")
            # line = line.replace(")", "-1)")

        line = line.replace("!", "!(")
        line = line.replace("==1{", "==1){")

        if var != "":
            tmp = var.replace(idx, "idx")
            line = f"\nlet idx={idx};\n{tmp} -= 1;\n" + line

    if "(ctx,lvarcall," in line:
        line = line.replace("(ctx,lvarcall,", "(ctx,&mut lvarcall,")
        
    if "_create(" in line:
        line = line.replace("\",", "\".to_string(),")

    if "ctx.generate_position_array" in line:
        line = line.replace("\"+", "\".to_owned()+")
        line = line.replace("ctx.generate_position_array", "&ctx.generate_position_array")

    if "ctx.templateInsId2IOSignalInfo" in line:
        line = line.replace("templateInsId2IOSignalInfo[", "templateInsId2IOSignalInfo.get(&")
        line = line.replace("].defs[", ").unwrap()[")

    if "*_functionTable" in line:
        line = line.replace("*_functionTable", "_functionTable")

    if "ctx.generate_position_array(aux_dimensions" in line:
        line = line.replace("ctx.generate_position_array(aux_dimensions", "ctx.generate_position_array(&aux_dimensions")

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
        if "Circom_TemplateFunction _functionTable" in line or "uint get_main_input_signal_start()" in line:
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
    fdatout = open("src/constants.dat", "wb")
    for i in range(0,GETTERS["get_size_of_constants()"]):
        buf = f.read(40)
        fdatout.write(buf)
    fdatout.close()

    # read rest of file and write into tmp dat 
    fdatout = open("src/iosignals.dat", "wb")
    while True:
        buf = f.read(1024)
        if not buf:
            break
        fdatout.write(buf)
    fdatout.close()

fout.write(FOOTER)
fout.close()

# additional processing

# {
# let aux_dest = &lvar[41] as *const FieldElement;
# Fr_copy!(aux_dest,ctx.circuitConstants[326]);
# }

with open(OUT_FILE, 'r') as file:
    data = file.read()

def get_template(i, j):
    return f"""{{
let aux_dest = &lvar[{i}] as *const FieldElement;
Fr_copy!(aux_dest,ctx.circuitConstants[{j}]);
}}
"""

def is_next(i0, j0, i1, j1):
    t1 = get_template(i0, j0)
    t2 = get_template(i1, j1)
    if data.index(t2) - data.index(t1) == len(t1):
        return True
    return False

rex = """({
let aux_dest = &lvar\[[0-9]+\] as \*const FieldElement;
Fr_copy!\(aux_dest,ctx\.circuitConstants\[[0-9]+\]\);
})"""

cc = [list(map(int, re.findall(r'\d+', code))) for code in re.findall(rex, data)]

for i in range(0, len(cc)):
    if cc[i] == None:
        continue
    
    if i+2<len(cc) and cc[i][1]+1==cc[i+1][1] and is_next(cc[i][0], cc[i][1], cc[i+1][0], cc[i+1][1]):
        start_i = cc[i][0]
        start_j = cc[i][1]
        j = 0
        old_code = ""
        while True:
            old_code += get_template(cc[i+j][0], cc[i+j][1])
            val = cc[i+j][1]
            if i+j+1 < len(cc):
                next_line = is_next(cc[i+j][0], cc[i+j][1], cc[i+j+1][0], cc[i+j+1][1])
            else:
                next_line = False
            cc[i+j] = None
            j += 1
            if i+j >= len(cc) or cc[i+j][1]-val != 1 or not next_line:
                break 
        new_code = f"for i in 0..{j} {{lvar[i+{start_i}]=ctx.circuitConstants[{start_j}+i];}}\n"
    elif i+2<len(cc) and cc[i][1]+2==cc[i+1][1] and is_next(cc[i][0], cc[i][1], cc[i+1][0], cc[i+1][1]):
        start_i = cc[i][0]
        start_j = cc[i][1]
        j = 0
        old_code = ""
        while True:
            old_code += get_template(cc[i+j][0], cc[i+j][1])
            val = cc[i+j][1]
            if i+j+1 < len(cc):
                next_line = is_next(cc[i+j][0], cc[i+j][1], cc[i+j+1][0], cc[i+j+1][1])
            else:
                next_line = False
            cc[i+j] = None
            j += 1
            if i+j >= len(cc) or cc[i+j][1]-val != 2 or not next_line:
                break 
        new_code = f"for i in 0..{j} {{lvar[i+{start_i}]=ctx.circuitConstants[{start_j}+(2*i)];}}\n"
    else:
        old_code = get_template(cc[i][0], cc[i][1])
        new_code = f"lvar[{cc[i][0]}]=ctx.circuitConstants[{cc[i][1]}];\n"
    
    data = data.replace(old_code, new_code, 1)
        
fout = open(OUT_FILE, "w")
fout.write(data)
fout.close()

# os.system(f"cargo fmt -- {OUT_FILE}")