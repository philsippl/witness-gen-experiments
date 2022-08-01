#include <stdio.h>
#include <iostream>
#include <assert.h>
#include "circom.hpp"
#include "calcwit.hpp"
void Num2Bits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Num2Bits_0_run(uint ctx_index,Circom_CalcWit* ctx);
Circom_TemplateFunction _functionTable[1] = { 
Num2Bits_0_run };
uint get_main_input_signal_start() {return 9;}

uint get_main_input_signal_no() {return 1;}

uint get_total_signal_no() {return 10;}

uint get_number_of_components() {return 1;}

uint get_size_of_input_hashmap() {return 256;}

uint get_size_of_witness() {return 9;}

uint get_size_of_constants() {return 4;}

uint get_size_of_io_map() {return 0;}

// function declarations
// template declarations
void Num2Bits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 0;
ctx->componentMemory[coffset].templateName = "Num2Bits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Num2Bits_0_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* signalValues = ctx->signalValues;
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
FrElement* circuitConstants = ctx->circuitConstants;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
FrElement expaux[7];
FrElement lvar[3];
uint sub_component_aux;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 8
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)];
// load src
Fr_shr(&expaux[1],&signalValues[mySignalStart + 8],&lvar[2]); // line circom 9
Fr_band(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 9
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_sub(&expaux[3],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)],&circuitConstants[2]); // line circom 10
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)],&expaux[3]); // line circom 10
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 10
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 10. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_pow(&expaux[4],&circuitConstants[3],&lvar[2]); // line circom 11
Fr_mul(&expaux[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)],&expaux[4]); // line circom 11
Fr_add(&expaux[0],&lvar[1],&expaux[2]); // line circom 11
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 8
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 8
}
Fr_eq(&expaux[0],&lvar[1],&signalValues[mySignalStart + 8]); // line circom 14
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 14. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}

void run(Circom_CalcWit* ctx){
Num2Bits_0_create(1,0,ctx,"main",0);
Num2Bits_0_run(0,ctx);
}

