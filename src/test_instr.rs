use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::*;

#[test]
fn test_inr() {
     
    let mut cpu = Cpu::new( );
    cpu.c = 0x99;
    cpu.ram.save_byte(0x0000, 0x0c);
    cpu.next().unwrap();
    assert_eq!(cpu.c, 0x9a);
}

#[test]
fn test_dcr() {
     
    let mut cpu = Cpu::new( );
    cpu.h = 0x3a;
    cpu.l = 0x7c;
    cpu.ram.save_byte(0x3a7c, 0x40);
    cpu.ram.save_byte(0x0000, 0x35);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x3a7c), 0x3f);
}

#[test]
fn test_cma() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x51;
    cpu.ram.save_byte(0x0000, 0x2f);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xae);
}

#[test]
fn test_daa() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x9b;
    cpu.ram.save_byte(0x0000, 0x27);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 1);
    assert!(cpu.get_flag(AUXILIARY_CARRY_BIT));
    assert!(cpu.get_flag(CARRY_BIT));
}

#[test]
fn test_mov() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xff;
    cpu.h = 0x2b;
    cpu.l = 0xe9;
    cpu.ram.save_byte(0x0000, 0x77);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x2be9), 0xff);
}

#[test]
fn test_stax() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xff;
    cpu.b = 0x3f;
    cpu.c = 0x16;
    cpu.ram.save_byte(0x0000, 0x02);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x3f16), 0xff);
}

#[test]
fn test_ldax() {
     
    let mut cpu =  Cpu::new( );
    cpu.d = 0x93;
    cpu.e = 0x8b;
    cpu.ram.save_byte(0x938b, 0xff);
    cpu.ram.save_byte(0x0000, 0x1a);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xff);
}

#[test]
fn test_add_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.d = 0x2e;
    cpu.a = 0x6c;
    cpu.ram.save_byte(0x0000, 0x82);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x9a);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), true);
    assert_eq!(cpu.get_flag(PARITY_BIT), true);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_add_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x01;
    cpu.ram.save_byte(0x0000, 0x87);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x02);
}

#[test]
fn test_adc_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x42;
    cpu.c = 0x3d;
    cpu.ram.save_byte(0x0000, 0x89);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x7f);
    assert_eq!(cpu.get_flag(SIGN_BIT), false);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), false);
    assert_eq!(cpu.get_flag(PARITY_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_adc_2() {
    let mut cpu =  Cpu::new( );
    cpu.a = 0x42;
    cpu.c = 0x3d;
    cpu.set_flag(CARRY_BIT, true);
    cpu.ram.save_byte(0x0000, 0x89);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x80);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), true);
    assert_eq!(cpu.get_flag(PARITY_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_adc_3() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x3f;
    cpu.flag = 0xd3;
    cpu.ram.save_byte(0x0000, 0x8f);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x7f);
    assert_eq!(cpu.flag, 0x12);
}

#[test]
fn test_sub() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x3e;
    cpu.ram.save_byte(0x0000, 0x97);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.get_flag(SIGN_BIT), false);
    assert_eq!(cpu.get_flag(ZERO_BIT), true);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), true);
    assert_eq!(cpu.get_flag(PARITY_BIT), true);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_sbb() {
     
    let mut cpu =  Cpu::new( );
    cpu.l = 0x02;
    cpu.a = 0x04;
    cpu.set_flag(CARRY_BIT, true);
    cpu.ram.save_byte(0x0000, 0x9d);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.get_flag(SIGN_BIT), false);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), true);
    assert_eq!(cpu.get_flag(PARITY_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_ana() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xfc;
    cpu.c = 0x0f;
    cpu.ram.save_byte(0x0000, 0xa1);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x0c);
}

#[test]
fn test_xra_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x0a;
    cpu.b = 0x0b;
    cpu.c = 0x0c;
    cpu.ram.save_byte(0x0000, 0xaf);
    cpu.ram.save_byte(0x0001, 0x47);
    cpu.ram.save_byte(0x0002, 0x4f);
    cpu.next().unwrap();
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.c, 0x00);
}

