use pyo3::prelude::*;
use pyo3::types::PyList;

extern crate base_x;

/*
const BASE_LIST: &str = "1234567890,-";
const BASE_BF: &str = "+-<>[]{}";
const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
*/

#[pyfunction]
fn encode(alphabet: &str, input: &PyList) -> PyResult<String> {
    // convert the input to a vector of u8
    let input: Vec<u8> = input.extract()?;
    let ret = base_x::encode(alphabet, &input);
    Ok(ret)
}


/// Decode an input vector using the given alphabet.
#[pyfunction]
fn decode(alphabet: &str, input: &str) -> PyResult<Vec<u8>> {
    let decoded = base_x::decode(alphabet, input).unwrap();
    Ok(decoded)
}


/// A Python module implemented in Rust.
#[pymodule]
fn base_x_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;    
    Ok(())
}