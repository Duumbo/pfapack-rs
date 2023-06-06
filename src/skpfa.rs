use crate::{c_char, c32, c64};

/// Real skew-symmetric matrix, single precision.
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///
///     > 'U':  Upper triangle of A is stored;
///
///     > 'L':  Lower triangle of A is stored.
///
/// * __*`mthd`*__ - (input)
///
///     > 'P': Compute Pfaffian using Parlett-Reid algorithm (recommended)
///
///     > 'H': Compute Pfaffian using Householder reflections
///
/// * __*`n`*__ - (input) The order of the matrix `a`.  `n >= 0`.
///
/// * __*`a`*__ - (input/output) dimension `(lda,n)`. On entry, the skew-symmetric
/// matrix A.
///
///     > If UPLO = 'U', the upper triangular part of A contains
///     > the upper triangular part of the matrix A, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If UPLO = 'L', the lower triangular part of A contains
///     > the lower triangular part of the matrix A, and the
///     > strictly upper triangular part of A is not referenced.
///
///     If the matrix size is odd, A is not referenced. If the matrix
///     size is even, A is overwritten by values generated during
///     the computation.
///
/// * __*`lda`*__ - (input) The leading dimension of the array `a`. `lda >= max(1,N)`.
///
/// * __*`pfaff`*__ - (output) The value of the Pfaffian.
///
/// * __*`iwork`*__ - (workspace) dimension (N). Not referenced if `mthd = 'H'`.
///
/// * __*`work`*__ - (workspace) array
///     > dimension `max(1, lwork)`, if `mthd = 'P'`;
///
///     > dimension `max(n,lwork)`, if `mthd = 'H'`.
///
///     On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
///
/// * __*`lwork`*__ - (input) The dimension of the array `work`.
///     > If `mthd = 'P'`, `lwork >= 1`,
///
///     > If `mthd = 'H'`, `lwork >= n`.
///
///     For optimum performance `lwork >= n*NB` for `mthd = 'P'` or
///     `lwork >= n*NB+n-1` for `mthd = 'H'`, where NB is the
///     optimal blocksize.
///
///     If `lwork = -1`, then a workspace query is assumed; the routine
///     only calculates the optimal size of the `work` array, returns
///     this value as the first entry of the `work` array, and no error
///     message related to `lwork` is issued by XERBLA.
///
/// * __*`info`*__ - (output) If `info`
///     > = 0:  successful exit
///
///     > < 0:  if `info = -i`, the `i`-th argument had an illegal value
#[inline]
pub unsafe fn sskpfa(
    uplo: u8,
    mthd: u8,
    n: &i32,
    a: &mut [f32],
    lda: &i32,
    pfaff: &mut f32,
    iwork: &mut [i32],
    work: &mut [f32],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::sskpfa_(
        &(uplo as c_char),
        &(mthd as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        pfaff,
        iwork.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}

/// Real skew-symmetric matrix, double precision.
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///
///     > 'U':  Upper triangle of A is stored;
///
///     > 'L':  Lower triangle of A is stored.
///
/// * __*`mthd`*__ - (input)
///
///     > 'P': Compute Pfaffian using Parlett-Reid algorithm (recommended)
///
///     > 'H': Compute Pfaffian using Householder reflections
///
/// * __*`n`*__ - (input) The order of the matrix `a`.  `n >= 0`.
///
/// * __*`a`*__ - (input/output) dimension `(lda,n)`. On entry, the skew-symmetric
/// matrix A.
///
///     > If UPLO = 'U', the upper triangular part of A contains
///     > the upper triangular part of the matrix A, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If UPLO = 'L', the lower triangular part of A contains
///     > the lower triangular part of the matrix A, and the
///     > strictly upper triangular part of A is not referenced.
///
///     If the matrix size is odd, A is not referenced. If the matrix
///     size is even, A is overwritten by values generated during
///     the computation.
///
/// * __*`lda`*__ - (input) The leading dimension of the array `a`. `lda >= max(1,N)`.
///
/// * __*`pfaff`*__ - (output) The value of the Pfaffian.
///
/// * __*`iwork`*__ - (workspace) dimension (N). Not referenced if `mthd = 'H'`.
///
/// * __*`work`*__ - (workspace) array
///     > dimension `max(1, lwork)`, if `mthd = 'P'`;
///
///     > dimension `max(n,lwork)`, if `mthd = 'H'`.
///
///     On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
///
/// * __*`lwork`*__ - (input) The dimension of the array `work`.
///     > If `mthd = 'P'`, `lwork >= 1`,
///
///     > If `mthd = 'H'`, `lwork >= n`.
///
///     For optimum performance `lwork >= n*NB` for `mthd = 'P'` or
///     `lwork >= n*NB+n-1` for `mthd = 'H'`, where NB is the
///     optimal blocksize.
///
///     If `lwork = -1`, then a workspace query is assumed; the routine
///     only calculates the optimal size of the `work` array, returns
///     this value as the first entry of the `work` array, and no error
///     message related to `lwork` is issued by XERBLA.
///
/// * __*`info`*__ - (output) If `info`
///     > = 0:  successful exit
///
///     > < 0:  if `info = -i`, the `i`-th argument had an illegal value
#[inline]
pub unsafe fn dskpfa(
    uplo: u8,
    mthd: u8,
    n: &i32,
    a: &mut [f64],
    lda: &i32,
    pfaff: &mut f64,
    iwork: &mut [i32],
    work: &mut [f64],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::dskpfa_(
        &(uplo as c_char),
        &(mthd as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        pfaff,
        iwork.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}

/// Complex skew-symmetric matrix, single precision.
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///
///     > 'U':  Upper triangle of A is stored;
///
///     > 'L':  Lower triangle of A is stored.
///
/// * __*`mthd`*__ - (input)
///
///     > 'P': Compute Pfaffian using Parlett-Reid algorithm (recommended)
///
///     > 'H': Compute Pfaffian using Householder reflections
///
/// * __*`n`*__ - (input) The order of the matrix `a`.  `n >= 0`.
///
/// * __*`a`*__ - (input/output) dimension `(lda,n)`. On entry, the skew-symmetric
/// matrix A.
///
///     > If UPLO = 'U', the upper triangular part of A contains
///     > the upper triangular part of the matrix A, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If UPLO = 'L', the lower triangular part of A contains
///     > the lower triangular part of the matrix A, and the
///     > strictly upper triangular part of A is not referenced.
///
///     If the matrix size is odd, A is not referenced. If the matrix
///     size is even, A is overwritten by values generated during
///     the computation.
///
/// * __*`lda`*__ - (input) The leading dimension of the array `a`. `lda >= max(1,N)`.
///
/// * __*`pfaff`*__ - (output) The value of the Pfaffian.
///
/// * __*`iwork`*__ - (workspace) dimension (N). Not referenced if `mthd = 'H'`.
///
/// * __*`work`*__ - (workspace) array
///     > dimension `max(1, lwork)`, if `mthd = 'P'`;
///
///     > dimension `max(n,lwork)`, if `mthd = 'H'`.
///
///     On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
///
/// * __*`lwork`*__ - (input) The dimension of the array `work`.
///     > If `mthd = 'P'`, `lwork >= 1`,
///
///     > If `mthd = 'H'`, `lwork >= n`.
///
///     For optimum performance `lwork >= n*NB` for `mthd = 'P'` or
///     `lwork >= n*NB+n-1` for `mthd = 'H'`, where NB is the
///     optimal blocksize.
///
///     If `lwork = -1`, then a workspace query is assumed; the routine
///     only calculates the optimal size of the `work` array, returns
///     this value as the first entry of the `work` array, and no error
///     message related to `lwork` is issued by XERBLA.
///
/// * __*`rwork`*__ - (workspace) dimension (N-1). Not referenced if `mthd = 'P'`.
///
/// * __*`info`*__ - (output) If `info`
///     > = 0:  successful exit
///
///     > < 0:  if `info = -i`, the `i`-th argument had an illegal value
#[inline]
pub unsafe fn cskpfa(
    uplo: u8,
    mthd: u8,
    n: &i32,
    a: &mut [c32],
    lda: &i32,
    pfaff: *mut c32,
    iwork: &mut [i32],
    work: &mut [c32],
    lwork: &i32,
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cskpfa_(
        &(uplo as c_char),
        &(mthd as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        pfaff,
        iwork.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        info,
    )
}

/// Complex skew-symmetric matrix, double precision.
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///
///     > 'U':  Upper triangle of A is stored;
///
///     > 'L':  Lower triangle of A is stored.
///
/// * __*`mthd`*__ - (input)
///
///     > 'P': Compute Pfaffian using Parlett-Reid algorithm (recommended)
///
///     > 'H': Compute Pfaffian using Householder reflections
///
/// * __*`n`*__ - (input) The order of the matrix `a`.  `n >= 0`.
///
/// * __*`a`*__ - (input/output) dimension `(lda,n)`. On entry, the skew-symmetric
/// matrix A.
///
///     > If UPLO = 'U', the upper triangular part of A contains
///     > the upper triangular part of the matrix A, and the
///     > strictly lower triangular part of A is not referenced.
///
///     > If UPLO = 'L', the lower triangular part of A contains
///     > the lower triangular part of the matrix A, and the
///     > strictly upper triangular part of A is not referenced.
///
///     If the matrix size is odd, A is not referenced. If the matrix
///     size is even, A is overwritten by values generated during
///     the computation.
///
/// * __*`lda`*__ - (input) The leading dimension of the array `a`. `lda >= max(1,N)`.
///
/// * __*`pfaff`*__ - (output) The value of the Pfaffian.
///
/// * __*`iwork`*__ - (workspace) dimension (N). Not referenced if `mthd = 'H'`.
///
/// * __*`work`*__ - (workspace) array
///     > dimension `max(1, lwork)`, if `mthd = 'P'`;
///
///     > dimension `max(n,lwork)`, if `mthd = 'H'`.
///
///     On exit, if `info = 0`, `work[0]` returns the optimal `lwork`.
///
/// * __*`lwork`*__ - (input) The dimension of the array `work`.
///     > If `mthd = 'P'`, `lwork >= 1`,
///
///     > If `mthd = 'H'`, `lwork >= n`.
///
///     For optimum performance `lwork >= n*NB` for `mthd = 'P'` or
///     `lwork >= n*NB+n-1` for `mthd = 'H'`, where NB is the
///     optimal blocksize.
///
///     If `lwork = -1`, then a workspace query is assumed; the routine
///     only calculates the optimal size of the `work` array, returns
///     this value as the first entry of the `work` array, and no error
///     message related to `lwork` is issued by XERBLA.
///
/// * __*`rwork`*__ - (workspace) dimension (N-1). Not referenced if `mthd = 'P'`.
///
/// * __*`info`*__ - (output) If `info`
///     > = 0:  successful exit
///
///     > < 0:  if `info = -i`, the `i`-th argument had an illegal value
#[inline]
pub unsafe fn zskpfa(
    uplo: u8,
    mthd: u8,
    n: &i32,
    a: &mut [c64],
    lda: &i32,
    pfaff: *mut c64,
    iwork: &mut [i32],
    work: &mut [c64],
    lwork: &i32,
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zskpfa_(
        &(uplo as c_char),
        &(mthd as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        pfaff,
        iwork.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        info,
    )
}