#[test]
fn test_xra_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xff;
    cpu.b = 0b1010_1010;
    cpu.ram.save_byte(0x0000, 0xa8);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0b0101_0101);
}

#[test]
fn test_xra_3() {}

#[test]
fn test_ora() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x33;
    cpu.c = 0x0f;
    cpu.ram.save_byte(0x0000, 0xb1);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x3f);
}

#[test]
fn test_cmp_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x0a;
    cpu.e = 0x05;
    cpu.ram.save_byte(0x0000, 0xbb);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x0a);
    assert_eq!(cpu.e, 0x05);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_cmp_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x02;
    cpu.e = 0x05;
    cpu.ram.save_byte(0x0000, 0xbb);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.e, 0x05);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_cmp_3() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xe5;
    cpu.e = 0x05;
    cpu.ram.save_byte(0x0000, 0xbb);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xe5);
    assert_eq!(cpu.e, 0x05);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_rlc() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xf2;
    cpu.ram.save_byte(0x0000, 0x07);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xe5);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_rrc() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xf2;
    cpu.ram.save_byte(0x0000, 0x0f);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x79);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_ral() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xb5;
    cpu.ram.save_byte(0x0000, 0x17);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x6a);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_rar() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x6a;
    cpu.set_flag(CARRY_BIT, true);
    cpu.ram.save_byte(0x0000, 0x1f);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xb5);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_stack_push_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.d = 0x8f;
    cpu.e = 0x9d;
    cpu.sp = 0x3a2c;
    cpu.ram.save_byte(0x0000, 0xd5);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x3a2b), 0x8f);
    assert_eq!(cpu.ram.load_byte(0x3a2a), 0x9d);
    assert_eq!(cpu.sp, 0x3a2a);
}

#[test]
fn test_stack_push_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x1f;
    cpu.sp = 0x502a;
    cpu.set_flag(CARRY_BIT, true);
    cpu.set_flag(ZERO_BIT, true);
    cpu.set_flag(PARITY_BIT, true);
    cpu.ram.save_byte(0x0000, 0xf5);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x5029), 0x1f);
    assert_eq!(cpu.ram.load_byte(0x5028), 0x47);
    assert_eq!(cpu.sp, 0x5028);
}

#[test]
fn test_stack_pop_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x1239, 0x3d);
    cpu.ram.save_byte(0x123a, 0x93);
    cpu.sp = 0x1239;
    cpu.ram.save_byte(0x0000, 0xe1);
    cpu.next().unwrap();
    assert_eq!(cpu.l, 0x3d);
    assert_eq!(cpu.h, 0x93);
    assert_eq!(cpu.sp, 0x123b);
}

#[test]
fn test_stack_pop_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x2c00, 0xc3);
    cpu.ram.save_byte(0x2c01, 0xff);
    cpu.sp = 0x2c00;
    cpu.ram.save_byte(0x0000, 0xf1);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.flag, 0xc3);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), true);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), false);
    assert_eq!(cpu.get_flag(PARITY_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_dad_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.b = 0x33;
    cpu.c = 0x9f;
    cpu.h = 0xa1;
    cpu.l = 0x7b;
    cpu.ram.save_byte(0x0000, 0x09);
    cpu.next().unwrap();
    assert_eq!(cpu.h, 0xd5);
    assert_eq!(cpu.l, 0x1a);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_dad_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.h = 0xa1;
    cpu.l = 0x7b;
    cpu.ram.save_byte(0x0000, 0x29);
    cpu.next().unwrap();
    assert_eq!(cpu.get_hl_addr(), 0xa17b << 1);
}

#[test]
fn test_inx_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.d = 0x38;
    cpu.e = 0xff;
    cpu.ram.save_byte(0x0000, 0x13);
    cpu.next().unwrap();
    assert_eq!(cpu.d, 0x39);
    assert_eq!(cpu.e, 0x00);
}

