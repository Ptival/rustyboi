use std::num::Wrapping;

use crate::{
    conditions::Condition,
    machine::Machine,
    registers::{R16, R8},
};

#[derive(Clone, Debug)]
pub struct Immediate16 {
    pub lower_byte: Wrapping<u8>,
    pub higher_byte: Wrapping<u8>,
}

impl Immediate16 {
    pub fn as_u16(&self) -> Wrapping<u16> {
        Wrapping((self.higher_byte.0 as u16) << 8 | self.lower_byte.0 as u16)
    }

    pub fn from_u16(u16: Wrapping<u16>) -> Self {
        Immediate16 {
            lower_byte: Wrapping(u16.0 as u8),
            higher_byte: Wrapping((u16.0 >> 8) as u8),
        }
    }

    // In ROM, immediate 16-bit values are stored lower-byte-first.
    pub fn from_memory(machine: &Machine, address: Wrapping<u16>) -> Immediate16 {
        Immediate16 {
            lower_byte: machine.read_u8(address),
            higher_byte: machine.read_u8(address + Wrapping(1)),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    ADC_A_mHL,
    ADC_A_r8(R8),
    ADC_A_u8(Wrapping<u8>),
    ADD_A_mHL,
    ADD_A_r8(R8),
    ADD_A_u8(Wrapping<u8>),
    ADD_HL_r16(R16),
    ADD_SP_i8(Wrapping<i8>),
    AND_A_mHL,
    AND_A_r8(R8),
    AND_u8(Wrapping<u8>),
    BIT_u3_mHL(u8),
    BIT_u3_r8(u8, R8),
    CALL_a16(Immediate16),
    CALL_cc_u16(Condition, Immediate16),
    CCF,
    CP_A_mHL,
    CP_A_r8(R8),
    CP_A_u8(Wrapping<u8>),
    CPL,
    DAA,
    DEC_mHL,
    DEC_r16(R16),
    DEC_r8(R8),
    DI,
    EI,
    HALT,
    Illegal(u8),
    INC_mHL,
    INC_r16(R16),
    INC_r8(R8),
    JP_cc_u16(Condition, Immediate16),
    JP_HL,
    JP_u16(Immediate16),
    JR_cc_i8(Condition, Wrapping<i8>),
    JR_i8(Wrapping<i8>),
    JR_r8(R8),
    LD_A_FFC,
    LD_A_FFu8(Wrapping<u8>),
    LD_A_mHLdec,
    LD_A_mHLinc,
    LD_A_mr16(R16),
    LD_A_mu16(Immediate16),
    LD_FFC_A,
    LD_FFu8_A(Wrapping<u8>),
    LD_H_mHL,
    LD_HL_SP_i8(Wrapping<i8>),
    LD_L_mHL,
    LD_mHL_u8(Wrapping<u8>),
    LD_mHLdec_A,
    LD_mHLinc_A,
    LD_mr16_r8(R16, R8),
    LD_mu16_A(Immediate16),
    LD_mu16_SP(Immediate16),
    LD_r16_d16(R16, Immediate16),
    LD_r8_mr16(R8, R16),
    LD_r8_r8(R8, R8),
    LD_r8_u8(R8, Wrapping<u8>),
    LD_SP_HL,
    LD_SP_u16(Immediate16),
    NOP,
    OR_A_mHL,
    OR_A_r8(R8),
    OR_A_u8(Wrapping<u8>),
    POP_r16(R16),
    PUSH_r16(R16),
    RES_u3_mHL(u8),
    RES_u3_r8(u8, R8),
    RET_cc(Condition),
    RET,
    RETI,
    RL_mHL,
    RL_r8(R8),
    RLA, // Note: this is different from "RL A"
    RLC_mHL,
    RLC_r8(R8),
    RLCA, // Note: this is different from "RLC A"
    RR_mHL,
    RR_r8(R8),
    RRA, // Note: this is different from "RR A"
    RRC_mHL,
    RRC_r8(R8),
    RRCA, // Note: this is different from "RRC A"
    RST(Immediate16),
    SBC_A_mHL,
    SBC_A_r8(R8),
    SBC_A_u8(Wrapping<u8>),
    SCF,
    SET_u3_mHL(u8),
    SET_u3_r8(u8, R8),
    SLA_mHL,
    SLA_r8(R8),
    SRA_mHL,
    SRA_r8(R8),
    SRL_mHL,
    SRL_r8(R8),
    STOP,
    SUB_A_mHL,
    SUB_A_r8(R8),
    SUB_A_u8(Wrapping<u8>),
    SWAP_mHL,
    SWAP_r8(R8),
    XOR_A_mHL,
    XOR_A_r8(R8),
    XOR_A_u8(Wrapping<u8>),
}
