use pyo3::prelude::*;
use notepad_parser::NotepadTabStat;

#[pyclass]
pub struct PyTabState {
    #[pyo3(get)]
    pub tabstate_path: Option<String>,
    #[pyo3(get)]
    pub is_saved_file: bool,
    #[pyo3(get)]
    pub path: Option<String>,
    #[pyo3(get)]
    pub file_size: Option<u64>,
    #[pyo3(get)]
    pub encoding: Option<String>,
    #[pyo3(get)]
    pub cr_type: Option<String>,
    #[pyo3(get)]
    pub file_hash: Option<String>,
    #[pyo3(get)]
    pub cursor_start: Option<u64>,
    #[pyo3(get)]
    pub cursor_end: Option<u64>,
    #[pyo3(get)]
    pub word_wrap: bool,
    #[pyo3(get)]
    pub rtl: bool,
    #[pyo3(get)]
    pub show_unicode: bool,
    #[pyo3(get)]
    pub version: u64,
    #[pyo3(get)]
    pub file_content: String,
    #[pyo3(get)]
    pub contain_unsaved_data: bool,
    #[pyo3(get)]
    pub checksum: String,
    #[pyo3(get)]
    pub unsaved_chunks_str: Option<String>,
}

#[pyfunction]
fn parse_bin(path: String) -> PyResult<PyTabState> {
    match NotepadTabStat::from_path(&path) {
        Ok(state) => {
            Ok(PyTabState {
                tabstate_path: state.tabstate_path,
                is_saved_file: state.is_saved_file,
                path: state.path,
                file_size: state.file_size,
                encoding: state.encoding.map(|e| match e {
                    notepad_parser::enums::Encoding::ANSI => "ANSI".to_string(),
                    notepad_parser::enums::Encoding::UTF16LE => "UTF16LE".to_string(),
                    notepad_parser::enums::Encoding::UTF16BE => "UTF16BE".to_string(),
                    notepad_parser::enums::Encoding::UTF8BOM => "UTF8BOM".to_string(),
                    notepad_parser::enums::Encoding::UTF8 => "UTF8".to_string(),
                    notepad_parser::enums::Encoding::UNKNOWN(x) => format!("UNKNOWN({})", x),
                }),
                cr_type: state.cr_type.map(|c| match c {
                    notepad_parser::enums::CRType::CRLF => "CRLF".to_string(),
                    notepad_parser::enums::CRType::CR => "CR".to_string(),
                    notepad_parser::enums::CRType::LF => "LF".to_string(),
                    notepad_parser::enums::CRType::UNKNOWN(x) => format!("UNKNOWN({})", x),
                }),
                file_hash: state.file_hash,
                cursor_start: state.cursor_start,
                cursor_end: state.cursor_end,
                word_wrap: state.config_block.word_wrap,
                rtl: state.config_block.rtl,
                show_unicode: state.config_block.show_unicode,
                version: state.config_block.version,
                file_content: state.file_content,
                contain_unsaved_data: state.contain_unsaved_data,
                checksum: state.checksum,
                unsaved_chunks_str: state.unsaved_chunks_str,
            })
        },
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("Failed to parse notepad bin: {}", e))),
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTabState>()?;
    m.add_function(wrap_pyfunction!(parse_bin, m)?)?;
    Ok(())
}
