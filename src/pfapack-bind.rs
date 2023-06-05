#[inline]
pub unsafe fn ssktrf(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [i32],
    arg7: &mut [f32],
    arg8: &[i32],
    arg9: &mut [i32],
) {
    ffi::ssktrf_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsktrf(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [i32],
    arg7: &mut [f64],
    arg8: &[i32],
    arg9: &mut [i32],
) {
    ffi::dsktrf_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csktrf(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [i32],
    arg7: &mut [c32],
    arg8: &[i32],
    arg9: &mut [i32],
) {
    ffi::csktrf_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr() as *mut _,
        arg8.as_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsktrf(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [i32],
    arg7: &mut [c64],
    arg8: &[i32],
    arg9: &mut [i32],
) {
    ffi::zsktrf_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr() as *mut _,
        arg8.as_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssktrd(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [f32],
    arg8: &mut [f32],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::ssktrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsktrd(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [f64],
    arg8: &mut [f64],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::dsktrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csktrd(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [c32],
    arg8: &mut [c32],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::csktrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsktrd(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [c64],
    arg8: &mut [c64],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::zsktrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sskpfa(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [i32],
    arg8: &mut [f32],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::sskpfa_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dskpfa(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [i32],
    arg8: &mut [f64],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::dskpfa_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cskpfa(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [c32],
    arg7: &mut [i32],
    arg8: &mut [c32],
    arg9: &[i32],
    arg10: &mut [f32],
    arg11: &mut [i32],
) {
    ffi::cskpfa_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
        arg11.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zskpfa(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [c64],
    arg7: &mut [i32],
    arg8: &mut [c64],
    arg9: &[i32],
    arg10: &mut [f64],
    arg11: &mut [i32],
) {
    ffi::zskpfa_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
        arg11.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sskpf10(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [i32],
    arg8: &mut [f32],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::sskpf10_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dskpf10(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [i32],
    arg8: &mut [f64],
    arg9: &[i32],
    arg10: &mut [i32],
) {
    ffi::dskpf10_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cskpf10(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [c32],
    arg7: &mut [i32],
    arg8: &mut [c32],
    arg9: &[i32],
    arg10: &mut [f32],
    arg11: &mut [i32],
) {
    ffi::cskpf10_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
        arg11.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zskpf10(
    arg1: u8,
    arg2: u8,
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [c64],
    arg7: &mut [i32],
    arg8: &mut [c64],
    arg9: &[i32],
    arg10: &mut [f64],
    arg11: &mut [i32],
) {
    ffi::zskpf10_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr() as *mut _,
        arg9.as_ptr(),
        arg10.as_mut_ptr(),
        arg11.as_mut_ptr(),
    )
}

/// Reduces a complex skew-symmetric band matrix A to real
/// skew-symmetric tridiagonal form T by a unitary congruence transformation.
///
/// # Purpose
///
/// `zskbtrd` reduces a complex skew-symmetric band matrix A to real
/// skew-symmetric tridiagonal form T by a unitary congruence transformation:
/// Q^dagger * A * Q^* = T. Alternatively, the routine can also compute a
/// partial tridiagonal form useful for computing the Pfaffian.
///
/// # Further Details
///
/// The storage scheme for the skew-symmetric matrix is identical to the
/// storage scheme for symmetric or Hermitian band matrices in LAPACK,
/// i.e. the diagonal and the `kd` super- or subdiagonals are stored in an
/// array with `kd+1` rows and `n` columns. Note that the zero diagonal must
/// also be explicitely stored (this was done to keep the structure of
/// the program identical to the symmetric case)
///
/// In particular this means that if
/// - `uplo = 'U'`, then `ab(kd+1+i-j,j)` = A(i,j) for `max(1,j-kd)<=i<=j`
///
///   Example: `n=5`, `kd=2`
///
/// (  0     a12   a13              )
/// (  -a12  0     a23   a24        )
/// (  -a13  -a23  0     a34   a35  )
/// (        -a24  -a34  0     a45  )
/// (              -a35  -a45  0    )
///
/// is stored as
///
///  x   x   a13 a24 a35
///  x   a12 a23 a34 a45
///  0   0   0   0   0
///
/// where x denotes an unused entry
///
/// - `uplo = 'L'`, then `ab(1+i-j,j)` = A(i,j) for `j<=i<=min(n,j+kd)`
///
///   Example: `n=5`, `kd=2`
///
/// (  0     -a21  -a31              )
/// (  a21   0     -a32  -a42        )
/// (  a31   a32   0     -a43  -a53  )
/// (        a42   a43   0     -a54  )
/// (              a53   a54   0     )
///
/// is stored as
///
///  0   0   0   0   0
///  a21 a32 a43 a54 x
///  a31 a42 a53 x   x
///
/// where x denotes an unused entry
pub mod skbtrd;

/// Computes the Pfaffian of a banded skew-symmetric matrix.
///
/// # Purpose
///
/// `skbpfa` computes the Pfaffian of a banded skew-symmetric matrix.
///
/// # Further details
///
/// For odd-sized matrices, the Pfaffian is 0 by default, hence no computation
/// is needed in this case. For even-sized matrices, the Pfaffian is computed
/// by bringing the skew-symmetric matrix A into tridiagonal form T by a
/// unitary congruence transformation: `Q^H * A * Q^* = T`. This transformation
/// is computed by the routine [`ssktrd`] (for further details see there).
pub mod skbpfa;

/// Computes the Pfaffian of a banded skew-symmetric matrix taking special care
/// to avoid numerical under- or overflow.
///
/// # Purpose
///
/// `skbpf10` computes the Pfaffian of a banded skew-symmetric matrix, taking
/// special care to avoid numerical under- or overflow. (at the cost of
/// possible additional round-off errors).
///
/// # Further details
///
/// For odd-sized matrices, the Pfaffian is 0 by default, hence no computation
/// is needed in this case. For even-sized matrices, the Pfaffian is computed
/// by bringing the skew-symmetric matrix A into tridiagonal form T by a
/// unitary congruence transformation: Q^H * A * Q^* = T. This transformation
/// is computed by the routine [`ssktrd`] (for further details see there).
pub mod skbpf10;
