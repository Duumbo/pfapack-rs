use crate::{c32, c64, c_char};

/// Single precision real.
///
/// # Arguments
///
/// * __*`vect`*__ - (input) If `vect= 'N'`: do not form Q; If `vect = 'V':
/// form Q; if `vect = 'U': update a matrix X, by forming X*Q.
/// * __*`uplo`*__ - (input) If `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// __*`mode`*__ - (input) If `mode = 'N'`: A is fully triagonalized; if
/// `mode = 'P'`: A is partially triagonalized for Pfaffian computation
/// (details see below).
/// __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be even if
/// `mode = 'P'`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Dimension (`ldab, n`) On entry, the upper or
/// lower triangle of the skew-symmetric band matrix A, stored in the first
/// `kd+1` rows of the array. The j-th column of A is stored in the j-th column
/// of the array `ab` as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j)` = A(i,j) for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j)` = A(i,j) for `j<=i<=min(n,j+kd)`.
///
///     On exit, the zero diagonal elements of AB are left unchanged, if `kd > 0`,
///     the elements on the first superdiagonal (if `uplo = 'U'`) or the first
///     subdiagonal (if UPLO = 'L') are overwritten by the off-diagonal elements of
///     T; the rest of AB is overwritten by values generated during the reduction.
///     If `mode = 'P'`, only the off-diagonal entries in the odd rows (columns)
///     are computed for `uplo = 'U'` (`uplo = 'L'`).
/// * __*`ldab`*__ (input) The leading dimension of the array `ab`.
/// `ldab >= kd+1`.
/// * __*`e`*__ - (output) Dimension (`n-1`). The off-diagonal elements of the
/// tridiagonal matrix T: `e(i)` = T(i,i+1) if `uplo = 'U'`; `e(i)` = T(i+1,i)
/// if `uplo = 'L'`.
/// * __*`q`*__ - (input/output) Dimension (`ldq, n`). On entry, if `vect =`
///
///     > 'U', then Q must contain an `n`-by-`n` matrix X;
///
///     > 'N' or 'V', then Q need not be set.
///
///     On exit, it `vect =`
///
///     > 'V', Q contains the N-by-N unitary matrix Q;
///
///     > 'U', Q contains the product X*Q;
///
///     > 'N' , the array Q is not referenced.
///
/// __*`ldq`*__ - (input) The leading dimension of the array Q. `ldq >= 1`, and
/// `ldq >= n` if `vect = 'V' or 'U'`.
/// * __*`work`*__ - (workspace) Array, dimension (`n`).
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn sskbtrd(
    vect: u8,
    uplo: u8,
    mode: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [f32],
    ldab: &i32,
    e: &mut [f32],
    q: &mut [f32],
    ldq: &i32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sskbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &(mode as c_char),
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        ldq,
        work.as_mut_ptr(),
        info,
    )
}

