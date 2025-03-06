#[derive(Debug)]
pub enum Instruction {
    
    // Carry bit instructions
    CMC, STC,

    // Single register instructions
    INR(u8), DCR(u8), CMA, DAA,

    // Nop instructions
    NOP,

    // Data transfer instructions
    MOV(u8, u8), SATX(u8), LDAX(u8), 

    // Register or memory to accumulator instructions
    ADD(u8), ADC(u8), SUB(u8), SBB(u8),
    ANA(u8), XRA(u8), ORA(u8), CMP(u8),

    // Rotate accumulator instructions
    RLC, RRC, RAL, RAR,

    // Register pair instructions
    PUSH(u8), POP(u8), DAD(u8), INX(u8),
    DCX(u8), XCHG, XTHL, SPHL,

    // Immediate instructions
    LXI(u8, u8, u8), MVI(u8, u8), ADI(u8),
    ACI(u8), SUI(u8), SBI(u8), ANI(u8),
    XBI(u8), XRI(u8), ORI(u8), CPI(u8),

    STA(u8, u8), LDA(u8, u8), SHLD(u8, u8), LHLD(u8, u8),

    // Jump instructions
    PCHL,
    JMP(u8, u8), JC(u8, u8), JNC(u8, u8), JZ(u8, u8),
    JNZ(u8, u8), JM(u8, u8), JP(u8, u8), JPE(u8, u8), JPO(u8, u8), 

    // Call subroutine instructions
    CALL(u8, u8), CC(u8, u8), CNC(u8, u8), CZ(u8, u8),
    CNZ(u8, u8), CM(u8, u8), CP(u8, u8), CPE(u8, u8), CPO(u8, u8),

    // Return from subroutine instructions
    RET, RC, RNC, RZ,
    RNZ, RM, RP, RPE, RPO,

    // RST instructions
    EI, DI,

    // Input/Output instructions
    IN(u8), OUT(u8),

    // HLT halt instruction
    HLT,
}