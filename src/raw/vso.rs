use crate::raw::structs::{
    DSPDoubleImmutableSplitComplex, DSPDoubleSplitComplex, DSPImmutableSplitComplex,
    DSPSplitComplex,
};
use libc::{c_double, c_float, c_int, c_long, c_ulong};

// Rust binding of the Vector-Scalar Arithmetic module of the vDSP library included
// in the Apple Accelerate Framework.
// It contains basic arithmetic functions for performing operations on scalar values and vectors.
// See the following URL for details.
// <https://developer.apple.com/documentation/accelerate/vdsp/basic_arithmetic_functions>

#[link(name = "Accelerate", kind = "framework")]
extern "C" {
    // Vector-Scaler Addition
    /// Adds a single-precision scalar value to a single-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450275-vdsp_vsadd>
    pub fn vDSP_vsadd(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Adds a double-precision scalar value to a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450860-vdsp_vsaddd>
    pub fn vDSP_vsaddD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Adds an integer scalar value to an integer vector.
    /// <https://developer.apple.com/documentation/accelerate/1450088-vdsp_vsaddi>
    pub fn vDSP_vsaddi(
        __a: *const c_int,
        __ia: c_long,
        __b: *const c_int,
        __c: *mut c_int,
        __ic: c_long,
        __n: c_ulong,
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
        __n: c_ulong,
    );

    /// Multiplies a double-precision scalar value by a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1449676-vdsp_vsmuld>
    pub fn vDSP_vsmulD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Multiplies a single-precision complex scalar value by a single-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1450410-vdsp_zvzsml>
    pub fn vDSP_zvzsml(
        __a: *const DSPImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPImmutableSplitComplex,
        __c: *mut DSPSplitComplex,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Multiplies a double-precision complex scalar value by a double-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1449727-vdsp_zvzsmld>
    pub fn vDSP_zvzsmlD(
        __a: *const DSPDoubleImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPDoubleImmutableSplitComplex,
        __c: *mut DSPDoubleSplitComplex,
        __ic: c_long,
        __n: c_ulong,
    );

    // Vector-Scaler Division
    /// Divides a single-precision vector by a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450680-vdsp_vsdiv>
    pub fn vDSP_vsdiv(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Divides a double-precision vector by a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450212-vdsp_vsdivd>
    pub fn vDSP_vsdivD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Divides an integer vector by an integer scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1449689-vdsp_vsdivi>
    pub fn vDSP_vsdivi(
        __a: *const c_int,
        __ia: c_long,
        __b: *const c_int,
        __c: *mut c_int,
        __ic: c_long,
        __n: c_ulong,
    );

    // Scaler-Vector Division
    /// Divides a single-precision scalar value by a single-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450412-vdsp_svdiv>
    pub fn vDSP_svdiv(
        __a: *const c_float,
        __b: *const c_float,
        __ib: c_long,
        __c: *mut c_float,
        __ic: c_long,
        __n: c_ulong,
    );

    /// Divides a double-precision scalar value by a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450028-vdsp_svdivd>
    pub fn vDSP_svdivD(
        __a: *const c_double,
        __b: *const c_double,
        __ib: c_long,
        __c: *mut c_double,
        __ic: c_long,
        __n: c_ulong,
    );

    // Vector-Vector-Scalar Add-Multiply
    /// Multiplies the sum of two single-precision vectors by a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1449773-vdsp_vasm>
    pub fn vDSP_vasm(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __ib: c_long,
        __c: *const c_float,
        __d: *mut c_float,
        __id: c_long,
        __n: c_ulong,
    );

    /// Multiplies the sum of two double-precision vectors by a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450146-vdsp_vasmd>
    pub fn vDSP_vasmD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __ib: c_long,
        __c: *const c_double,
        __d: *mut c_double,
        __id: c_long,
        __n: c_ulong,
    );

    // Vector-Vector-Scalar Subtract-Multiply
    /// Multiplies the difference of two single-precision vectors by a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450734-vdsp_vsbsm>
    pub fn vDSP_vsbsm(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __ib: c_long,
        __c: *const c_float,
        __d: *mut c_float,
        __id: c_long,
        __n: c_ulong,
    );

    /// Multiplies the difference of two double-precision vectors by a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450372-vdsp_vsbsmd>
    pub fn vDSP_vsbsmD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __ib: c_long,
        __c: *const c_double,
        __d: *mut c_double,
        __id: c_long,
        __n: c_ulong,
    );

    // Vector-Vector-Scalar Multiply-Subtract
    /// Subtracts a single-precision vector from the product of a single-precision scalar value and
    /// a single-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450822-vdsp_vsmsb/>
    pub fn vDSP_vsmsb(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *const c_float,
        __ic: c_long,
        __d: *mut c_float,
        __id: c_long,
        __n: c_ulong,
    );

    /// Subtracts a double-precision vector from the product of a double-precision scalar value and
    /// a double-precision vector.
    /// <https://developer.apple.com/documentation/accelerate/1450238-vdsp_vsmsbd>
    pub fn vDSP_vsmsbD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *const c_double,
        __ic: c_long,
        __d: *mut c_double,
        __id: c_long,
        __n: c_ulong,
    );

    // Vector-Vector-Scalar Multiply-Add
    /// Adds a single-precision scalar value to the product of two single-precision vectors.
    /// <https://developer.apple.com/documentation/accelerate/1450590-vdsp_vmsa>
    pub fn vDSP_vmsa(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __ib: c_long,
        __c: *const c_float,
        __d: *mut c_float,
        __id: c_long,
        __n: c_ulong,
    );

    /// Adds a double-precision scalar value to the product of two double-precision vectors.
    /// <https://developer.apple.com/documentation/accelerate/1450698-vdsp_vmsad>
    pub fn vDSP_vmsaD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __ib: c_long,
        __c: *const c_double,
        __d: *mut c_double,
        __id: c_long,
        __n: c_ulong,
    );

    /// Adds a single-precision complex vector to the product of a single-precision
    /// complex scalar value and a single-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1449902-vdsp_zvsma>
    pub fn vDSP_zvsma(
        __a: *const DSPImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPImmutableSplitComplex,
        __c: *const DSPImmutableSplitComplex,
        __ic: c_long,
        __d: *mut DSPSplitComplex,
        __id: c_long,
        __n: c_ulong,
    );

    /// Adds a double-precision complex vector to the product of a double-precision
    /// complex scalar value and a double-precision complex vector.
    /// <https://developer.apple.com/documentation/accelerate/1450570-vdsp_zvsmad>
    pub fn vDSP_zvsmaD(
        __a: *const DSPDoubleImmutableSplitComplex,
        __ia: c_long,
        __b: *const DSPDoubleImmutableSplitComplex,
        __c: *const DSPDoubleImmutableSplitComplex,
        __ic: c_long,
        __d: *mut DSPDoubleSplitComplex,
        __id: c_long,
        __n: c_ulong,
    );

    // Vector-Scalar-Scalar Multiply-Add
    /// Adds a single-precision scalar value to the product of a single-precision vector
    /// and a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450380-vdsp_vsmsa>
    pub fn vDSP_vsmsa(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *const c_float,
        __d: *mut c_float,
        __id: c_long,
        __n: c_ulong,
    );

    /// Adds a double-precision scalar value to the product of a double-precision vector
    /// and a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450432-vdsp_vsmsad>
    pub fn vDSP_vsmsaD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *const c_double,
        __d: *mut c_double,
        __id: c_long,
        __n: c_ulong,
    );

    /// Adds the product of a single-precision vector and a single-precision scalar value
    /// to a second product of a single-precision vector and a single-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1450324-vdsp_vsmsma>
    pub fn vDSP_vsmsma(
        __a: *const c_float,
        __ia: c_long,
        __b: *const c_float,
        __c: *const c_float,
        __ic: c_long,
        __d: *const c_float,
        __e: *mut c_float,
        __ie: c_long,
        __n: c_ulong,
    );

    /// Adds the product of a double-precision vector and a double-precision scalar value
    /// to a second product of a double-precision vector and a double-precision scalar value.
    /// <https://developer.apple.com/documentation/accelerate/1449850-vdsp_vsmsmad>
    pub fn vDSP_vsmsmaD(
        __a: *const c_double,
        __ia: c_long,
        __b: *const c_double,
        __c: *const c_double,
        __ic: c_long,
        __d: *const c_double,
        __e: *mut c_double,
        __ie: c_long,
        __n: c_ulong,
    );
}