/// Double precision real.
///
/// # Arguments
///
/// * __*`vect`*__ - (input) If `vect= 'N'`: do not form Q; If `vect = 'V':
/// form Q; if `vect = 'U': update a matrix X, by forming X*Q.
/// * __*`uplo`*__ - (input) If `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// __*`mode`*__ - (input) If `mode = 'N'`: A is fully triagonalized; if
/// `mode = 'P'`: A is partially triagonalized for Pfaffian computation
/// (details see below).
/// __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be even if
/// `mode = 'P'`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Dimension (`ldab, n`) On entry, the upper or
/// lower triangle of the skew-symmetric band matrix A, stored in the first
/// `kd+1` rows of the array. The j-th column of A is stored in the j-th column
/// of the array `ab` as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j)` = A(i,j) for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j)` = A(i,j) for `j<=i<=min(n,j+kd)`.
///
///     On exit, the zero diagonal elements of AB are left unchanged, if `kd > 0`,
///     the elements on the first superdiagonal (if `uplo = 'U'`) or the first
///     subdiagonal (if UPLO = 'L') are overwritten by the off-diagonal elements of
///     T; the rest of AB is overwritten by values generated during the reduction.
///     If `mode = 'P'`, only the off-diagonal entries in the odd rows (columns)
///     are computed for `uplo = 'U'` (`uplo = 'L'`).
/// * __*`ldab`*__ (input) The leading dimension of the array `ab`.
/// `ldab >= kd+1`.
/// * __*`e`*__ - (output) Dimension (`n-1`). The off-diagonal elements of the
/// tridiagonal matrix T: `e(i)` = T(i,i+1) if `uplo = 'U'`; `e(i)` = T(i+1,i)
/// if `uplo = 'L'`.
/// * __*`q`*__ - (input/output) Dimension (`ldq, n`). On entry, if `vect`
///
///     > = 'U', then Q must contain an `n`-by-`n` matrix X;
///
///     > = 'N' or 'V', then Q need not be set.
///
///     On exit, it `vect`
///
///     > = 'V', Q contains the N-by-N unitary matrix Q;
///
///     > = 'U', Q contains the product X*Q;
///
///     > = 'N' , the array Q is not referenced.
///
/// __*`ldq`*__ - (input) The leading dimension of the array Q. `ldq >= 1`, and
/// `ldq >= n` if `vect = 'V' or 'U'`.
/// * __*`work`*__ - (workspace) Array, dimension (`n`).
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn dskbtrd(
    vect: u8,
    uplo: u8,
    mode: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [f64],
    ldab: &i32,
    e: &mut [f64],
    q: &mut [f64],
    ldq: &i32,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dskbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &(mode as c_char),
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        e.as_mut_ptr(),
        q.as_mut_ptr(),
        ldq,
        work.as_mut_ptr(),
        info,
    )
}

