use core::arch::asm;
use core::fmt;
use core::ops;
use gimli::{Register, RiscV};

// Match DWARF_FRAME_REGISTERS in libgcc
pub const MAX_REG_RULES: usize = 74;

#[repr(C)]
#[derive(Clone, Default)]
pub struct Context {
    pub gp: [usize; 32],
    pub fp: [u64; 32],
    // condition code registers
    pub ccr: [usize; 8],
    // accumulator registers
    pub ar: [usize; 2],
}

impl fmt::Debug for Context {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_struct("Context");
        for i in 0..=31 {
            fmt.field(RiscV::register_name(Register(i as _)).unwrap(), &self.gp[i]);
        }
        for i in 0..=31 {
            fmt.field(
                RiscV::register_name(Register((i + 32) as _)).unwrap(),
                &self.fp[i],
            );
        }
        fmt.finish()
    }
}

impl ops::Index<Register> for Context {
    type Output = usize;

    fn index(&self, reg: Register) -> &usize {
        match reg {
            Register(0..=31) => &self.gp[reg.0 as usize],
            _ => unimplemented!(),
        }
    }
}

impl ops::IndexMut<gimli::Register> for Context {
    fn index_mut(&mut self, reg: Register) -> &mut usize {
        match reg {
            Register(0..=31) => &mut self.gp[reg.0 as usize],
            _ => unimplemented!(),
        }
    }
}

