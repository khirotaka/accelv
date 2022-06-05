use libc::{c_double, c_float};


#[repr(C)]
pub struct DSPSplitComplex {
    pub realp: *mut c_float,
    pub imagp: *mut c_float,
}

#[repr(C)]
pub struct DSPImmutableSplitComplex {
    pub realp: *const c_float,
    pub imagp: *const c_float,
}

#[repr(C)]
pub struct DSPDoubleImmutableSplitComplex {
    pub realp: *const c_double,
    pub imagp: *const c_double,
}

#[repr(C)]
pub struct DSPDoubleSplitComplex {
    pub realp: *mut c_double,
    pub imagp: *mut c_double,
}