/// Single precision complex.
///
/// # Arguments
///
/// * __*`vect`*__ - (input) If `vect= 'N'`: do not form Q; If `vect = 'V':
/// form Q; if `vect = 'U': update a matrix X, by forming X*Q.
/// * __*`uplo`*__ - (input) If `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// __*`mode`*__ - (input) If `mode = 'N'`: A is fully triagonalized; if
/// `mode = 'P'`: A is partially triagonalized for Pfaffian computation
/// (details see below).
/// __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be even if
/// `mode = 'P'`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Dimension (`ldab, n`) On entry, the upper or
/// lower triangle of the skew-symmetric band matrix A, stored in the first
/// `kd+1` rows of the array. The j-th column of A is stored in the j-th column
/// of the array `ab` as follows:
///
///     > if `uplo = 'U'`, `ab(kd+1+i-j,j)` = A(i,j) for `max(1,j-kd)<=i<=j`;
///
///     > if `uplo = 'L'`, `ab(1+i-j,j)` = A(i,j) for `j<=i<=min(n,j+kd)`.
///
///     On exit, the zero diagonal elements of AB are left unchanged, if `kd > 0`,
///     the elements on the first superdiagonal (if `uplo = 'U'`) or the first
///     subdiagonal (if UPLO = 'L') are overwritten by the off-diagonal elements of
///     T; the rest of AB is overwritten by values generated during the reduction.
///     If `mode = 'P'`, only the off-diagonal entries in the odd rows (columns)
///     are computed for `uplo = 'U'` (`uplo = 'L'`).
/// * __*`ldab`*__ (input) The leading dimension of the array `ab`.
/// `ldab >= kd+1`.
/// * __*`e`*__ - (output) Dimension (`n-1`). The off-diagonal elements of the
/// tridiagonal matrix T: `e(i)` = T(i,i+1) if `uplo = 'U'`; `e(i)` = T(i+1,i)
/// if `uplo = 'L'`.
/// * __*`detq`*__ - (output) The value of the determinant of Q, which is a pure
/// phase factor. Always computed, even if Q is not explicitely formed.
/// * __*`q`*__ - (input/output) Dimension (`ldq, n`). On entry, if `vect`
///
///     > = 'U', then Q must contain an `n`-by-`n` matrix X;
///
///     > = 'N' or 'V', then Q need not be set.
///
///     On exit, it `vect`
///
///     > = 'V', Q contains the N-by-N unitary matrix Q;
///
///     > = 'U', Q contains the product X*Q;
///
///     > = 'N' , the array Q is not referenced.
///
/// __*`ldq`*__ - (input) The leading dimension of the array Q. `ldq >= 1`, and
/// `ldq >= n` if `vect = 'V' or 'U'`.
/// * __*`work`*__ - (workspace) Array, dimension (`n`).
/// * __*`rwork`*__ - (workspace) Array, dimension (`n`).
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn cskbtrd(
    vect: u8,
    uplo: u8,
    mode: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [c32],
    ldab: &[i32],
    e: &mut [f32],
    detq: &mut [c32],
    q: &mut [c32],
    ldq: &i32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cskbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &(mode as c_char),
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab.as_ptr(),
        e.as_mut_ptr(),
        detq.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

/// Double precision complex.
///
/// # Arguments
///
/// * __*`vect`*__ - (input) If `vect= 'N'`: do not form Q; If `vect = 'V':
/// form Q; if `vect = 'U': update a matrix X, by forming X*Q.
/// * __*`uplo`*__ - (input) If `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// __*`mode`*__ - (input) If `mode = 'N'`: A is fully triagonalized; if
/// `mode = 'P'`: A is partially triagonalized for Pfaffian computation
/// (details see below).
/// __*`n`*__ - (input) The order of the matrix A. `n >= 0`. `n` must be even if
/// `mode = 'P'`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Dimension (`ldab, n`) On entry, the upper or
/// lower triangle of the skew-symmetric band matrix A, stored in the first
/// `kd+1` rows of the array. The j-th column of A is stored in the j-th column
/// of the array `ab` as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j)` = A(i,j) for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j)` = A(i,j) for `j<=i<=min(n,j+kd)`.
///
///     On exit, the zero diagonal elements of AB are left unchanged, if `kd > 0`,
///     the elements on the first superdiagonal (if `uplo = 'U'`) or the first
///     subdiagonal (if UPLO = 'L') are overwritten by the off-diagonal elements of
///     T; the rest of AB is overwritten by values generated during the reduction.
///     If `mode = 'P'`, only the off-diagonal entries in the odd rows (columns)
///     are computed for `uplo = 'U'` (`uplo = 'L'`).
/// * __*`ldab`*__ (input) The leading dimension of the array `ab`.
/// `ldab >= kd+1`.
/// * __*`e`*__ - (output) Dimension (`n-1`). The off-diagonal elements of the
/// tridiagonal matrix T: `e(i)` = T(i,i+1) if `uplo = 'U'`; `e(i)` = T(i+1,i)
/// if `uplo = 'L'`.
/// * __*`detq`*__ - (output) The value of the determinant of Q, which is a pure
/// phase factor. Always computed, even if Q is not explicitely formed.
/// * __*`q`*__ - (input/output) Dimension (`ldq, n`). On entry, if `vect`
///
///     > = 'U', then Q must contain an `n`-by-`n` matrix X;
///
///     > = 'N' or = 'V', then Q need not be set.
///
///     On exit, it `vect`
///
///     > = 'V', Q contains the N-by-N unitary matrix Q;
///
///     > = 'U', Q contains the product X*Q;
///
///     > = 'N' , the array Q is not referenced.
///
/// __*`ldq`*__ - (input) The leading dimension of the array Q. `ldq >= 1`, and
/// `ldq >= n` if `vect = 'V' or 'U'`.
/// * __*`work`*__ - (workspace) Array, dimension (`n`).
/// * __*`rwork`*__ - (workspace) Array, dimension (`n`).
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn zskbtrd(
    vect: u8,
    uplo: u8,
    mode: u8,
    n: &i32,
    kd: &i32,
    a: &mut [c64],
    ldab: &i32,
    e: &mut [f64],
    detq: &mut [c64],
    q: &mut [c64],
    ldq: &i32,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zskbtrd_(
        &(vect as c_char),
        &(uplo as c_char),
        &(mode as c_char),
        n,
        kd,
        a.as_mut_ptr() as *mut _,
        ldab,
        e.as_mut_ptr(),
        detq.as_mut_ptr() as *mut _,
        q.as_mut_ptr() as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}