// TODO
// https://github.com/llvm/llvm-project/blob/main/libunwind/src/UnwindRegistersRestore.S
// https://github.com/llvm/llvm-project/blob/main/libunwind/src/UnwindRegistersSave.S
macro_rules! code {
    (save_gp) => {
        "
        .set push
        .set noat
        sw    $1, (4 * 1)($4)
        sw    $2, (4 * 2)($4)
        sw    $3, (4 * 3)($4)
        sw    $4, (4 * 4)($4)
        sw    $5, (4 * 5)($4)
        sw    $6, (4 * 6)($4)
        sw    $7, (4 * 7)($4)
        sw    $8, (4 * 8)($4)
        sw    $9, (4 * 9)($4)
        sw    $10, (4 * 10)($4)
        sw    $11, (4 * 11)($4)
        sw    $12, (4 * 12)($4)
        sw    $13, (4 * 13)($4)
        sw    $14, (4 * 14)($4)
        sw    $15, (4 * 15)($4)
        sw    $16, (4 * 16)($4)
        sw    $17, (4 * 17)($4)
        sw    $18, (4 * 18)($4)
        sw    $19, (4 * 19)($4)
        sw    $20, (4 * 20)($4)
        sw    $21, (4 * 21)($4)
        sw    $22, (4 * 22)($4)
        sw    $23, (4 * 23)($4)
        sw    $24, (4 * 24)($4)
        sw    $25, (4 * 25)($4)
        sw    $26, (4 * 26)($4)
        sw    $27, (4 * 27)($4)
        sw    $28, (4 * 28)($4)
        sw    $29, (4 * 29)($4)
        sw    $30, (4 * 30)($4)
        sw    $31, (4 * 31)($4)
        # Store return address to pc
        sw    $31, (4 * 32)($4)
        # hi and lo
        mfhi  $8
        sw    $8,  (4 * 33)($4)
        mflo  $8
        sw    $8,  (4 * 34)($4)
        .set pop
        "
    };
    (save_fp) => {
        "
        .set push
        .set noat
        #ifdef __mips_hard_float
        #if __mips_fpr != 64
            sdc1  $f0, (4 * 36 + 8 * 0)($4)
            sdc1  $f2, (4 * 36 + 8 * 2)($4)
            sdc1  $f4, (4 * 36 + 8 * 4)($4)
            sdc1  $f6, (4 * 36 + 8 * 6)($4)
            sdc1  $f8, (4 * 36 + 8 * 8)($4)
            sdc1  $f10, (4 * 36 + 8 * 10)($4)
            sdc1  $f12, (4 * 36 + 8 * 12)($4)
            sdc1  $f14, (4 * 36 + 8 * 14)($4)
            sdc1  $f16, (4 * 36 + 8 * 16)($4)
            sdc1  $f18, (4 * 36 + 8 * 18)($4)
            sdc1  $f20, (4 * 36 + 8 * 20)($4)
            sdc1  $f22, (4 * 36 + 8 * 22)($4)
            sdc1  $f24, (4 * 36 + 8 * 24)($4)
            sdc1  $f26, (4 * 36 + 8 * 26)($4)
            sdc1  $f28, (4 * 36 + 8 * 28)($4)
            sdc1  $f30, (4 * 36 + 8 * 30)($4)
        #else
            sdc1  $f0, (4 * 36 + 8 * 0)($4)
            sdc1  $f1, (4 * 36 + 8 * 1)($4)
            sdc1  $f2, (4 * 36 + 8 * 2)($4)
            sdc1  $f3, (4 * 36 + 8 * 3)($4)
            sdc1  $f4, (4 * 36 + 8 * 4)($4)
            sdc1  $f5, (4 * 36 + 8 * 5)($4)
            sdc1  $f6, (4 * 36 + 8 * 6)($4)
            sdc1  $f7, (4 * 36 + 8 * 7)($4)
            sdc1  $f8, (4 * 36 + 8 * 8)($4)
            sdc1  $f9, (4 * 36 + 8 * 9)($4)
            sdc1  $f10, (4 * 36 + 8 * 10)($4)
            sdc1  $f11, (4 * 36 + 8 * 11)($4)
            sdc1  $f12, (4 * 36 + 8 * 12)($4)
            sdc1  $f13, (4 * 36 + 8 * 13)($4)
            sdc1  $f14, (4 * 36 + 8 * 14)($4)
            sdc1  $f15, (4 * 36 + 8 * 15)($4)
            sdc1  $f16, (4 * 36 + 8 * 16)($4)
            sdc1  $f17, (4 * 36 + 8 * 17)($4)
            sdc1  $f18, (4 * 36 + 8 * 18)($4)
            sdc1  $f19, (4 * 36 + 8 * 19)($4)
            sdc1  $f20, (4 * 36 + 8 * 20)($4)
            sdc1  $f21, (4 * 36 + 8 * 21)($4)
            sdc1  $f22, (4 * 36 + 8 * 22)($4)
            sdc1  $f23, (4 * 36 + 8 * 23)($4)
            sdc1  $f24, (4 * 36 + 8 * 24)($4)
            sdc1  $f25, (4 * 36 + 8 * 25)($4)
            sdc1  $f26, (4 * 36 + 8 * 26)($4)
            sdc1  $f27, (4 * 36 + 8 * 27)($4)
            sdc1  $f28, (4 * 36 + 8 * 28)($4)
            sdc1  $f29, (4 * 36 + 8 * 29)($4)
            sdc1  $f30, (4 * 36 + 8 * 30)($4)
            sdc1  $f31, (4 * 36 + 8 * 31)($4)
        #endif
        #endif
        .set pop
        "
    };
    (restore_gp) => {
        "
        .set push
        .set noat
        // restore hi and lo
        lw    $8, (4 * 33)($4)
        mthi  $8
        lw    $8, (4 * 34)($4)
        mtlo  $8
        // r0 is zero
        lw    $1, (4 * 1)($4)
        lw    $2, (4 * 2)($4)
        lw    $3, (4 * 3)($4)
        // skip a0 for now
        lw    $5, (4 * 5)($4)
        lw    $6, (4 * 6)($4)
        lw    $7, (4 * 7)($4)
        lw    $8, (4 * 8)($4)
        lw    $9, (4 * 9)($4)
        lw    $10, (4 * 10)($4)
        lw    $11, (4 * 11)($4)
        lw    $12, (4 * 12)($4)
        lw    $13, (4 * 13)($4)
        lw    $14, (4 * 14)($4)
        lw    $15, (4 * 15)($4)
        lw    $16, (4 * 16)($4)
        lw    $17, (4 * 17)($4)
        lw    $18, (4 * 18)($4)
        lw    $19, (4 * 19)($4)
        lw    $20, (4 * 20)($4)
        lw    $21, (4 * 21)($4)
        lw    $22, (4 * 22)($4)
        lw    $23, (4 * 23)($4)
        lw    $24, (4 * 24)($4)
        lw    $25, (4 * 25)($4)
        lw    $26, (4 * 26)($4)
        lw    $27, (4 * 27)($4)
        lw    $28, (4 * 28)($4)
        lw    $29, (4 * 29)($4)
        lw    $30, (4 * 30)($4)
        // load new pc into ra
        lw    $31, (4 * 32)($4)
        // jump to ra, load a0 in the delay slot
        jr    $31
        lw    $4, (4 * 4)($4)
        .set pop
        "
    };
    (restore_fp) => {
        "
        .set push
        .set noat
        #ifdef __mips_hard_float
        #if __mips_fpr != 64
            ldc1  $f0, (4 * 36 + 8 * 0)($4)
            ldc1  $f2, (4 * 36 + 8 * 2)($4)
            ldc1  $f4, (4 * 36 + 8 * 4)($4)
            ldc1  $f6, (4 * 36 + 8 * 6)($4)
            ldc1  $f8, (4 * 36 + 8 * 8)($4)
            ldc1  $f10, (4 * 36 + 8 * 10)($4)
            ldc1  $f12, (4 * 36 + 8 * 12)($4)
            ldc1  $f14, (4 * 36 + 8 * 14)($4)
            ldc1  $f16, (4 * 36 + 8 * 16)($4)
            ldc1  $f18, (4 * 36 + 8 * 18)($4)
            ldc1  $f20, (4 * 36 + 8 * 20)($4)
            ldc1  $f22, (4 * 36 + 8 * 22)($4)
            ldc1  $f24, (4 * 36 + 8 * 24)($4)
            ldc1  $f26, (4 * 36 + 8 * 26)($4)
            ldc1  $f28, (4 * 36 + 8 * 28)($4)
            ldc1  $f30, (4 * 36 + 8 * 30)($4)
        #else
            ldc1  $f0, (4 * 36 + 8 * 0)($4)
            ldc1  $f1, (4 * 36 + 8 * 1)($4)
            ldc1  $f2, (4 * 36 + 8 * 2)($4)
            ldc1  $f3, (4 * 36 + 8 * 3)($4)
            ldc1  $f4, (4 * 36 + 8 * 4)($4)
            ldc1  $f5, (4 * 36 + 8 * 5)($4)
            ldc1  $f6, (4 * 36 + 8 * 6)($4)
            ldc1  $f7, (4 * 36 + 8 * 7)($4)
            ldc1  $f8, (4 * 36 + 8 * 8)($4)
            ldc1  $f9, (4 * 36 + 8 * 9)($4)
            ldc1  $f10, (4 * 36 + 8 * 10)($4)
            ldc1  $f11, (4 * 36 + 8 * 11)($4)
            ldc1  $f12, (4 * 36 + 8 * 12)($4)
            ldc1  $f13, (4 * 36 + 8 * 13)($4)
            ldc1  $f14, (4 * 36 + 8 * 14)($4)
            ldc1  $f15, (4 * 36 + 8 * 15)($4)
            ldc1  $f16, (4 * 36 + 8 * 16)($4)
            ldc1  $f17, (4 * 36 + 8 * 17)($4)
            ldc1  $f18, (4 * 36 + 8 * 18)($4)
            ldc1  $f19, (4 * 36 + 8 * 19)($4)
            ldc1  $f20, (4 * 36 + 8 * 20)($4)
            ldc1  $f21, (4 * 36 + 8 * 21)($4)
            ldc1  $f22, (4 * 36 + 8 * 22)($4)
            ldc1  $f23, (4 * 36 + 8 * 23)($4)
            ldc1  $f24, (4 * 36 + 8 * 24)($4)
            ldc1  $f25, (4 * 36 + 8 * 25)($4)
            ldc1  $f26, (4 * 36 + 8 * 26)($4)
            ldc1  $f27, (4 * 36 + 8 * 27)($4)
            ldc1  $f28, (4 * 36 + 8 * 28)($4)
            ldc1  $f29, (4 * 36 + 8 * 29)($4)
            ldc1  $f30, (4 * 36 + 8 * 30)($4)
            ldc1  $f31, (4 * 36 + 8 * 31)($4)
        #endif
        #endif
        .set pop
        "
    };
}

#[naked]
pub extern "C-unwind" fn save_context() -> Context {
    // No need to save caller-saved registers here.
    #[cfg(target_feature = "d")]
    unsafe {
        asm!(
            concat!(code!(save_gp), code!(save_fp), "ret"),
            options(noreturn)
        );
    }
    #[cfg(not(target_feature = "d"))]
    unsafe {
        asm!(concat!(code!(save_gp), "ret"), options(noreturn));
    }
}

#[naked]
pub unsafe extern "C" fn restore_context(ctx: &Context) -> ! {
    #[cfg(target_feature = "d")]
    unsafe {
        asm!(
            concat!(code!(restore_fp), code!(restore_gp), "lw a0, 0x28(a0)\nret"),
            options(noreturn)
        );
    }
    #[cfg(not(target_feature = "d"))]
    unsafe {
        asm!(
            concat!(code!(restore_gp), "lw a0, 0x28(a0)\nret"),
            options(noreturn)
        );
    }
}
