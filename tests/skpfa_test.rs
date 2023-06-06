use pfapack::{skpfa::*, c64, c32};
use pretty_assertions::assert_eq;
use assert::close;

const N: i32 = 4;

#[test]
fn test_sskpfa() {
    let mut pfaff: f32 = 0.0;
    let mut pfaff2: f32 = 0.0;
    let mut info: i32 = 0;
    let mut a: Vec<f32> = vec![
        0.0, 1.0, 2.0, 3.0,
        -1.0, 0.0, 0.0, 5.0,
        -2.0, 0.0, 0.0, 6.0,
        -3.0, -5.0, -6.0, 0.0
    ];
    let mut iwork: Vec<i32> = Vec::with_capacity(N as usize);

    // Workspace query
    let mut work: Vec<f32> = Vec::with_capacity(1);
    work.push(0.0);
    unsafe {
        sskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &(-1),
            &mut info
        )
    }
    let lwork: i32 = work[0] as i32;
    // Compute using the lower and p method.
    let mut work: Vec<f32> = Vec::with_capacity(lwork as usize);
    unsafe {
        sskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &lwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(-4.0, pfaff, 1e-5);
    // Workspace query
    let mut work: Vec<f32> = Vec::with_capacity(1);
    work.push(0.0);
    unsafe {
        sskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &(-1),
            &mut info
        )
    }
    // Compute using upper triangle and method H.
    let lwork: i32 = work[0] as i32;
    let mut work: Vec<f32> = Vec::with_capacity(lwork as usize);
    unsafe {
        sskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &lwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(pfaff, pfaff2, 1e-5);
}

#[test]
fn test_dskpfa() {
    let mut pfaff: f64 = 0.0;
    let mut pfaff2: f64 = 0.0;
    let mut info: i32 = 0;
    let mut a: Vec<f64> = vec![
        0.0, 1.0, 2.0, 3.0,
        -1.0, 0.0, 0.0, 5.0,
        -2.0, 0.0, 0.0, 6.0,
        -3.0, -5.0, -6.0, 0.0
    ];
    let mut iwork: Vec<i32> = Vec::with_capacity(N as usize);

    // Workspace query
    let mut work: Vec<f64> = Vec::with_capacity(1);
    work.push(0.0);
    unsafe {
        dskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &(-1),
            &mut info
        )
    }
    let lwork: i32 = work[0] as i32;
    // Compute using the lower and p method.
    let mut work: Vec<f64> = Vec::with_capacity(lwork as usize);
    unsafe {
        dskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &lwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(-4.0, pfaff, 1e-5);
    // Workspace query
    let mut work: Vec<f64> = Vec::with_capacity(1);
    work.push(0.0);
    unsafe {
        dskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &(-1),
            &mut info
        )
    }
    // Compute using upper triangle and method H.
    let lwork: i32 = work[0] as i32;
    let mut work: Vec<f64> = Vec::with_capacity(lwork as usize);
    unsafe {
        dskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &lwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(pfaff, pfaff2, 1e-5);
}

#[test]
fn test_cskpfa() {
    let mut pfaff: c32 = c32::from(0.0);
    let mut pfaff2: c32 = c32::from(0.0);
    let mut info: i32 = 0;
    let mut a: Vec<c32> = vec![
        c32::from(0.0), c32::from(1.0), c32::from(2.0), c32::from(3.0),
        c32::from(-1.0), c32::from(0.0), c32::new(0.0, 4.0), c32::from(5.0),
        c32::from(-2.0), c32::new(0.0, -4.0), c32::from(0.0), c32::from(6.0),
        c32::from(-3.0), c32::from(-5.0), c32::from(-6.0), c32::from(0.0)
    ];
    let mut iwork: Vec<i32> = Vec::with_capacity(N as usize);

    // Workspace query
    let mut work: Vec<c32> = Vec::with_capacity(1);
    let mut rwork: Vec<f32> = Vec::with_capacity((N - 1) as usize);
    work.push(c32::from(0.0));
    unsafe {
        cskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &(-1),
            &mut rwork,
            &mut info
        )
    }
    let lwork: i32 = work[0].re as i32;
    // Compute using the lower and p method.
    let mut work: Vec<c32> = Vec::with_capacity(lwork as usize);
    unsafe {
        cskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &lwork,
            &mut rwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(-4.0, pfaff.re, 1e-5);
    close(12.0, pfaff.im, 1e-5);
    // Workspace query
    let mut work: Vec<c32> = Vec::with_capacity(1);
    work.push(c32::from(0.0));
    unsafe {
        cskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &(-1),
            &mut rwork,
            &mut info
        )
    }
    // Compute using upper triangle and method H.
    let lwork: i32 = work[0].re as i32;
    let mut work: Vec<c32> = Vec::with_capacity(lwork as usize);
    unsafe {
        cskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &lwork,
            &mut rwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(pfaff.re, pfaff2.re, 1e-5);
    close(pfaff.im, pfaff2.im, 1e-5);
}

#[test]
fn test_zskpfa() {
    let mut pfaff: c64 = c64::from(0.0);
    let mut pfaff2: c64 = c64::from(0.0);
    let mut info: i32 = 0;
    let mut a: Vec<c64> = vec![
        c64::from(0.0), c64::from(1.0), c64::from(2.0), c64::from(3.0),
        c64::from(-1.0), c64::from(0.0), c64::new(0.0, 4.0), c64::from(5.0),
        c64::from(-2.0), c64::new(0.0, -4.0), c64::from(0.0), c64::from(6.0),
        c64::from(-3.0), c64::from(-5.0), c64::from(-6.0), c64::from(0.0)
    ];
    let mut iwork: Vec<i32> = Vec::with_capacity(N as usize);

    // Workspace query
    let mut work: Vec<c64> = Vec::with_capacity(1);
    let mut rwork: Vec<f64> = Vec::with_capacity((N - 1) as usize);
    work.push(c64::from(0.0));
    unsafe {
        zskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &(-1),
            &mut rwork,
            &mut info
        )
    }
    let lwork: i32 = work[0].re as i32;
    // Compute using the lower and p method.
    let mut work: Vec<c64> = Vec::with_capacity(lwork as usize);
    unsafe {
        zskpfa(
            'L' as u8,
            'P' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff,
            &mut iwork,
            &mut work,
            &lwork,
            &mut rwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(-4.0, pfaff.re, 1e-5);
    close(12.0, pfaff.im, 1e-5);
    // Workspace query
    let mut work: Vec<c64> = Vec::with_capacity(1);
    work.push(c64::from(0.0));
    unsafe {
        zskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &(-1),
            &mut rwork,
            &mut info
        )
    }
    // Compute using upper triangle and method H.
    let lwork: i32 = work[0].re as i32;
    let mut work: Vec<c64> = Vec::with_capacity(lwork as usize);
    unsafe {
        zskpfa(
            'U' as u8,
            'H' as u8,
            &N,
            &mut a,
            &N,
            &mut pfaff2,
            &mut iwork,
            &mut work,
            &lwork,
            &mut rwork,
            &mut info
        )
    }
    assert_eq!(info, 0);
    close(pfaff.re, pfaff2.re, 1e-5);
    close(pfaff.im, pfaff2.im, 1e-5);
}
