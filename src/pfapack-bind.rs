/// Computes the factorization of a skew-symmetric matrix A
/// using the Parlett-Reid algorithm.
///
/// # Purpose
///
/// `ssktrf` computes the factorization of a skew-symmetric matrix A
/// using the Parlett-Reid algorithm:
/// P*A*P^T = U*T*U^T or
/// P*A*P^T = L*T*L^T
/// where U (or L) unit upper (lower) triangular matrix (^T denotes
/// the transpose), T is a skew-symmetric tridiagonal matrix and P
/// is a permutation matrix. In addition to being unit triangular,
/// U(1:n-1,n)=0 and L(2:n,1)=0.
/// Instead of a full tridiagonalization, SKTRF can also compute a
/// partial tridiagonal form for computing the Pfaffian.

/// This subroutine uses, if enough memory is available, the
/// blocked version of the algorithm.


/// Pointer to a memory block of size N*N*sizeof(datatype)
/// On entry, the skew-symmetric NxN-matrix A in Fortran format.
/// On exit, the tridiagonal matrix T and the multipliers used
/// to obtain the factor U or L (see below for further details).
///
/// # Modes
/// ## Normal mode (`mode = 'N'`):

/// If UPLO = 'U', the U*T*U^T decomposition of A is computed. U is a
/// upper triangular unit matrix with the additional constraint
/// U(1:n-1,n) = 0, and T a tridiagonal matrix. The upper diagonal
/// of T is stored on exit in A(i,i+1) for i = 1 .. n-1. The column
/// U(1:i-1, i) is stored in A(1:i-1,i+1).

/// If UPLO = 'L', the L*T*L^T decomposition of A is computed. L is a
/// lower triangular unit matrix with the additional constraint
/// L(2:n,1) = 0, and T a tridiagonal matrix. The lower diagonal
/// of T is stored on exit in A(i+1,i) for i = 1 .. n-1. The column
/// L(i+1:n, i) is stored in A(i+1:n,i-1).

/// The contents of A on exit are illustrated by the following examples
/// with n = 5:

/// if UPLO = 'U':                       if UPLO = 'L':

///   (  0   e   u2  u3  u4 )              (  0                  )
///   (      0   e   u3  u4 )              (  e   0              )
///   (          0   e   u4 )              (  l2  e   0          )
///   (              0   e  )              (  l2  l3  e   0      )
///   (                  0  )              (  l2  l3  l4  e   0  )

/// where e denotes the off-diagonal elements of T, and ui (li)
/// denotes an element of the i-th column of U (L).

/// ## Pfaffian mode (`mode` = 'P'):

/// For computing the Pfaffian, it is enough to bring A into a partial
/// tridiagonal form. In particular, assuming n even, it is enough to
/// bring A into a form with A(i,j) = A(j,i) = 0 for i > j+1 with j odd
/// (this is computed if UPLO = 'L'), or A(i,j) = A(j,i) = 0 for
/// i > j-1 with j even (this is computed if UPLO = 'U'). Note that
/// only the off-diagonal entries in the odd columns (if UPLO = 'L')
/// or in the even columns (if UPLU = 'U') are properly computed by SKTRF.

/// If UPLO = 'U', the U*pT*U^T decomposition of A is computed. U is a
/// upper triangular unit matrix with the additional constraint
/// U(1:i-1,i) = 0 for even i, and pT a partially tridiagonal matrix.
/// The entries in the odd rows of the upper diagonal of pT are stored
/// on exit in A(i,i+1) for i odd. The column U(1:i-1, i) for odd i
/// is stored in A(1:i-1,i+1).

/// If UPLO = 'L', the L*pT*L^T decomposition of A is computed. L is a
/// lower triangular unit matrix with the additional constraint
/// L(i+1:n,i) = 0 for odd i, and pT a partially tridiagonal matrix.
/// The entries in odd columns in the lower diagonal of pT are stored
/// on exit in A(i+1,i) for i odd. The column L(i+1:n, i) for i odd
/// is stored in A(i+1:n,i-1).

