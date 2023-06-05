use crate::{c_char, c32, c64};

/// Single precision real
///
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) If `uplo = 'U'`, the upper triangular part of `a` contains
/// the upper triangular part of the matrix `a`, and the strictly lower triangular
/// part of `a` is not referenced. If `uplo = 'L'`, the lower triangular part of `a`
/// contains the lower triangular part of the matrix `a`, and the strictly upper
/// triangular part of `a` is not referenced.
/// * __*`mode`*__ - (input) If 'N':  `a` is fully tridiagonalized.
/// If 'P':  `a` is partially tridiagonalized for Pfaffian computation.
/// * __*`n`*__ - (input) Size of the matrix `a`. If `mode = 'P'`, `n` must be even. `n
/// >= 0`.
/// * __*`a`*__ - (input/output) Pointer to a memory block of size `n*n*sizeof(datatype)`.
/// On entry, the skew-symmetric `n`x`n`-matrix `a` in Fortran format.
///
///     > If `uplo = 'U'`, the upper triangular part of `a` contains
///     > the upper triangular part of the matrix `a`, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the lower triangular part of `a` contains
///     > the lower triangular part of the matrix `a`, and the
///     > strictly upper triangular part of `a` is not referenced.
///
///     On exit, the tridiagonal matrix T and the multipliers used
///     to obtain the factor U or L (see below for further details).
/// * __*`ldim`*__ - (input) The leading dimension of the array `a`.
/// `ldim >= max(1,n)`.
/// * __*`ipiv`*__ - (output) Pointer to an array of dimension N.
/// Information about the permutation matrix P: row and column
/// i are interchanged with `ipiv[i-1]`. If `uplo = 'U'`, those
/// interchanges are done in the order `i = N ... 1`, if `uplo = 'L'`
/// in the order `i = 1 ... N`.
/// * __*`work`*__ - (workspace/output) Array of dimension `max(1,lwork)`.
/// On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The length of `work`.  `lwork >= 1`.
/// For best performance
/// `lwork >= n*NB`, where `NB` is the block size returned by ILAENV
/// (at the moment, uses the same block size as SSYTRF from Lapack).
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by
/// [XERBLA](https://netlib.org/lapack/explore-html/d0/d73/group__aux__blas_ga377ee61015baf8dea7770b3a404b1c07.html).
/// * __*`info`*__ - (output) Succes of the computation. If `info`
///   > = 0: successful exit,
///
///   > < 0: if `info = -k`, the k-th argument had an illegal value,
///
///   > \> 0: if `info = k`, the off-diagonal entry in the k-th row
///   > `uplo = 'U'` or k-th column `uplo = 'L'`
///   > is exactly zero.
///
///   > = -100: failed to allocate enough internal memory.
///
#[inline]
pub unsafe fn ssktrf(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [f32],
    ldim: &i32,
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: &[i32],
    info: &mut i32,
) {
    ffi::ssktrf_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr(),
        ldim,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork.as_ptr(),
        info,
    )
}


/// Double precision real
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) If `uplo = 'U'`, the upper triangular part of `a` contains
/// the upper triangular part of the matrix `a`, and the strictly lower triangular
/// part of `a` is not referenced. If `uplo = 'L'`, the lower triangular part of `a`
/// contains the lower triangular part of the matrix `a`, and the strictly upper
/// triangular part of `a` is not referenced.
/// * __*`mode`*__ - (input) If 'N':  `a` is fully tridiagonalized.
/// If 'P':  `a` is partially tridiagonalized for Pfaffian computation.
/// * __*`n`*__ - (input) Size of the matrix `a`. If `mode = 'P'`, `n` must be even. `n
/// >= 0`.
/// * __*`a`*__ - (input/output) Pointer to a memory block of size `n*n*sizeof(datatype)`.
/// On entry, the skew-symmetric `n`x`n`-matrix `a` in Fortran format.
///
///     > If `uplo = 'U'`, the upper triangular part of `a` contains
///     > the upper triangular part of the matrix `a`, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the lower triangular part of `a` contains
///     > the lower triangular part of the matrix `a`, and the
///     > strictly upper triangular part of `a` is not referenced.
///
///     On exit, the tridiagonal matrix T and the multipliers used
///     to obtain the factor U or L (see below for further details).
/// * __*`ldim`*__ - (input) The leading dimension of the array `a`.
/// `ldim >= max(1,n)`.
/// * __*`ipiv`*__ - (output) Pointer to an array of dimension N.
/// Information about the permutation matrix P: row and column
/// i are interchanged with `ipiv[i-1]`. If `uplo = 'U'`, those
/// interchanges are done in the order `i = N ... 1`, if `uplo = 'L'`
/// in the order `i = 1 ... N`.
/// * __*`work`*__ - (workspace/output) Array of dimension `max(1,lwork)`.
/// On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The length of `work`.  `lwork >= 1`.
/// For best performance
/// `lwork >= n*NB`, where `NB` is the block size returned by ILAENV
/// (at the moment, uses the same block size as SSYTRF from Lapack).
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by
/// [XERBLA](https://netlib.org/lapack/explore-html/d0/d73/group__aux__blas_ga377ee61015baf8dea7770b3a404b1c07.html).
/// * __*`info`*__ - (output) Succes of the computation. If `info`
///   > = 0: successful exit,
///
///   > < 0: if `info = -k`, the k-th argument had an illegal value,
///
///   > \> 0: if `info = k`, the off-diagonal entry in the k-th row
///   > `uplo = 'U'` or k-th column `uplo = 'L'`
///   > is exactly zero.
///
///   > = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn dsktrf(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [f64],
    ldim: &i32,
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: &i32,
    info: &mut i32
) {
    ffi::dsktrf_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr(),
        ldim,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        info,
    )
}


