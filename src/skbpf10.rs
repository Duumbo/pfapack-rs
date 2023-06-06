use crate::{c32, c64, c_char};

/// Single precision real.
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) if `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// * __*`n`*__ - (input) Size of the matrix A. `n >= 0`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Pointer to a memory block of size
/// `(kd+1)*n*sizeof(datatype)`.
///
///     > If `n` is odd: `ab` is not referenced.
///
///     > If `n` is even: On entry, A must contain a Fortran matrtix. The upper
///     > or lower triangle of the skew-symmetric band matrix A, stored in the
///     > first `kd+1` rows of the array. The j-th column of A is stored in the
///     > j-th column of the array `ab` as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j) = a(i,j)` for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j) = A(i,j)` for `j<=i<=min(n,j+kd)`.
///
///     On exit, `ab` is overwritten with values generated during the
///     computation.
///
/// * __*`ldab`*__ - (input) The leading dimension of the array A:
/// `ldab >= max(1, n)`.
/// * __*`pfaff`*__ - (output) Pointer to a an array `datatype[2]`The value of
/// the Pfaffian.
/// in the form `PFAFF[0] * (10^PFAFF[1])`.
/// * __*`work`*__ - (workspace) LAPACK work array.
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn sskbpf10(
    uplo: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [f32],
    ldab: &i32,
    pfaff: &mut f32,
    work: &mut [f32],
    info: &mut i32,
) {
    ffi::sskbpf10_(
        &(uplo as c_char),
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        pfaff,
        work.as_mut_ptr(),
        info,
    )
}

/// Double precision real.
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) if `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// * __*`n`*__ - (input) Size of the matrix A. `n >= 0`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Pointer to a memory block of size
/// `(kd+1)*n*sizeof(datatype)`.
///
///     > If `n` is odd: `ab` is not referenced.
///
///     > If `n` is even: On entry, A must contain a Fortran matrtix. The upper
///     > or lower triangle of the skew-symmetric band matrix A, stored in the
///     > first `kd+1` rows of the array. The j-th column of A is stored in the
///     > j-th column of the array `ab` as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j) = A(i,j)` for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j) = A(i,j)` for `j<=i<=min(n,j+kd)`.
///
///     On exit, `ab` is overwritten with values generated during the
///     computation.
///
/// * __*`ldab`*__ - (input) The leading dimension of the array A:
/// `ldab >= max(1, n)`.
/// * __*`pfaff`*__ - (output) Pointer to a an array `datatype[2]`The value of
/// the Pfaffian.
/// in the form `PFAFF[0] * (10^PFAFF[1])`.
/// * __*`work`*__ - (workspace) LAPACK work array.
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn dskbpf10(
    uplo: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [f64],
    ldab: &i32,
    pfaff: &mut f64,
    work: &mut [f64],
    info: &mut i32,
) {
    ffi::dskbpf10_(
        &(uplo as c_char),
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        pfaff,
        work.as_mut_ptr(),
        info,
    )
}

/// Single precision complex.
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) if `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// * __*`n`*__ - (input) Size of the matrix A. `n >= 0`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Pointer to a memory block of size
/// `(kd+1)*n*sizeof(datatype)`.
///
///     > If `n` is odd: `ab` is not referenced.
///
///     > If `n` is even: On entry, A must contain a Fortran matrtix. The upper
///     > or lower triangle of the skew-symmetric band matrix A, stored in the
///     > first `kd+1` rows of the array. The j-th column of A is stored in the
///     > j-th column of the array AB as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j) = A(i,j)` for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j) = A(i,j)` for `j<=i<=min(n,j+kd)`.
///
///     On exit, `ab` is overwritten with values generated during the
///     computation.
///
/// * __*`ldab`*__ - (input) The leading dimension of the array A:
/// `ldab >= max(1, n)`.
/// * __*`pfaff`*__ - (output) Pointer to a an array `datatype[2]`The value of
/// the Pfaffian.
/// in the form `PFAFF[0] * (10^PFAFF[1])`.
/// * __*`work`*__ - (workspace) LAPACK work array.
/// * __*`rwork`*__ - (workspace) Real LAPACK work array.
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn cskbpf10(
    uplo: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [c32],
    ldab: &i32,
    pfaff: *mut c32,
    work: &mut [c32],
    rwork: &mut [f32],
    info: &mut i32,
) {
    ffi::cskbpf10_(
        &(uplo as c_char),
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        pfaff,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}

/// Double precision complex.
///
/// # Arguments
///
/// * __*`uplo`*__ - (input) if `uplo = 'U'`: Upper triangle of A is stored;
/// if `uplo = 'L'`: Lower triangle of A is stored.
/// * __*`n`*__ - (input) Size of the matrix A. `n >= 0`.
/// * __*`kd`*__ - (input) Number of super- (if `uplo = 'U'`) or subdiagonals
/// (if `uplo = 'L'`). `kd >= 0`.
/// * __*`ab`*__ - (input/output) Pointer to a memory block of size
/// `(kd+1)*n*sizeof(datatype)`.
///
///     > If `n` is odd: `ab` is not referenced.
///
///     > If `n` is even: On entry, A must contain a Fortran matrtix. The upper
///     > or lower triangle of the skew-symmetric band matrix A, stored in the
///     > first `kd+1` rows of the array. The j-th column of A is stored in the
///     > j-th column of the array AB as follows:
///
///     > If `uplo = 'U'`, `ab(kd+1+i-j,j) = A(i,j)` for `max(1,j-kd)<=i<=j`;
///
///     > If `uplo = 'L'`, `ab(1+i-j,j) = A(i,j)` for `j<=i<=min(n,j+kd)`.
///
///     On exit, `ab` is overwritten with values generated during the
///     computation.
///
/// * __*`ldab`*__ - (input) The leading dimension of the array A:
/// `ldab >= max(1, n)`.
/// * __*`pfaff`*__ - (output) Pointer to a an array `datatype[2]`The value of
/// the Pfaffian.
/// in the form `PFAFF[0] * (10^PFAFF[1])`.
/// * __*`work`*__ - (workspace) LAPACK work array.
/// * __*`rwork`*__ - (workspace) Real LAPACK work array.
/// * __*`info`*__ - (output)
///
///     > If = 0: successful exit;
///
///     > If < 0: if the return value is -i, the i-th argument had an illegal
///     > value;
///
///     > If = -100: failed to allocate enough internal memory.
#[inline]
pub unsafe fn zskbpf10(
    uplo: u8,
    n: &i32,
    kd: &i32,
    ab: &mut [c64],
    ldab: &i32,
    pfaff: *mut c64,
    work: &mut [c64],
    rwork: &mut [f64],
    info: &mut i32,
) {
    ffi::zskbpf10_(
        &(uplo as c_char),
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        pfaff,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        info,
    )
}