/// The contents of A on exit are illustrated by the following examples
/// with n = 6:

/// if UPLO = 'U':                       if UPLO = 'L':

///   (  0   e   x   u3  x   u5 )              (  0                    )
///   (      0   x   u3  x   u5 )              (  e   0                )
///   (          0   e   x   u5 )              (  l2  x   0            )
///   (              0   x   u5 )              (  l2  x   e   0        )
///   (                  0   e  )              (  l2  x   l4  x   0    )
///   (                      0  )              (  l2  x   l4  x   e  0 )

/// where e denotes the off-diagonal elements of T, ui (li)
/// denotes an element of the i-th column of U (L), and x denotes an
/// element not computed by SKTRF.
///
///
/// # Further Details
/// The normal use for SKTRF is to compute the U T U^T or L T L^T
/// decomposition of a skew-symmetric matrix with pivoting. This mode
/// is chosen by setting MODE = 'N' ("normal" mode). The other
/// use of SKTRF is the computation the Pfaffian of a skew-symmetric matrix,
/// which only requires a partial computation of T, this mode is chosen
/// by setting MODE = 'P' ("Pfaffian" mode).
pub mod sktrf;

/// Reduces a real skew-symmetric matrix A to skew-symmetric
/// tridiagonal form T by an orthogonal similarity transformation.
/// # Purpose
///
/// `sktrd` reduces a real skew-symmetric matrix A to skew-symmetric
/// tridiagonal form T by an orthogonal similarity transformation:
/// Q^T * A * Q = T.  Alternatively, the routine can also compute
/// a partial tridiagonal form useful for computing the Pfaffian.
///
/// # Further Details
///
/// The normal use for `sktrd` is to compute the tridiagonal form of
/// a skew-symmetric matrix under an orthogonal similarity transformation,
/// and chosen by setting `mode = 'N'` ("normal" mode). The other
/// use of `sktrd` is the computation the Pfaffian of a skew-symmetric matrix,
/// which only requires a partial tridiagonalization, this mode is chosen
/// by setting `mode = 'P'` ("Pfaffian" mode).
///
/// # Modes
/// ## Normal mode (`mode = 'N'`):
///
/// The routine computes a tridiagonal matrix T and an orthogonal Q such
/// that A = Q * T * Q^T .
///
/// If `uplo = 'U'`, the matrix Q is represented as a product of elementary
/// reflectors
///
///    Q = H(n-1) . . . H(2) H(1).
///
/// Each H(i) has the form
///
///    H(i) = I - tau * v * v'
///
/// where tau is a real scalar, and v is a real vector with
/// v(i+1:n) = 0 and v(i) = 1; v(1:i-1) is stored on exit in
/// A(1:i-1,i+1), and tau in TAU(i).
///
/// If `uplo = 'L'`, the matrix Q is represented as a product of elementary
/// reflectors
///
///    Q = H(1) H(2) . . . H(n-1).
///
/// Each H(i) has the form
///
///    H(i) = I - tau * v * v'
///
/// where tau is a real scalar, and v is a real vector with
/// v(1:i) = 0 and v(i+1) = 1; v(i+2:n) is stored on exit in A(i+2:n,i),
/// and tau in TAU(i).
///
/// The contents of A on exit are illustrated by the following examples
/// with n = 5:
///
/// if UPLO = 'U':                       if UPLO = 'L':
///
///   (  0   e   v2  v3  v4 )              (  0                  )
///   (      0   e   v3  v4 )              (  e   0              )
///   (          0   e   v4 )              (  v1  e   0          )
///   (              0   e  )              (  v1  v2  e   0      )
///   (                  0  )              (  v1  v2  v3  e   0  )
///
/// where d and e denote diagonal and off-diagonal elements of T, and vi
/// denotes an element of the vector defining H(i).
///
/// The LAPACK routine DORGTR can be used to form the transformation
/// matrix explicitely, and DORMTR can be used to multiply another
/// matrix without forming the transformation.
///
/// ## Pfaffian mode (`mode = 'P'`):
///
/// For computing the Pfaffian, it is enough to bring A into a partial
/// tridiagonal form. In particular, assuming n even, it is enough to
/// bring A into a form with A(i,j) = A(j,i) = 0 for i > j+1 with j odd
/// (this is computed if UPLO = 'L'), or A(i,j) = A(j,i) = 0 for
/// i > j-1 with j even (this is computed if `uplo = 'U'`). Note that
/// only the off-diagonal entries in the odd columns (if UPLO = 'L')
/// or in the even columns (if `uplo = 'U'`) are properly computed by `sktrd`.
///
/// A is brought into this special form pT using an orthogonal matrix Q:
/// A = Q * pT * Q^T
///
/// If `uplo = 'U'`, the matrix Q is represented as a product of elementary
/// reflectors
///
///    Q = H(n-1) H(n-3) . . . H(3) H(1).
///
/// Each H(i) has the form
///
///    H(i) = I - tau * v * v^T
///
/// where tau is a real scalar, and v is a real vector with
/// v(i+1:n) = 0 and v(i) = 1; v(1:i-1) is stored on exit in
/// A(1:i-1,i+1), and tau in TAU(i).
///
/// If `uplo = 'L'`, the matrix Q is represented as a product of elementary
/// reflectors
///
///    Q = H(1) H(3) . . . H(n-3) H(n-1).
///
/// Each H(i) has the form
///
///    H(i) = I - tau * v * v^T
///
/// where tau is a real scalar, and v is a real vector with
/// v(1:i) = 0 and v(i+1) = 1; v(i+2:n) is stored on exit in A(i+2:n,i),
/// and tau in TAU(i).
///
/// The contents of A on exit are illustrated by the following examples
/// with n = 6:
///
/// if UPLO = 'U':                       if UPLO = 'L':
///
///   (  0   e   x   v3  x   v5 )        (  0                      )
///   (      0   x   v3  x   v5 )        (  e   0                  )
///   (          0   e   x   v5 )        (  v1  x   0              )
///   (              0   x   v5 )        (  v1  x   e   0          )
///   (                  0   e  )        (  v1  x   v3  x   0      )
///   (                      0  )        (  v1  x   v3  x   e   0  )
///
/// where d and e denote diagonal and off-diagonal elements of T, vi
/// denotes an element of the vector defining H(i), and x denotes an
/// element not computed by `sktrd`.
pub mod sktrd;

