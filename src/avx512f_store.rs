use std::arch::x86_64::*;

pub use crate::macros::*;

define_masked_store_unaligned!("avx512f", _mm512_mask_storeu_epi32, "32-bit integers", vmovdqu32, __m512i, zmm_reg, __mmask16, i32);
define_masked_store_unaligned!("avx512f", _mm512_mask_storeu_epi64, "64-bit integers", vmovdqu64, __m512i, zmm_reg, __mmask8, i64);
define_masked_store_unaligned!("avx512f", _mm512_mask_storeu_ps, "single-precision (32-bit) floating-point elements", vmovups, __m512, zmm_reg, __mmask16, f32);
define_masked_store_unaligned!("avx512f", _mm512_mask_storeu_pd, "double-precision (64-bit) floating-point elements", vmovupd, __m512d, zmm_reg, __mmask8, f64);

define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm256_mask_storeu_epi32, "32-bit integers", vmovdqu32, __m256i, ymm_reg, __mmask8, i32);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm256_mask_storeu_epi64, "64-bit integers", vmovdqu64, __m256i, ymm_reg, __mmask8, i64);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm256_mask_storeu_ps, "single-precision (32-bit) floating-point elements", vmovups, __m256, ymm_reg, __mmask8, f32);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm256_mask_storeu_pd, "double-precision (64-bit) floating-point elements", vmovupd, __m256d, ymm_reg, __mmask8, f64);

define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm_mask_storeu_epi32, "32-bit integers", vmovdqu32, __m128i, xmm_reg, __mmask8, i32);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm_mask_storeu_epi64, "64-bit integers", vmovdqu64, __m128i, xmm_reg, __mmask8, i64);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm_mask_storeu_ps, "single-precision (32-bit) floating-point elements", vmovups, __m128, xmm_reg, __mmask8, f32);
define_masked_store_unaligned!("avx512f,avx512vl,avx", _mm_mask_storeu_pd, "double-precision (64-bit) floating-point elements", vmovupd, __m128d, xmm_reg, __mmask8, f64);

define_masked_store_aligned!("avx512f", _mm512_mask_store_epi32, "32-bit integers", vmovdqa32, __m512i, zmm_reg, __mmask16, i32, "64-byte");
define_masked_store_aligned!("avx512f", _mm512_mask_store_epi64, "64-bit integers", vmovdqa64, __m512i, zmm_reg, __mmask8, i64, "64-byte");
define_masked_store_aligned!("avx512f", _mm512_mask_store_ps, "single-precision (32-bit) floating-point elements", vmovaps, __m512, zmm_reg, __mmask16, f32, "64-byte");
define_masked_store_aligned!("avx512f", _mm512_mask_store_pd, "double-precision (64-bit) floating-point elements", vmovapd, __m512d, zmm_reg, __mmask8, f64, "64-byte");

define_masked_store_aligned!("avx512f,avx512vl,avx", _mm256_mask_store_epi32, "32-bit integers", vmovdqa32, __m256i, ymm_reg, __mmask8, i32, "32-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm256_mask_store_epi64, "64-bit integers", vmovdqa64, __m256i, ymm_reg, __mmask8, i64, "32-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm256_mask_store_ps, "single-precision (32-bit) floating-point elements", vmovaps, __m256, ymm_reg, __mmask8, f32, "32-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm256_mask_store_pd, "double-precision (64-bit) floating-point elements", vmovapd, __m256d, ymm_reg, __mmask8, f64, "32-byte");

define_masked_store_aligned!("avx512f,avx512vl,avx", _mm_mask_store_epi32, "32-bit integers", vmovdqa32, __m128i, xmm_reg, __mmask8, i32, "16-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm_mask_store_epi64, "64-bit integers", vmovdqa64, __m128i, xmm_reg, __mmask8, i64, "16-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm_mask_store_ps, "single-precision (32-bit) floating-point elements", vmovaps, __m128, xmm_reg, __mmask8, f32, "16-byte");
define_masked_store_aligned!("avx512f,avx512vl,avx", _mm_mask_store_pd, "double-precision (64-bit) floating-point elements", vmovapd, __m128d, xmm_reg, __mmask8, f64, "16-byte");