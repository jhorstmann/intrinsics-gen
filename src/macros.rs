pub use assert_instr_macro::*;

macro_rules! define_masked_load_aligned {
    ($feature:literal, $name:ident, $name_zero_masked:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path, $alignment_description:literal) => {
        define_masked_load!($feature, $name, $name_zero_masked, $element_description, $instruction, $simd_type, $reg_type, $mask_type, $lane_type, "", "mem_addr must be aligned on a ", $alignment_description, " boundary or a general-protection exception may be generated.");
    };
}

macro_rules! define_masked_load_unaligned {
    ($feature:literal, $name:ident, $name_zero_masked:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path) => {
        define_masked_load!($feature, $name, $name_zero_masked, $element_description, $instruction, $simd_type, $reg_type, $mask_type, $lane_type, "mem_addr does not need to be aligned on any particular boundary.");
    };
}

macro_rules! define_masked_load {
    ($feature:literal, $name:ident, $name_zero_masked:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path, $($additional_doc:literal),*) => {
        #[doc = concat!("Load packed ", $element_description, " from memory into dst using writemask k")]
        #[doc = "(elements are copied from src when the corresponding mask bit is not set)."]
        #[doc = concat!($($additional_doc),*)]
        #[doc = ""]
        #[doc = concat!("[Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=", stringify!($name), ")")]
        #[inline]
        #[target_feature(enable = $feature)]
        #[cfg_attr(test, assert_instr($instruction))]
        pub unsafe fn $name(src: $simd_type, k: $mask_type, mem_addr: *const $lane_type) -> $simd_type {
            let mut dst: $simd_type = src;
            asm!(
                concat!(stringify!($instruction), " {r}{{{k}}}, [{p}]"),
                p = in(reg) mem_addr,
                k = in(kreg) k,
                r = inout($reg_type) dst,
                options(nostack), options(pure), options(readonly)
            );
            dst
        }

        #[doc = concat!("Load packed ", $element_description, " from memory into dst using zeromask k")]
        #[doc = "(elements are zeroed out when the corresponding mask bit is not set)."]
        #[doc = concat!($($additional_doc),*)]
        #[doc = ""]
        #[doc = concat!("[Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=", stringify!($name_zero_masked), ")")]
        #[inline]
        #[target_feature(enable = $feature)]
        #[cfg_attr(test, assert_instr($instruction))]
        pub unsafe fn $name_zero_masked(k: $mask_type, mem_addr: *const $lane_type) -> $simd_type {
            let mut dst: $simd_type;
            asm!(
                concat!(stringify!($instruction), " {r}{{{k}}} {{z}}, [{p}]"),
                p = in(reg) mem_addr,
                k = in(kreg) k,
                r = out($reg_type) dst,
                options(nostack), options(pure), options(readonly)
            );
            dst
        }
    };
}

macro_rules! define_masked_store {
    ($feature:literal, $name:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path, $($additional_doc:literal),+) => {
        #[doc = concat!("Store packed ", $element_description, " from a into memory using writemask k.")]
        #[doc = concat!($($additional_doc),*)]
        #[doc = ""]
        #[doc = concat!("[Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=", stringify!($name), ")")]
        #[inline]
        #[target_feature(enable = $feature)]
        #[cfg_attr(test, assert_instr($instruction))]
        pub unsafe fn $name(mem_addr: *mut $lane_type, mask: $mask_type, a: $simd_type) {
            asm!(
                concat!(stringify!($instruction), " [{p}]{{{k}}}, {a}"),
                p = in(reg) mem_addr,
                k = in(kreg) mask,
                a = in($reg_type) a,
                options(nostack)
            );
        }
    }
}

macro_rules! define_masked_store_aligned {
    ($feature:literal, $name:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path, $alignment_description:literal) => {
        define_masked_store!($feature, $name, $element_description, $instruction, $simd_type, $reg_type, $mask_type, $lane_type, "mem_addr must be aligned on a ", $alignment_description, " boundary or a general-protection exception may be generated.");
    };
}

macro_rules! define_masked_store_unaligned {
    ($feature:literal, $name:ident, $element_description:literal, $instruction:ident, $simd_type:path, $reg_type:ident, $mask_type:path, $lane_type:path) => {
        define_masked_store!($feature, $name, $element_description, $instruction, $simd_type, $reg_type, $mask_type, $lane_type, "mem_addr does not need to be aligned on any particular boundary.");
    };
}
