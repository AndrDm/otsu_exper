#![allow(unused_imports)]
#![allow(dead_code)]

use crate::nivision::*;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

// Fixed RAII wrapper for IMAQ resources
pub struct ImaqImage(pub *mut Image); // Add `pub` here!

impl ImaqImage {
	pub fn new(image_type: ImageType_enum) -> Option<Self> {
		let ptr = unsafe { imaqCreateImage(image_type, 0) };
		if ptr.is_null() { None } else { Some(Self(ptr)) }
	}

	pub fn read_file(&self, filename: &str) -> Result<i32, &'static str> {
		let filename_c =
			CString::new(filename).map_err(|_| "Invalid filename")?;

		let status = unsafe {
			imaqReadFile2(
				self.as_ptr(),
				filename_c.as_ptr(),
				0,
				ptr::null_mut(),
				ptr::null_mut(),
			)
		};

		if status == 0 {
			return Err("imaqReadFile2 failed");
		}

		Ok(status)
	}

	pub fn display(
		&self,
		window: i32,
		resize: i32,
	) -> Result<(), &'static str> {
		unsafe {
			imaqDisplayImage(self.as_ptr(), window, resize);
		}
		Ok(())
	}

	pub fn compute_histogram(
		&self,
		bins: u32,
		min: f64,
		max: f64,
	) -> Result<(ImaqHistogram, &[i32]), &'static str> {
		let histogram_ptr = unsafe {
			imaqHistogram(
				self.as_ptr(),
				bins as i32,
				min as f32,
				max as f32,
				ptr::null_mut(),
			)
		};

		let histogram = unsafe {
			histogram_ptr.as_ref().ok_or("Histogram allocation failed")?
		};
		let hist_slice = unsafe {
			slice::from_raw_parts(histogram.histogram, bins as usize)
		};

		Ok((ImaqHistogram(histogram_ptr as *mut c_void), hist_slice))
	}

	// Public getter for safe external access
	pub fn as_ptr(&self) -> *mut Image {
		self.0
	}
}

impl Drop for ImaqImage {
	fn drop(&mut self) {
		println!("image dropped");
		unsafe { imaqDispose(self.0 as *mut c_void) };
	}
}

// RAII wrapper for histogram - auto-disposes in Drop
pub struct ImaqHistogram(*mut c_void);

impl Drop for ImaqHistogram {
	fn drop(&mut self) {
		println!("histogram dropped");
		if !self.0.is_null() {
			unsafe { imaqDispose(self.0) };
		}
	}
}
