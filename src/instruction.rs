#[derive(Debug)]
pub enum Instruction {
    
    // Carry bit instructions
    CMC, STC,

    // Single register instructions
    INR(Src), DCR(Src), CMA, DAA,

    // Nop instructions
    NOP,

    // Data transfer instructions
    MOV(Src, Src), SATX(RegPair), LDAX(RegPair), 

    // Register or memory to accumulator instructions
    ADD(Src), ADC(Src), SUB(Src), SBB(Src),
    ANA(Src), XRA(Src), ORA(Src), CMP(Src),

    // Rotate accumulator instructions
    RLC, RRC, RAL, RAR,

    // Register pair instructions
    PUSH(RegPair), POP(RegPair), DAD(RegPair), INX(RegPair),
    DCX(RegPair), XCHG, XTHL, SPHL,

    // Immediate instructions
    LXI(RegPair, u8, u8), MVI(Src, u8), 
    ADI(u8), ACI(u8), SUI(u8), SBI(u8), 
    ANI(u8), XRI(u8), ORI(u8), CPI(u8),

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
    RST(u8) ,EI, DI,

    // Input/Output instructions
    IN(u8), OUT(u8),

    // HLT halt instruction
    HLT,
}

#[derive(Debug, Clone, Copy)]
pub enum Src {
    B, C, D, E, H, L, A, Mem,
}

#[derive(Debug, Clone, Copy)]
pub enum RegPair {
    BC, DE, HL, PSW, SP,
}