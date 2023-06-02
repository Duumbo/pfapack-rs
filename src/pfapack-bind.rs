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

#[inline]
pub unsafe fn sskbtrd(
    arg1: u8,
    arg2: u8,
    arg3: u8,
    arg4: &[i32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &[i32],
    arg8: &mut [f32],
    arg9: &mut [f32],
    arg10: &[i32],
    arg11: &mut [f32],
    arg12: &mut [i32],
) {
    ffi::sskbtrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        &(arg3 as c_char),
        arg4.as_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
        arg10.as_ptr(),
        arg11.as_mut_ptr(),
        arg12.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dskbtrd(
    arg1: u8,
    arg2: u8,
    arg3: u8,
    arg4: &[i32],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &[i32],
    arg8: &mut [f64],
    arg9: &mut [f64],
    arg10: &[i32],
    arg11: &mut [f64],
    arg12: &mut [i32],
) {
    ffi::dskbtrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        &(arg3 as c_char),
        arg4.as_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
        arg10.as_ptr(),
        arg11.as_mut_ptr(),
        arg12.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cskbtrd(
    arg1: u8,
    arg2: u8,
    arg3: u8,
    arg4: &[i32],
    arg5: &[i32],
    arg6: &mut [c32],
    arg7: &[i32],
    arg8: &mut [f32],
    arg9: &mut [c32],
    arg10: &mut [c32],
    arg11: &[i32],
    arg12: &mut [c32],
    arg13: &mut [f32],
    arg14: &mut [i32],
) {
    ffi::cskbtrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        &(arg3 as c_char),
        arg4.as_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr() as *mut _,
        arg10.as_mut_ptr() as *mut _,
        arg11.as_ptr(),
        arg12.as_mut_ptr() as *mut _,
        arg13.as_mut_ptr(),
        arg14.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zskbtrd(
    arg1: u8,
    arg2: u8,
    arg3: u8,
    arg4: &[i32],
    arg5: &[i32],
    arg6: &mut [c64],
    arg7: &[i32],
    arg8: &mut [f64],
    arg9: &mut [c64],
    arg10: &mut [c64],
    arg11: &[i32],
    arg12: &mut [c64],
    arg13: &mut [f64],
    arg14: &mut [i32],
) {
    ffi::zskbtrd_(
        &(arg1 as c_char),
        &(arg2 as c_char),
        &(arg3 as c_char),
        arg4.as_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_ptr(),
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr() as *mut _,
        arg10.as_mut_ptr() as *mut _,
        arg11.as_ptr(),
        arg12.as_mut_ptr() as *mut _,
        arg13.as_mut_ptr(),
        arg14.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sskbpfa(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [f32],
    arg8: &mut [i32],
) {
    ffi::sskbpfa_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dskbpfa(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [f64],
    arg8: &mut [i32],
) {
    ffi::dskbpfa_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cskbpfa(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [c32],
    arg7: &mut [c32],
    arg8: &mut [f32],
    arg9: &mut [i32],
) {
    ffi::cskbpfa_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zskbpfa(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [c64],
    arg7: &mut [c64],
    arg8: &mut [f64],
    arg9: &mut [i32],
) {
    ffi::zskbpfa_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sskbpf10(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [f32],
    arg5: &[i32],
    arg6: &mut [f32],
    arg7: &mut [f32],
    arg8: &mut [i32],
) {
    ffi::sskbpf10_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dskbpf10(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [f64],
    arg5: &[i32],
    arg6: &mut [f64],
    arg7: &mut [f64],
    arg8: &mut [i32],
) {
    ffi::dskbpf10_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr(),
        arg5.as_ptr(),
        arg6.as_mut_ptr(),
        arg7.as_mut_ptr(),
        arg8.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cskbpf10(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [c32],
    arg5: &[i32],
    arg6: &mut [c32],
    arg7: &mut [c32],
    arg8: &mut [f32],
    arg9: &mut [i32],
) {
    ffi::cskbpf10_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zskbpf10(
    arg1: u8,
    arg2: &[i32],
    arg3: &[i32],
    arg4: &mut [c64],
    arg5: &[i32],
    arg6: &mut [c64],
    arg7: &mut [c64],
    arg8: &mut [f64],
    arg9: &mut [i32],
) {
    ffi::zskbpf10_(
        &(arg1 as c_char),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_mut_ptr() as *mut _,
        arg5.as_ptr(),
        arg6.as_mut_ptr() as *mut _,
        arg7.as_mut_ptr() as *mut _,
        arg8.as_mut_ptr(),
        arg9.as_mut_ptr(),
    )
}
