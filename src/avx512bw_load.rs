use std::arch::x86_64::*;

pub use crate::macros::*;

define_masked_load_unaligned!("avx512f,avx512bw", _mm512_mask_loadu_epi16, _mm512_maskz_loadu_epi16, "16-bit integers", vmovdqu16, __m512i, zmm_reg, __mmask32, i16);
define_masked_load_unaligned!("avx512f,avx512bw", _mm512_mask_loadu_epi8, _mm512_maskz_loadu_epi8, "8-bit integers", vmovdqu8, __m512i, zmm_reg, __mmask64, i8);

define_masked_load_unaligned!("avx512f,avx512bw,avx512vl,avx", _mm256_mask_loadu_epi16, _mm256_maskz_loadu_epi16, "16-bit integers", vmovdqu16, __m256i, ymm_reg, __mmask16, i16);
define_masked_load_unaligned!("avx512f,avx512bw,avx512vl,avx", _mm256_mask_loadu_epi8, _mm256_maskz_loadu_epi8, "8-bit integers", vmovdqu8, __m256i, ymm_reg, __mmask32, i8);

define_masked_load_unaligned!("avx512f,avx512bw,avx512vl,avx", _mm_mask_loadu_epi16, _mm_maskz_loadu_epi16, "16-bit integers", vmovdqu16, __m128i, xmm_reg, __mmask8, i16);
define_masked_load_unaligned!("avx512f,avx512bw,avx512vl,avx", _mm_mask_loadu_epi8, _mm_maskz_loadu_epi8, "8-bit integers", vmovdqu8, __m128i, xmm_reg, __mmask16, i8);
