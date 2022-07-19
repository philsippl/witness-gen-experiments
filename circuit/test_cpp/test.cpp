#include <stdio.h>
#include <iostream>
#include <assert.h>
#include "circom.hpp"
#include "calcwit.hpp"
void Multiplier_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Multiplier_0_run(uint ctx_index,Circom_CalcWit* ctx);
Circom_TemplateFunction _functionTable[1] = { 
Multiplier_0_run };
uint get_main_input_signal_start() {return 2;}

uint get_main_input_signal_no() {return 2;}

uint get_total_signal_no() {return 6;}

uint get_number_of_components() {return 1;}

uint get_size_of_input_hashmap() {return 256;}

uint get_size_of_witness() {return 6;}

uint get_size_of_constants() {return 2;}

uint get_size_of_io_map() {return 0;}

// function declarations
// template declarations
void Multiplier_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 0;
ctx->componentMemory[coffset].templateName = "Multiplier";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 2;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Multiplier_0_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* signalValues = ctx->signalValues;
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
FrElement* circuitConstants = ctx->circuitConstants;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
FrElement expaux[5];
FrElement lvar[1];
uint sub_component_aux;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 3];
// load src
Fr_sub(&expaux[2],&signalValues[mySignalStart + 1],&circuitConstants[1]); // line circom 32
Fr_div(&expaux[0],&circuitConstants[1],&expaux[2]); // line circom 32
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_sub(&expaux[2],&signalValues[mySignalStart + 1],&circuitConstants[1]); // line circom 33
Fr_mul(&expaux[1],&expaux[2],&signalValues[mySignalStart + 3]); // line circom 33
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 33
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 33. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
{
PFrElement aux_dest = &signalValues[mySignalStart + 4];
// load src
Fr_sub(&expaux[2],&signalValues[mySignalStart + 2],&circuitConstants[1]); // line circom 35
Fr_div(&expaux[0],&circuitConstants[1],&expaux[2]); // line circom 35
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_sub(&expaux[2],&signalValues[mySignalStart + 2],&circuitConstants[1]); // line circom 36
Fr_mul(&expaux[1],&expaux[2],&signalValues[mySignalStart + 4]); // line circom 36
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 36
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 36. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
Fr_mul(&expaux[0],&signalValues[mySignalStart + 1],&signalValues[mySignalStart + 2]); // line circom 38
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}

void run(Circom_CalcWit* ctx){
Multiplier_0_create(1,0,ctx,"main",0);
Multiplier_0_run(0,ctx);
}

