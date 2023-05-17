mod signal_type;

use pyo3::prelude::*;

#[pymodule]
fn simulated_signal_data(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<signal_type::SignalType>()?;
    m.add_wrapped(wrap_pyfunction!(signal_type::get_signal_types))?;
    Ok(())
}
