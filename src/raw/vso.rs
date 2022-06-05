use libc::{c_double, c_float, c_int, c_long, c_ulong};
use crate::raw::structs::{
    DSPDoubleSplitComplex, DSPSplitComplex,
    DSPDoubleImmutableSplitComplex, DSPImmutableSplitComplex
};


// Rust binding of the Vector-Scalar Arithmetic module of the vDSP library included
// in the Apple Accelerate Framework.
// It contains basic arithmetic functions for performing operations on scalar values and vectors.
// See the following URL for details.
// <https://developer.apple.com/documentation/accelerate/vdsp/basic_arithmetic_functions>

#[link(name="Accelerate", kind="framework")]
extern "C" {
    // Vector-Scaler Addition
    /// Adds a single-precision scalar value to a single-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450275-vdsp_vsadd>
    pub fn vDSP_vsadd(
        __a: *const c_float, __ia: c_long,
        __b: *const c_float,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong
    );

    /// Adds a double-precision scalar value to a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450860-vdsp_vsaddd>
    pub fn vDSP_vsaddD(
        __a: *const c_double, __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong
    );

    /// Adds an integer scalar value to an integer vector.
    /// <https://developer.apple.com/documentation/accelerate/1450088-vdsp_vsaddi>
    pub fn vDSP_vsaddi(
        __a: *const c_int, __ia: c_long,
        __b: *const c_int,
        __c: *mut c_int,
        __ic: c_long,
        __n: c_ulong
    );

    // Vector-Scaler Multiplication
    /// Multiplies a single-precision scalar value by a single-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450020-vdsp_vsmul>
    pub fn vDSP_vsmul(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong
    );

    /// Multiplies a double-precision scalar value by a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1449676-vdsp_vsmuld>
    pub fn vDSP_vsmulD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong
    );

    /// Multiplies a single-precision complex scalar value by a single-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1450410-vdsp_zvzsml>
    pub fn vDSP_zvzsml(
        __a: *const DSPImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPImmutableSplitComplex,
        __c: *mut DSPSplitComplex,
        __ic: c_long,
        __n: c_ulong
    );

    /// Multiplies a double-precision complex scalar value by a double-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1449727-vdsp_zvzsmld>
    pub fn vDSP_zvzsmlD(
        __a: *const DSPDoubleImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPDoubleImmutableSplitComplex,
        __c: *mut DSPDoubleSplitComplex,
        __ic: c_long,
        __n: c_ulong
    );

    // Vector-Scaler Division
    /// Divides a single-precision vector by a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450680-vdsp_vsdiv>
    pub fn vDSP_vsdiv(
        __a: *const c_float, __ia: c_long,
        __b: *const c_float,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong
    );

    /// Divides a double-precision vector by a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450212-vdsp_vsdivd>
    pub fn vDSP_vsdivD(
        __a: *const c_double, __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong
    );

    /// Divides an integer vector by an integer scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1449689-vdsp_vsdivi>
    pub fn vDSP_vsdivi(
        __a: *const c_int, __ia: c_long,
        __b: *const c_int,
        __c: *mut c_int,
        __ic: c_long,
        __n: c_ulong
    );
}

#[cfg(test)]
mod tests {
    use crate::raw;
    use libc::{c_ulong};
    use crate::raw::{DSPDoubleImmutableSplitComplex, DSPDoubleSplitComplex, DSPImmutableSplitComplex, DSPSplitComplex};