/// Computes the Pfaffian of a skew-symmetric matrix.
///
/// # Purpose
///
/// Computes the Pfaffian of a skew-symmetric matrix.
///
/// # Further Details
///
/// The Pfaffian is computed by bringing the skew-symmetric matrix A into
/// a partial tridiagonal form pT, either by computing a partial `L pT L^T`
/// decomposition `mthd = 'P'`, or by a by a unitary congruence transformation
/// `Q^H * A * Q^* = pT` `mthd = 'H'`.
/// These transformations are computed by the routines [`sktrf`] or [`sktrd`],
/// respectively (for further details see there).
pub mod skpfa;

/// Computes the Pfaffian of a skew-symmetric matrix, with care to overflow.
/// # Purpose
///
/// `skpf10` computes the Pfaffian of a skew-symmetric matrix, taking
/// special care to avoid numerical under- or overflow.
/// (at the cost of possible additional round-off errors)
///
/// # Further Details
///
/// The Pfaffian is computed by bringing the skew-symmetric matrix A into
/// a partial tridiagonal form pT, either by computing a partial `L pT L^T`
/// decomposition `mthd = 'P'`, or by a by a unitary congruence transformation
/// `Q^H * A * Q^* = pT` `mthd = 'H'`.
/// These transformations are computed by the routines [`sktrf`] or [`sktrd`]
/// respectively (for further details see there).
pub mod skpf10;

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