#[test]
fn test_inx_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.sp = 0xffff;
    cpu.ram.save_byte(0x0000, 0x33);
    cpu.next().unwrap();
    assert_eq!(cpu.sp, 0x0000);
}

#[test]
fn test_dcx() {
     
    let mut cpu =  Cpu::new( );
    cpu.h = 0x98;
    cpu.l = 0x00;
    cpu.ram.save_byte(0x0000, 0x2b);
    cpu.next().unwrap();
    assert_eq!(cpu.h, 0x97);
    assert_eq!(cpu.l, 0xff);
}

#[test]
fn test_xchg() {
     
    let mut cpu =  Cpu::new( );
    cpu.h = 0x00;
    cpu.l = 0xff;
    cpu.d = 0x33;
    cpu.e = 0x55;
    cpu.ram.save_byte(0x0000, 0xeb);
    cpu.next().unwrap();
    assert_eq!(cpu.h, 0x33);
    assert_eq!(cpu.l, 0x55);
    assert_eq!(cpu.d, 0x00);
    assert_eq!(cpu.e, 0xff);
}

#[test]
fn test_xthl() {
     
    let mut cpu =  Cpu::new( );
    cpu.sp = 0x10ad;
    cpu.h = 0x0b;
    cpu.l = 0x3c;
    cpu.ram.save_byte(0x10ad, 0xf0);
    cpu.ram.save_byte(0x10ae, 0x0d);
    cpu.ram.save_byte(0x0000, 0xe3);
    cpu.next().unwrap();
    assert_eq!(cpu.h, 0x0d);
    assert_eq!(cpu.l, 0xf0);
    assert_eq!(cpu.ram.load_byte(0x10ad), 0x3c);
    assert_eq!(cpu.ram.load_byte(0x10ae), 0x0b);
}

#[test]
fn test_sphl() {
     
    let mut cpu =  Cpu::new( );
    cpu.h = 0x50;
    cpu.l = 0x6c;
    cpu.ram.save_byte(0x0000, 0xf9);
    cpu.next().unwrap();
    assert_eq!(cpu.sp, 0x506c);
}

#[test]
fn test_mvi() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0x26);
    cpu.ram.save_byte(0x0001, 0x3c);
    cpu.ram.save_byte(0x0002, 0x2e);
    cpu.ram.save_byte(0x0003, 0xf4);
    cpu.ram.save_byte(0x0004, 0x36);
    cpu.ram.save_byte(0x0005, 0xff);
    cpu.next().unwrap();
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.h, 0x3c);
    assert_eq!(cpu.l, 0xf4);
    assert_eq!(cpu.ram.load_byte(0x3cf4), 0xff);
}

#[test]
fn test_adi() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0x3e);
    cpu.ram.save_byte(0x0001, 0x14);
    cpu.ram.save_byte(0x0002, 0xc6);
    cpu.ram.save_byte(0x0003, 0x42);
    cpu.ram.save_byte(0x0004, 0xc6);
    cpu.ram.save_byte(0x0005, 0xbe);
    cpu.next().unwrap();
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x14);
    assert_eq!(cpu.get_flag(SIGN_BIT), false);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), true);
    assert_eq!(cpu.get_flag(PARITY_BIT), true);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_aci() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0x3e);
    cpu.ram.save_byte(0x0001, 0x56);
    cpu.ram.save_byte(0x0002, 0xce);
    cpu.ram.save_byte(0x0003, 0xbe);
    cpu.ram.save_byte(0x0004, 0xce);
    cpu.ram.save_byte(0x0005, 0x42);
    cpu.next().unwrap();
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x57);
}

