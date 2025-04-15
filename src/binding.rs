use crate::base32string::Base32String;
use crate::{hotp, totp};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn generate_hotp(secret: &str, count: u64) -> Result<String, PyErr> {
    let b32str = match Base32String::from(secret) {
        Ok(s) => s,
        Err(err) => return Err(PyValueError::new_err(err.msg())),
    };
    let code = hotp::generate(&b32str, &count).to_string();
    Ok(code[code.len() - 6..].to_string())
}

#[pyfunction]
#[pyo3(signature = (code, *, secret, count, tolerance=None))]
fn validate_hotp(
    code: &str,
    secret: &str,
    count: u64,
    tolerance: Option<u64>,
) -> Result<bool, PyErr> {
    let b32str = match Base32String::from(secret) {
        Ok(s) => s,
        Err(err) => return Err(PyValueError::new_err(err.msg())),
    };

    Ok(hotp::validate(code, &b32str, &count, tolerance))
}

#[pyfunction]
fn generate_totp(secret: &str, now: u64) -> Result<String, PyErr> {
    let b32str = match Base32String::from(secret) {
        Ok(s) => s,
        Err(err) => return Err(PyValueError::new_err(err.msg())),
    };
    let code = totp::generate(&b32str, &now).to_string();
    Ok(code[code.len() - 6..].to_string())
}

#[pyfunction]
#[pyo3(signature = (code, *, secret, now, tolerance=None))]
fn validate_totp(
    code: &str,
    secret: &str,
    now: u64,
    tolerance: Option<u64>,
) -> Result<bool, PyErr> {
    let b32str = match Base32String::from(secret) {
        Ok(s) => s,
        Err(err) => return Err(PyValueError::new_err(err.msg())),
    };

    Ok(totp::validate(code, &b32str, &now, tolerance))
}

/// A Python module implemented in Rust.
#[pymodule]
fn otp_rpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_hotp, m)?)?;
    m.add_function(wrap_pyfunction!(validate_hotp, m)?)?;
    m.add_function(wrap_pyfunction!(generate_totp, m)?)?;
    m.add_function(wrap_pyfunction!(validate_totp, m)?)?;
    Ok(())
}
