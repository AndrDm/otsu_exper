#![allow(unused_imports)]
mod nivision;
mod nivision_ex;
use crate::{nivision::*, nivision_ex::*};

pub fn otsu_threshold_16bit(hist: &[i32]) -> u16 {
	const NUM_BINS: usize = 65536;
	debug_assert!(hist.len() == NUM_BINS);

	let mut total: u64 = 0;
	let mut sum: f64 = 0.0;

	for (i, &count) in hist.iter().enumerate() {
		let count = count as u64;
		total += count;
		sum += (i as f64) * (count as f64);
	}

	if total == 0 {
		return 0;
	}

	let mut sum_b = 0.0;
	let mut w_b = 0u64;
	let mut max_between = -1.0;
	let mut best_thresh = 0u16;

	for t in 0..NUM_BINS {
		let count = hist[t] as u64;
		w_b += count;
		if w_b == 0 {
			continue;
		}
		let w_f = total - w_b;
		if w_f == 0 {
			break;
		}

		sum_b += (t as f64) * (count as f64);
		let m_b = sum_b / (w_b as f64);
		let m_f = (sum - sum_b) / (w_f as f64);
		let between = (w_b as f64) * (w_f as f64) * (m_b - m_f) * (m_b - m_f);

		if between > max_between {
			max_between = between;
			best_thresh = t as u16;
		}
	}

	best_thresh
}

fn main() {
	let image = ImaqImage::new(ImageType_enum_IMAQ_IMAGE_U16).unwrap();

	if let Err(e) = image.read_file("Zippo.tif") {
		println!("Error reading file - {}", e);
		return;
	}
	let (_histo, hist_slice) =
		image.compute_histogram(65536, 0.0, 65535.0).unwrap();
	let thr = otsu_threshold_16bit(hist_slice);
	let _ = image.display(0, 0);
	//unsafe{imaqDisplayImage(image.as_ptr(), 0, 0)};
	println!("Otsu threshold = {}", thr);
}

/*
// With maximal error handling:
fn main() {
	// Single block for entire FFI workflow

	let image = match ImaqImage::new(ImageType_enum_IMAQ_IMAGE_U16) {
		Some(img) => img,
		None => {
			println!("Failed to create image buffer");
			return;
		}
	};

	// Safe wrapper usage - no more unsafe!
	if let Err(e) = image.read_file("ZippoC.tif") {
		println!("Error reading file - {}", e);
		return;
	}
	// Safe histogram with automatic cleanup!
	let (_histogram, hist_slice) = match image.compute_histogram(65536, 0.0, 65535.0) {
		Ok(result) => result,
		Err(e) => { println!("{}", e); return; }
	};
	// histogram drops automatically here - no manual dispose!

	let thr = otsu_threshold_16bit(hist_slice);
	println!("Otsu threshold = {}", thr);
}
 */