#[cfg(test)]
mod tests {
    use crate::raw;
    use is_close::all_close;
    use libc::c_ulong;

    #[test]
    fn test_v_dsp_vsadd() {
        let a: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let b: f32 = 5.;
        let mut c: Vec<f32> = vec![0.0; a.len()];
        unsafe { raw::vDSP_vsadd(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert!(all_close!(c, Vec::<f32>::from([6.0, 7.0, 8.0, 9.0, 10.0])));
    }

    #[test]
    fn test_vdsp_vsadd_d() {
        let a: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let b: f64 = 5.;
        let mut c: Vec<f64> = vec![0.0; a.len()];
        unsafe { raw::vDSP_vsaddD(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert!(all_close!(c, Vec::<f64>::from([6.0, 7.0, 8.0, 9.0, 10.0])));
    }

    #[test]
    fn test_vdsp_vsaddi() {
        let a: Vec<i32> = vec![1, 2, 3, 4, 5];
        let b: i32 = 5;
        let mut c: Vec<i32> = vec![0; a.len()];
        unsafe { raw::vDSP_vsaddi(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert_eq!(c, Vec::<i32>::from([6, 7, 8, 9, 10]));
    }

    #[test]
    fn test_vdsp_vsmul() {
        let a: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let b: f32 = 3.;
        let mut c: Vec<f32> = vec![0.0; a.len()];

        unsafe { raw::vDSP_vsmul(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }

        assert!(all_close!(c, Vec::<f32>::from([3., 6., 9., 12., 15.])));
    }

    #[test]
    fn test_vdsp_vsmul_d() {
        let a: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let b: f64 = 3.;
        let mut c: Vec<f64> = vec![0.0; a.len()];

        unsafe { raw::vDSP_vsmulD(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }

        assert!(all_close!(c, Vec::<f64>::from([3., 6., 9., 12., 15.])));
    }

    #[test]
    fn test_vdsp_zvzsml() {
        let a_r: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a_i: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a = raw::DSPImmutableSplitComplex {
            realp: a_r.as_ptr(),
            imagp: a_i.as_ptr(),
        };

        let b_r: f32 = 5.0;
        let b_i: f32 = 5.0;
        let b = raw::DSPImmutableSplitComplex {
            realp: &b_r,
            imagp: &b_i,
        };

        let mut c_r: Vec<f32> = vec![0.0; a_r.len()];
        let mut c_i: Vec<f32> = vec![0.0; a_r.len()];
        let mut c = raw::DSPSplitComplex {
            realp: c_r.as_mut_ptr(),
            imagp: c_i.as_mut_ptr(),
        };

        unsafe {
            raw::vDSP_zvzsml(&a, 1, &b, &mut c, 1, a_r.len() as c_ulong);
        }

        assert!(all_close!(c_r, Vec::<f32>::from([0., 0., 0., 0., 0.])));
        assert!(all_close!(c_i, Vec::<f32>::from([10., 20., 30., 40., 50.])));
    }

    #[test]
    fn test_vdsp_zvzsml_d() {
        let a_r: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a_i: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let a = raw::DSPDoubleImmutableSplitComplex {
            realp: a_r.as_ptr(),
            imagp: a_i.as_ptr(),
        };

        let b_r: f64 = 5.0;
        let b_i: f64 = 5.0;
        let b = raw::DSPDoubleImmutableSplitComplex {
            realp: &b_r,
            imagp: &b_i,
        };

        let mut c_r: Vec<f64> = vec![0.0; a_r.len()];
        let mut c_i: Vec<f64> = vec![0.0; a_r.len()];
        let mut c = raw::DSPDoubleSplitComplex {
            realp: c_r.as_mut_ptr(),
            imagp: c_i.as_mut_ptr(),
        };

        unsafe {
            raw::vDSP_zvzsmlD(&a, 1, &b, &mut c, 1, a_r.len() as c_ulong);
        }

        assert!(all_close!(c_r, Vec::<f64>::from([0., 0., 0., 0., 0.])));
        assert!(all_close!(c_i, Vec::<f64>::from([10., 20., 30., 40., 50.])));
    }

    #[test]
    fn test_vdsp_vsdiv() {
        let a: Vec<f32> = vec![4., 6., 8., 10., 12.];
        let b: f32 = 2.;
        let mut c: Vec<f32> = vec![0.0; a.len()];

        unsafe { raw::vDSP_vsdiv(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert!(all_close!(c, Vec::<f32>::from([2., 3., 4., 5., 6.])));
    }

    #[test]
    fn test_vdsp_vsdiv_d() {
        let a: Vec<f64> = vec![4., 6., 8., 10., 12.];
        let b: f64 = 2.;
        let mut c: Vec<f64> = vec![0.0; a.len()];

        unsafe { raw::vDSP_vsdivD(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert!(all_close!(c, Vec::<f64>::from([2., 3., 4., 5., 6.])));
    }

    #[test]
    fn test_vdsp_vsdiv_i() {
        let a: Vec<i32> = vec![4, 6, 8, 10, 12];
        let b: i32 = 2;
        let mut c: Vec<i32> = vec![0; a.len()];

        unsafe { raw::vDSP_vsdivi(a.as_ptr(), 1, &b, c.as_mut_ptr(), 1, a.len() as c_ulong) }
        assert_eq!(c, Vec::<i32>::from([2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_vdsp_svdiv() {
        let a: f32 = 2.0;
        let b: Vec<f32> = vec![1., 2., 4., 5.];
        let mut c: Vec<f32> = vec![0.0; b.len()];

        unsafe { raw::vDSP_svdiv(&a, b.as_ptr(), 1, c.as_mut_ptr(), 1, b.len() as c_ulong) }

        assert!(all_close!(c, Vec::<f32>::from([2., 1., 0.5, 0.4])));
    }

    #[test]
    fn test_vdsp_svdiv_d() {
        let a: f64 = 2.0;
        let b: Vec<f64> = vec![1., 2., 4., 5.];
        let mut c: Vec<f64> = vec![0.0; b.len()];

        unsafe { raw::vDSP_svdivD(&a, b.as_ptr(), 1, c.as_mut_ptr(), 1, b.len() as c_ulong) }

        assert!(all_close!(c, Vec::<f64>::from([2., 1., 0.5, 0.4])));
    }

    #[test]
    fn test_vdsp_vasm() {
        let a: Vec<f32> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f32> = vec![1., 2., 3., 4., 5.];

        let c: f32 = 2.;

        let mut d: Vec<f32> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vasm(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            );
        }
        assert!(all_close!(
            d,
            Vec::<f32>::from([22.0, 44.0, 66.0, 88.0, 110.0])
        ));
    }

    #[test]
    fn test_vdsp_vasm_d() {
        let a: Vec<f64> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let c: f64 = 2.;

        let mut d: Vec<f64> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vasmD(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            );
        }
        assert!(all_close!(
            d,
            Vec::<f64>::from([22.0, 44.0, 66.0, 88.0, 110.0])
        ));
    }

    #[test]
    fn test_vdsp_vsbsm() {
        let a: Vec<f32> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let c: f32 = 2.;
        let mut d: Vec<f32> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsbsm(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f32>::from([18.0, 36.0, 54.0, 72.0, 90.0])
        ));
    }

    #[test]
    fn test_vdsp_vsbsm_d() {
        let a: Vec<f64> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let c: f64 = 2.;
        let mut d: Vec<f64> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsbsmD(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f64>::from([18.0, 36.0, 54.0, 72.0, 90.0])
        ));
    }

    #[test]
    fn test_vdsp_vsmsb() {
        let a: Vec<f32> = vec![10., 20., 30., 40., 50.];
        let b: f32 = 2.0;
        let c: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let mut d: Vec<f32> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsmsb(
                a.as_ptr(),
                1,
                &b,
                c.as_ptr(),
                1,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f32>::from([19.0, 38.0, 57.0, 76.0, 95.0])
        ));
    }

    #[test]
    fn test_vdsp_vsmsb_d() {
        let a: Vec<f64> = vec![10., 20., 30., 40., 50.];
        let b: f64 = 2.0;
        let c: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let mut d: Vec<f64> = vec![0.0; a.len()];

        unsafe {
            raw::vDSP_vsmsbD(
                a.as_ptr(),
                1,
                &b,
                c.as_ptr(),
                1,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f64>::from([19.0, 38.0, 57.0, 76.0, 95.0])
        ));
    }

    #[test]
    fn test_vdsp_vmsa() {
        let a: Vec<f32> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let c: f32 = 2.;
        let mut d: Vec<f32> = vec![0.; a.len()];

        unsafe {
            raw::vDSP_vmsa(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f32>::from([12.0, 42.0, 92.0, 162.0, 252.0])
        ));
    }

    #[test]
    fn test_vdsp_vmsa_d() {
        let a: Vec<f64> = vec![10., 20., 30., 40., 50.];
        let b: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let c: f64 = 2.;
        let mut d: Vec<f64> = vec![0.; a.len()];

        unsafe {
            raw::vDSP_vmsaD(
                a.as_ptr(),
                1,
                b.as_ptr(),
                1,
                &c,
                d.as_mut_ptr(),
                1,
                a.len() as c_ulong,
            )
        }

        assert!(all_close!(
            d,
            Vec::<f64>::from([12.0, 42.0, 92.0, 162.0, 252.0])
        ));
    }
}