/// Single precision complex
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) If `uplo = 'U'`, the upper triangular part of `a` contains
/// the upper triangular part of the matrix `a`, and the strictly lower triangular
/// part of `a` is not referenced. If `uplo = 'L'`, the lower triangular part of `a`
/// contains the lower triangular part of the matrix `a`, and the strictly upper
/// triangular part of `a` is not referenced.
/// * __*`mode`*__ - (input) If 'N':  `a` is fully tridiagonalized.
/// If 'P':  `a` is partially tridiagonalized for Pfaffian computation.
/// * __*`n`*__ - (input) Size of the matrix `a`. If `mode = 'P'`, `n` must be even. `n
/// >= 0`.
/// * __*`a`*__ - (input/output) Pointer to a memory block of size `n*n*sizeof(datatype)`.
/// On entry, the skew-symmetric `n`x`n`-matrix `a` in Fortran format.
///
///     > If `uplo = 'U'`, the upper triangular part of `a` contains
///     > the upper triangular part of the matrix `a`, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the lower triangular part of `a` contains
///     > the lower triangular part of the matrix `a`, and the
///     > strictly upper triangular part of `a` is not referenced.
///
///     On exit, the tridiagonal matrix T and the multipliers used
///     to obtain the factor U or L (see below for further details).
/// * __*`ldim`*__ - (input) The leading dimension of the array `a`.
/// `ldim >= max(1,n)`.
/// * __*`ipiv`*__ - (output) Pointer to an array of dimension N.
/// Information about the permutation matrix P: row and column
/// i are interchanged with `ipiv[i-1]`. If `uplo = 'U'`, those
/// interchanges are done in the order `i = N ... 1`, if `uplo = 'L'`
/// in the order `i = 1 ... N`.
/// * __*`work`*__ - (workspace/output) Array of dimension `max(1,lwork)`.
/// On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The length of `work`.  `lwork >= 1`.
/// For best performance
/// `lwork >= n*NB`, where `NB` is the block size returned by ILAENV
/// (at the moment, uses the same block size as SSYTRF from Lapack).
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by
/// [XERBLA](https://netlib.org/lapack/explore-html/d0/d73/group__aux__blas_ga377ee61015baf8dea7770b3a404b1c07.html).
/// * __*`info`*__ - (output) Succes of the computation. If `info`
///   > = 0: successful exit,
///
///   > < 0: if `info = -k`, the k-th argument had an illegal value,
///
///   > \> 0: if `info = k`, the off-diagonal entry in the k-th row
///   > `uplo = 'U'` or k-th column `uplo = 'L'`
///   > is exactly zero.
///
///   > = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn csktrf(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [c32],
    ldim: &i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::csktrf_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        ldim,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}

/// Double precision complex
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) If `uplo = 'U'`, the upper triangular part of `a` contains
/// the upper triangular part of the matrix `a`, and the strictly lower triangular
/// part of `a` is not referenced. If `uplo = 'L'`, the lower triangular part of `a`
/// contains the lower triangular part of the matrix `a`, and the strictly upper
/// triangular part of `a` is not referenced.
/// * __*`mode`*__ - (input) If 'N':  `a` is fully tridiagonalized.
/// If 'P':  `a` is partially tridiagonalized for Pfaffian computation.
/// * __*`n`*__ - (input) Size of the matrix `a`. If `mode = 'P'`, `n` must be even. `n
/// >= 0`.
/// * __*`a`*__ - (input/output) Pointer to a memory block of size `n*n*sizeof(datatype)`.
/// On entry, the skew-symmetric `n`x`n`-matrix `a` in Fortran format.
///
///     > If `uplo = 'U'`, the upper triangular part of `a` contains
///     > the upper triangular part of the matrix `a`, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the lower triangular part of `a` contains
///     > the lower triangular part of the matrix `a`, and the
///     > strictly upper triangular part of `a` is not referenced.
///
///     On exit, the tridiagonal matrix T and the multipliers used
///     to obtain the factor U or L (see below for further details).
/// * __*`ldim`*__ - (input) The leading dimension of the array `a`.
/// `ldim >= max(1,n)`.
/// * __*`ipiv`*__ - (output) Pointer to an array of dimension N.
/// Information about the permutation matrix P: row and column
/// i are interchanged with `ipiv[i-1]`. If `uplo = 'U'`, those
/// interchanges are done in the order `i = N ... 1`, if `uplo = 'L'`
/// in the order `i = 1 ... N`.
/// * __*`work`*__ - (workspace/output) Array of dimension `max(1,lwork)`.
/// On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The length of `work`.  `lwork >= 1`.
/// For best performance
/// `lwork >= n*NB`, where `NB` is the block size returned by ILAENV
/// (at the moment, uses the same block size as SSYTRF from Lapack).
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by
/// [XERBLA](https://netlib.org/lapack/explore-html/d0/d73/group__aux__blas_ga377ee61015baf8dea7770b3a404b1c07.html).
/// * __*`info`*__ - (output) Succes of the computation. If `info`
///   > = 0: successful exit,
///
///   > < 0: if `info = -k`, the k-th argument had an illegal value,
///
///   > \> 0: if `info = k`, the off-diagonal entry in the k-th row
///   > `uplo = 'U'` or k-th column `uplo = 'L'`
///   > is exactly zero.
///
///   > = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn zsktrf(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [c64],
    ldim: &i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::zsktrf_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        ldim,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}
