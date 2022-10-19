use std::os::raw::c_char;
use ffi_support::ExternError;
use cfn_guard::run_checks;

mod types;
mod errors;

use types::FfiValidateInput;
use errors::FfiError;

/**
 * C prototype for this function:
 * typedef struct {
 *   int32_t code;
 *   char *message;
 * } extern_err_t;
 *
 * typedef struct {
 *   char *content;
 *   char *file_name;
 * } validate_input_t;
 *
 * char* cfn_guard_run_checks(validate_input_t &template, validate_input_t &rules, _Bool verbose, extern_err_t * err);
 * void cfn_guard_free_string(char *);
 *
 * if an error is returned, it will be populated in `err`. `cfn_guard_free_string` must be called
 * for the `message` field in `err`.
 *
 * if `err.code` == 0, then the result will be a json string. This `*char` must be passed to
 * `cfn_guard_free_string` to return the memory allocated by rust.
 */
#[no_mangle]
pub extern "C" fn cfn_guard_run_checks<'a>(data: &FfiValidateInput<'a>, rules: &FfiValidateInput<'a>, verbose: c_char, err : &mut ExternError) -> *mut c_char {
    ffi_support::call_with_result(err, || {
        match run_checks(data.into(), rules.into(), verbose == 1) {
            Err(e) => Err(FfiError(e)),
            Ok(r) => Ok(r)
        }
    })
}

ffi_support::define_string_destructor!(cfn_guard_free_string);

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use ffi_support::{ExternError, FfiStr};

    #[test]
    fn run_check() {
        let mut extern_err = ExternError::default();

        let result = cfn_guard_run_checks(&FfiValidateInput {
            content: FfiStr::from_cstr(&CString::new("foo").unwrap()),
            file_name: FfiStr::from_cstr(&CString::new("data").unwrap()),
        }, &FfiValidateInput {
            content: FfiStr::from_cstr(&CString::new("foo").unwrap()),
            file_name: FfiStr::from_cstr(&CString::new("rules").unwrap()),
        }, 0, &mut extern_err);

        assert_eq!(5, extern_err.get_code().code());
        assert_eq!(true, extern_err.get_message().as_str().starts_with("Parser"));
        assert_eq!(0 as *mut c_char, result);
    }
}
