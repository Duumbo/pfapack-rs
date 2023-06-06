use crate::{c_char, c32, c64};

///  Single precision real
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///         > 'U':  Upper triangle of A is stored;
///         > 'L':  Lower triangle of A is stored.
/// * __*`mode`*__ - (input)
///         > 'N':  A is fully tridiagonalized
///         > 'P':  A is partially tridiagonalized for Pfaffian computation
///         > (details see below)
/// * __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be
/// even if `mode = 'P'`.
/// * __*`a`*__ - (input/output) dimension (LDA,N). On entry, the skew-symmetric
/// matrix A.
///     > If `uplo = 'U'`, the leading N-by-N upper triangular part
///     > of A contains the upper triangular part of the matrix A,
///     > and the strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the leading N-by-N lower triangular part
///     > of A contains the lower triangular part of the matrix A,
///     > and the strictly upper triangular part of A is not referenced.
///
///     On exit, if `mode = 'N'`:
///     > If UPLO = 'U', the diagonal and first superdiagonal
///     > of A are overwritten by the corresponding elements of the
///     > tridiagonal matrix T, and the elements above the first
///     > superdiagonal, with the array TAU, represent the unitary
///     > matrix Q as a product of elementary reflectors;
///     > If `uplo = 'L'`, the diagonal and first subdiagonal of A are over-
///     > written by the corresponding elements of the tridiagonal
///     > matrix T, and the elements below the first subdiagonal, with
///     > the array TAU, represent the unitary matrix Q as a product
///     > of elementary reflectors.
///     See Further Details, also for information about `mode = 'P'`.
/// * __*`lda`*__ - (input) The leading dimension of the array A.
/// `lda >= max(1,N)`.
/// * __*`e`*__ - (output) dimension (N-1). The off-diagonal elements of the
/// tridiagonal matrix T:
///     > If `uplo = 'U'`, `e[i] = a[i,i+1]`.
///
///     > If `uplo = 'L'`, `e[i] = a[i+1,i]`.
///     > If `mode = 'P'`, only the entries at i odd are well-defined
///     > (see Further Details).
/// * __*`tau`*__ - (output) dimension (N-1). The scalar factors of the
/// elementary reflectors (see Further Details).
/// * __*`work`*__ - (workspace/output) dimension (MAX(1,LWORK)). On exit, if
/// `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The dimension of the array `work`. `lwork >= 1`.
/// For optimum performance LWORK >= N*NB, where NB is the optimal blocksize.
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by XERBLA.
/// * __*`info`*__ - (output) If `info`
/// > = 0:  successful exit
///
/// > < 0:  if INFO = -i, the i-th argument had an illegal value
#[inline]
pub unsafe fn ssktrd(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [f32],
    lda: &i32,
    e: &mut [f32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::ssktrd_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        info
    )
}

///  Double precision real
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///         > 'U':  Upper triangle of A is stored;
///         > 'L':  Lower triangle of A is stored.
/// * __*`mode`*__ - (input)
///         > 'N':  A is fully tridiagonalized
///         > 'P':  A is partially tridiagonalized for Pfaffian computation
///         > (details see below)
/// * __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be
/// even if `mode = 'P'`.
/// * __*`a`*__ - (input/output) dimension (LDA,N). On entry, the skew-symmetric
/// matrix A.
///     > If `uplo = 'U'`, the leading N-by-N upper triangular part
///     > of A contains the upper triangular part of the matrix A,
///     > and the strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the leading N-by-N lower triangular part
///     > of A contains the lower triangular part of the matrix A,
///     > and the strictly upper triangular part of A is not referenced.
///
///     On exit, if `mode = 'N'`:
///     > If UPLO = 'U', the diagonal and first superdiagonal
///     > of A are overwritten by the corresponding elements of the
///     > tridiagonal matrix T, and the elements above the first
///     > superdiagonal, with the array TAU, represent the unitary
///     > matrix Q as a product of elementary reflectors;
///     > If `uplo = 'L'`, the diagonal and first subdiagonal of A are over-
///     > written by the corresponding elements of the tridiagonal
///     > matrix T, and the elements below the first subdiagonal, with
///     > the array TAU, represent the unitary matrix Q as a product
///     > of elementary reflectors.
///     See Further Details, also for information about `mode = 'P'`.
/// * __*`lda`*__ - (input) The leading dimension of the array A.
/// `lda >= max(1,N)`.
/// * __*`e`*__ - (output) dimension (N-1). The off-diagonal elements of the
/// tridiagonal matrix T:
///     > If `uplo = 'U'`, `e[i] = a[i,i+1]`.
///
///     > If `uplo = 'L'`, `e[i] = a[i+1,i]`.
///     > If `mode = 'P'`, only the entries at i odd are well-defined
///     > (see Further Details).
/// * __*`tau`*__ - (output) dimension (N-1). The scalar factors of the
/// elementary reflectors (see Further Details).
/// * __*`work`*__ - (workspace/output) dimension (MAX(1,LWORK)). On exit, if
/// `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The dimension of the array `work`. `lwork >= 1`.
/// For optimum performance LWORK >= N*NB, where NB is the optimal blocksize.
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by XERBLA.
/// * __*`info`*__ - (output) If `info`
/// > = 0:  successful exit
///
/// > < 0:  if INFO = -i, the i-th argument had an illegal value
#[inline]
pub unsafe fn dsktrd(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [f64],
    lda: &i32,
    e: &mut [f64],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::dsktrd_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        info
    )
}

