#![allow(non_camel_case_types)]
// FIXME ^ should be removed

use pyffi::*;
use super::types::*;
use super::pyarray::*;

pub type npy_bool = ::std::os::raw::c_uchar;
pub type npy_iter_get_dataptr_t =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut PyArrayIterObject,
                                                 arg1: *mut npy_intp)
                                                 -> *mut ::std::os::raw::c_char>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PyArrayIterObject {
    pub ob_base: PyObject,
    pub nd_m1: ::std::os::raw::c_int,
    pub index: npy_intp,
    pub size: npy_intp,
    pub coordinates: [npy_intp; 32usize],
    pub dims_m1: [npy_intp; 32usize],
    pub strides: [npy_intp; 32usize],
    pub backstrides: [npy_intp; 32usize],
    pub factors: [npy_intp; 32usize],
    pub ao: *mut PyArrayObject,
    pub dataptr: *mut ::std::os::raw::c_char,
    pub contiguous: npy_bool,
    pub bounds: [[npy_intp; 2usize]; 32usize],
    pub limits: [[npy_intp; 2usize]; 32usize],
    pub limits_sizes: [npy_intp; 32usize],
    pub translate: npy_iter_get_dataptr_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PyArrayMultiIterObject {
    pub ob_base: PyObject,
    pub numiter: ::std::os::raw::c_int,
    pub size: npy_intp,
    pub index: npy_intp,
    pub nd: ::std::os::raw::c_int,
    pub dimensions: [npy_intp; 32usize],
    pub iters: [*mut PyArrayIterObject; 32usize],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PyArrayNeighborhoodIterObject {
    pub ob_base: PyObject,
    pub nd_m1: ::std::os::raw::c_int,
    pub index: npy_intp,
    pub size: npy_intp,
    pub coordinates: [npy_intp; 32usize],
    pub dims_m1: [npy_intp; 32usize],
    pub strides: [npy_intp; 32usize],
    pub backstrides: [npy_intp; 32usize],
    pub factors: [npy_intp; 32usize],
    pub ao: *mut PyArrayObject,
    pub dataptr: *mut ::std::os::raw::c_char,
    pub contiguous: npy_bool,
    pub bounds: [[npy_intp; 2usize]; 32usize],
    pub limits: [[npy_intp; 2usize]; 32usize],
    pub limits_sizes: [npy_intp; 32usize],
    pub translate: npy_iter_get_dataptr_t,
    pub nd: npy_intp,
    pub dimensions: [npy_intp; 32usize],
    pub _internal_iter: *mut PyArrayIterObject,
    pub constant: *mut ::std::os::raw::c_char,
    pub mode: ::std::os::raw::c_int,
}
