#![feature(asm)]
#![feature(avx512_target_feature)]
#![feature(stdsimd)]

#[macro_use]
pub(crate) mod macros;

mod avx512f_load;
mod avx512f_store;

mod avx512bw_load;
mod avx512bw_store;

pub use avx512f_load::*;
pub use avx512f_store::*;

pub use avx512bw_load::*;
pub use avx512bw_store::*;