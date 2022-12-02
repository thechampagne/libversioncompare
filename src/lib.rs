use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use version_compare::compare;
use version_compare::compare_to;
use version_compare::Cmp;
use version_compare::Manifest;

#[repr(C)]
#[allow(non_camel_case_types)]
enum version_compare_cmp {
    CMP_EQ,
    CMP_NE,
    CMP_LT,
    CMP_LE,
    CMP_GE,
    CMP_GT
}

#[repr(C)]
struct version_compare_manifest {
    max_depth: usize,
    ignore_text: c_int
}

#[no_mangle]
unsafe extern "C" fn version_compare_compare(cmp: *mut version_compare_cmp, a: *const c_char, b: *const c_char) -> c_int {
    let a_rs = match CStr::from_ptr(a).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };

    let b_rs = match CStr::from_ptr(b).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };

    match compare(a_rs, b_rs) {
	Ok(v) => {
	    match v {
		Cmp::Eq => *cmp = version_compare_cmp::CMP_EQ,
		Cmp::Ne => *cmp = version_compare_cmp::CMP_NE,
		Cmp::Lt => *cmp = version_compare_cmp::CMP_LT,
		Cmp::Le => *cmp = version_compare_cmp::CMP_LE,
		Cmp::Ge => *cmp = version_compare_cmp::CMP_GE,
		Cmp::Gt => *cmp = version_compare_cmp::CMP_GT
	    };
	    0
	},
	Err(_) => -1
    }
}

#[no_mangle]
unsafe extern "C" fn version_compare_compare_to(a: *const c_char, b: *const c_char, operator: version_compare_cmp) -> c_int {
    let a_rs = match CStr::from_ptr(a).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };

    let b_rs = match CStr::from_ptr(b).to_str() {
	Ok(v) => v,
	Err(_) => return -1
    };

    let operator_rs = match operator {
	version_compare_cmp::CMP_EQ => Cmp::Eq,
	version_compare_cmp::CMP_NE => Cmp::Ne,
	version_compare_cmp::CMP_LT => Cmp::Lt,
	version_compare_cmp::CMP_LE => Cmp::Le,
	version_compare_cmp::CMP_GE => Cmp::Ge,
	version_compare_cmp::CMP_GT => Cmp::Gt
    };
    match compare_to(a_rs, b_rs, operator_rs) {
	Ok(v) => {
	    if v {
		1
	    } else {
		0
	    }
	},
	Err(_) => -1
    }
}

#[no_mangle]
unsafe extern "C" fn version_compare_manifest_default() -> version_compare_manifest {
    let manifest_rs = Manifest::default();
    let max_depth = match manifest_rs.max_depth {
	Some(v) => v,
	None => 0
    };
    version_compare_manifest {
	max_depth: max_depth,
	ignore_text: if manifest_rs.ignore_text { 1 } else { 0 } 
    }
}

#[no_mangle]
unsafe extern "C" fn version_compare_manifest_has_max_depth(manifest: *mut version_compare_manifest) -> c_int {
    let manifest_rs = Manifest {
	max_depth: if (*manifest).max_depth == 0 { None } else { Some((*manifest).max_depth) },
	ignore_text: if (*manifest).ignore_text == 0 { false } else { true }
    };
    let has = manifest_rs.has_max_depth();
    if has {
	1
    } else {
	0
    }
}