/// Single precision complex
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///         > 'U':  Upper triangle of A is stored;
///         > 'L':  Lower triangle of A is stored.
/// * __*`mode`*__ - (input)
///         > 'N':  A is fully tridiagonalized
///         > 'P':  A is partially tridiagonalized for Pfaffian computation
///         > (details see below)
/// * __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be
/// even if `mode = 'P'`.
/// * __*`a`*__ - (input/output) dimension (LDA,N). On entry, the skew-symmetric
/// matrix A.
///     > If `uplo = 'U'`, the leading N-by-N upper triangular part
///     > of A contains the upper triangular part of the matrix A,
///     > and the strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the leading N-by-N lower triangular part
///     > of A contains the lower triangular part of the matrix A,
///     > and the strictly upper triangular part of A is not referenced.
///
///     On exit, if `mode = 'N'`:
///     > If UPLO = 'U', the diagonal and first superdiagonal
///     > of A are overwritten by the corresponding elements of the
///     > tridiagonal matrix T, and the elements above the first
///     > superdiagonal, with the array TAU, represent the unitary
///     > matrix Q as a product of elementary reflectors;
///     > If `uplo = 'L'`, the diagonal and first subdiagonal of A are over-
///     > written by the corresponding elements of the tridiagonal
///     > matrix T, and the elements below the first subdiagonal, with
///     > the array TAU, represent the unitary matrix Q as a product
///     > of elementary reflectors.
///     See Further Details, also for information about `mode = 'P'`.
/// * __*`lda`*__ - (input) The leading dimension of the array A.
/// `lda >= max(1,N)`.
/// * __*`e`*__ - (output) dimension (N-1). The off-diagonal elements of the
/// tridiagonal matrix T:
///     > If `uplo = 'U'`, `e[i] = a[i,i+1]`.
///
///     > If `uplo = 'L'`, `e[i] = a[i+1,i]`.
///     > If `mode = 'P'`, only the entries at i odd are well-defined
///     > (see Further Details).
/// * __*`tau`*__ - (output) dimension (N-1). The scalar factors of the
/// elementary reflectors (see Further Details).
/// * __*`work`*__ - (workspace/output) dimension (MAX(1,LWORK)). On exit, if
/// `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The dimension of the array `work`. `lwork >= 1`.
/// For optimum performance LWORK >= N*NB, where NB is the optimal blocksize.
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by XERBLA.
/// * __*`info`*__ - (output) If `info`
/// > = 0:  successful exit
///
/// > < 0:  if INFO = -i, the i-th argument had an illegal value
#[inline]
pub unsafe fn csktrd(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [c32],
    lda: &i32,
    e: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::csktrd_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}


/// Double precision complex
/// # Arguments
///
/// * __*`uplo`*__ - (input)
///         > 'U':  Upper triangle of A is stored;
///         > 'L':  Lower triangle of A is stored.
/// * __*`mode`*__ - (input)
///         > 'N':  A is fully tridiagonalized
///         > 'P':  A is partially tridiagonalized for Pfaffian computation
///         > (details see below)
/// * __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be
/// even if `mode = 'P'`.
/// * __*`a`*__ - (input/output) dimension (LDA,N). On entry, the skew-symmetric
/// matrix A.
///     > If `uplo = 'U'`, the leading N-by-N upper triangular part
///     > of A contains the upper triangular part of the matrix A,
///     > and the strictly lower triangular part of A is not referenced.
///
///     > If `uplo = 'L'`, the leading N-by-N lower triangular part
///     > of A contains the lower triangular part of the matrix A,
///     > and the strictly upper triangular part of A is not referenced.
///
///     On exit, if `mode = 'N'`:
///     > If UPLO = 'U', the diagonal and first superdiagonal
///     > of A are overwritten by the corresponding elements of the
///     > tridiagonal matrix T, and the elements above the first
///     > superdiagonal, with the array TAU, represent the unitary
///     > matrix Q as a product of elementary reflectors;
///     > If `uplo = 'L'`, the diagonal and first subdiagonal of A are over-
///     > written by the corresponding elements of the tridiagonal
///     > matrix T, and the elements below the first subdiagonal, with
///     > the array TAU, represent the unitary matrix Q as a product
///     > of elementary reflectors.
///     See Further Details, also for information about `mode = 'P'`.
/// * __*`lda`*__ - (input) The leading dimension of the array A.
/// `lda >= max(1,N)`.
/// * __*`e`*__ - (output) dimension (N-1). The off-diagonal elements of the
/// tridiagonal matrix T:
///     > If `uplo = 'U'`, `e[i] = a[i,i+1]`.
///
///     > If `uplo = 'L'`, `e[i] = a[i+1,i]`.
///     > If `mode = 'P'`, only the entries at i odd are well-defined
///     > (see Further Details).
/// * __*`tau`*__ - (output) dimension (N-1). The scalar factors of the
/// elementary reflectors (see Further Details).
/// * __*`work`*__ - (workspace/output) dimension (MAX(1,LWORK)). On exit, if
/// `info = 0`, `work[0]` returns the optimal `lwork`.
/// * __*`lwork`*__ - (input) The dimension of the array `work`. `lwork >= 1`.
/// For optimum performance LWORK >= N*NB, where NB is the optimal blocksize.
/// If `lwork = -1`, then a workspace query is assumed; the routine
/// only calculates the optimal size of the `work` array, returns
/// this value as the first entry of the `work` array, and no error
/// message related to `lwork` is issued by XERBLA.
/// * __*`info`*__ - (output) If `info`
/// > = 0:  successful exit
///
/// > < 0:  if INFO = -i, the i-th argument had an illegal value
#[inline]
pub unsafe fn zsktrd(
    uplo: u8,
    mode: u8,
    n: &i32,
    a: &mut [c64],
    lda: &i32,
    e: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: &i32,
    info: &mut i32,
) {
    ffi::zsktrd_(
        &(uplo as c_char),
        &(mode as c_char),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        info,
    )
}
