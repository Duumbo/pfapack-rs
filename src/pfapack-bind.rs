/// Reduces a complex skew-symmetric band matrix A to real
/// skew-symmetric tridiagonal form T by a unitary congruence transformation.
///
/// # Purpose
///
/// `zskbtrd` reduces a complex skew-symmetric band matrix A to real
/// skew-symmetric tridiagonal form T by a unitary congruence transformation:
/// `Q^dagger * A * Q^* = T`. Alternatively, the routine can also compute a
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
/// unitary congruence transformation: `Q^H * A * Q^* = T`. This transformation
/// is computed by the routine [`ssktrd`] (for further details see there).
pub mod skbpf10;