#[test]
fn test_sui() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0x3e);
    cpu.ram.save_byte(0x0001, 0x00);
    cpu.ram.save_byte(0x0002, 0xd6);
    cpu.ram.save_byte(0x0003, 0x01);
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), false);
    assert_eq!(cpu.get_flag(PARITY_BIT), true);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_sbi_1() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0xde);
    cpu.ram.save_byte(0x0001, 0x01);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), false);
    assert_eq!(cpu.get_flag(PARITY_BIT), true);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_sbi_2() {
     
    let mut cpu =  Cpu::new( );
    cpu.set_flag(CARRY_BIT, true);
    cpu.ram.save_byte(0x0000, 0xde);
    cpu.ram.save_byte(0x0001, 0x01);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xfe);
    assert_eq!(cpu.get_flag(SIGN_BIT), true);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(AUXILIARY_CARRY_BIT), false);
    assert_eq!(cpu.get_flag(PARITY_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), true);
}

#[test]
fn test_ani() {
     
    let mut cpu =  Cpu::new( );
    cpu.c = 0x3a;
    cpu.ram.save_byte(0x0000, 0x79);
    cpu.ram.save_byte(0x0001, 0xe6);
    cpu.ram.save_byte(0x0002, 0x0f);
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x0a);
}

#[test]
fn test_xri() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0x3b;
    cpu.ram.save_byte(0x0000, 0xee);
    cpu.ram.save_byte(0x0001, 0x81);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xba);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_ori() {
     
    let mut cpu =  Cpu::new( );
    cpu.c = 0xb5;
    cpu.ram.save_byte(0x0000, 0x79);
    cpu.ram.save_byte(0x0001, 0xf6);
    cpu.ram.save_byte(0x0002, 0x0f);
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xbf);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_cpi() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0000, 0x3e);
    cpu.ram.save_byte(0x0001, 0x4a);
    cpu.ram.save_byte(0x0002, 0xfe);
    cpu.ram.save_byte(0x0003, 0x40);
    cpu.next().unwrap();
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0x4a);
    assert_eq!(cpu.get_flag(ZERO_BIT), false);
    assert_eq!(cpu.get_flag(CARRY_BIT), false);
}

#[test]
fn test_sta() {
     
    let mut cpu =  Cpu::new( );
    cpu.a = 0xff;
    cpu.ram.save_byte(0x0000, 0x32);
    cpu.ram.save_byte(0x0001, 0xb3);
    cpu.ram.save_byte(0x0002, 0x05);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x05b3), 0xff);
}

#[test]
fn test_lda() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x0300, 0xff);
    cpu.ram.save_byte(0x0000, 0x3a);
    cpu.ram.save_byte(0x0001, 0x00);
    cpu.ram.save_byte(0x0002, 0x03);
    cpu.next().unwrap();
    assert_eq!(cpu.a, 0xff);
}

#[test]
fn test_shld() {
     
    let mut cpu =  Cpu::new( );
    cpu.h = 0xae;
    cpu.l = 0x29;
    cpu.ram.save_byte(0x0000, 0x22);
    cpu.ram.save_byte(0x0001, 0x0a);
    cpu.ram.save_byte(0x0002, 0x01);
    cpu.next().unwrap();
    assert_eq!(cpu.ram.load_byte(0x010a), 0x29);
    assert_eq!(cpu.ram.load_byte(0x010b), 0xae);
}

#[test]
fn test_lhld() {
     
    let mut cpu =  Cpu::new( );
    cpu.ram.save_byte(0x025b, 0xff);
    cpu.ram.save_byte(0x025c, 0x03);
    cpu.ram.save_byte(0x0000, 0x2a);
    cpu.ram.save_byte(0x0001, 0x5b);
    cpu.ram.save_byte(0x0002, 0x02);
    cpu.next().unwrap();
    assert_eq!(cpu.l, 0xff);
    assert_eq!(cpu.h, 0x03);
}

#[test]
fn test_pchl() {     
    let mut cpu =  Cpu::new( );
    cpu.h = 0x41;
    cpu.l = 0x3e;
    cpu.ram.save_byte(0x0000, 0xe9);
    cpu.next().unwrap();
    assert_eq!(cpu.pc, 0x413e);
}