    #[test]
    fn test_v_dsp_vsadd() {
        let a: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let b: f32 = 5.;
        let mut c: Vec<f32> = vec![0.0; a.len()];
        unsafe {
            raw::vDSP_vsadd(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<f32>::from([6.0, 7.0, 8.0, 9.0, 10.0]));
    }

    #[test]
    fn test_vdsp_vsadd_d() {
        let a: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let b: f64 = 5.;
        let mut c: Vec<f64> = vec![0.0; a.len()];
        unsafe {
            raw::vDSP_vsaddD(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<f64>::from([6.0, 7.0, 8.0, 9.0, 10.0]));
    }

    #[test]
    fn test_vdsp_vsaddi() {
        let a: Vec<i32> = vec![1, 2, 3, 4, 5];
        let b: i32 = 5;
        let mut c: Vec<i32> = vec![0; a.len()];
        unsafe {
            raw::vDSP_vsaddi(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<i32>::from([6, 7, 8, 9, 10]));
    }

    #[test]
    fn test_vdsp_vsmul() {
        let a: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let b: f32 = 3.;
        let mut c: Vec<f32> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsmul(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }

        assert_eq!(c, Vec::<f32>::from([3., 6., 9., 12., 15.]));
    }

    #[test]
    fn test_vdsp_vsmul_d() {
        let a: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let b: f64 = 3.;
        let mut c: Vec<f64> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsmulD(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }

        assert_eq!(c, Vec::<f64>::from([3., 6., 9., 12., 15.]));
    }

    #[test]
    fn test_vdsp_zvzsml() {
        let a_r: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a_i: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a = DSPImmutableSplitComplex {
            realp: a_r.as_ptr(),
            imagp: a_i.as_ptr()
        };

        let b_r: f32 = 5.0;
        let b_i: f32 = 5.0;
        let b = DSPImmutableSplitComplex {
            realp: &b_r,
            imagp: &b_i
        };

        let mut c_r: Vec<f32> = vec![0.0; a_r.len()];
        let mut c_i: Vec<f32> = vec![0.0; a_r.len()];
        let mut c = DSPSplitComplex {
            realp: c_r.as_mut_ptr(),
            imagp: c_i.as_mut_ptr()
        };

        unsafe {
            raw::vDSP_zvzsml(
                &a,
                1,
                &b,
                &mut c,
                1,
                a_r.len() as c_ulong
            );
        }

        assert_eq!(
            c_r,
            Vec::<f32>::from([0., 0., 0., 0., 0.])
        );
        assert_eq!(
            c_i,
            Vec::<f32>::from([10., 20., 30., 40., 50.])
        );
    }

    fn test_vdsp_zvzsml_d() {
        let a_r: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a_i: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a = DSPDoubleImmutableSplitComplex {
            realp: a_r.as_ptr(),
            imagp: a_i.as_ptr()
        };

        let b_r: f64 = 5.0;
        let b_i: f64 = 5.0;
        let b = DSPDoubleImmutableSplitComplex {
            realp: &b_r,
            imagp: &b_i
        };

        let mut c_r: Vec<f64> = vec![0.0; a_r.len()];
        let mut c_i: Vec<f64> = vec![0.0; a_r.len()];
        let mut c = DSPDoubleSplitComplex {
            realp: c_r.as_mut_ptr(),
            imagp: c_i.as_mut_ptr()
        };

        unsafe {
            raw::vDSP_zvzsmlD(
                &a,
                1,
                &b,
                &mut c,
                1,
                a_r.len() as c_ulong
            );
        }

        assert_eq!(
            c_r,
            Vec::<f64>::from([0., 0., 0., 0., 0.])
        );
        assert_eq!(
            c_i,
            Vec::<f64>::from([10., 20., 30., 40., 50.])
        );
    }

    fn test_vdsp_vsdiv() {
        let a: Vec<f32> = vec![4., 6., 8., 10., 12.];
        let b: f32 = 2.;
        let mut c: Vec<f32> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsdiv(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<f32>::from([2., 3., 4., 5., 6.]));
    }

    fn test_vdsp_vsdiv_d() {
        let a: Vec<f64> = vec![4., 6., 8., 10., 12.];
        let b: f64 = 2.;
        let mut c: Vec<f64> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsdivD(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<f64>::from([2., 3., 4., 5., 6.]));
    }

    fn test_vdsp_vsdiv_i() {
        let a: Vec<i32> = vec![4, 6, 8, 10, 12];
        let b: i32 = 2;
        let mut c: Vec<i32> = vec![0; a.len()];

        unsafe {
            raw::vDSP_vsdivi(
                a.as_ptr(),
                1,
                &b,
                c.as_mut_ptr(),
                1,
                a.len() as c_ulong
            )
        }
        assert_eq!(c, Vec::<i32>::from([2, 3, 4, 5, 6]));
    }
}
