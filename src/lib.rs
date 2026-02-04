use pyo3::prelude::*;
use notepad_parser::NotepadTabStat;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum PyEncoding {
    ANSI = 0x01,
    UTF16LE = 0x02,
    UTF16BE = 0x03,
    UTF8BOM = 0x04,
    UTF8 = 0x05,
    UNKNOWN = 0x00,
}

#[pymethods]
impl PyEncoding {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyEncoding::ANSI => "ANSI",
            PyEncoding::UTF16LE => "UTF16LE",
            PyEncoding::UTF16BE => "UTF16BE",
            PyEncoding::UTF8BOM => "UTF8BOM",
            PyEncoding::UTF8 => "UTF8",
            PyEncoding::UNKNOWN => "UNKNOWN",
        }
    }

    fn __repr__(&self) -> String {
        format!("PyEncoding.{}", self.name())
    }

    fn __str__(&self) -> &str {
        self.name()
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum PyCRType {
    CRLF = 0x01,
    CR = 0x02,
    LF = 0x03,
    UNKNOWN = 0x00,
}

#[pymethods]
impl PyCRType {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyCRType::CRLF => "CRLF",
            PyCRType::CR => "CR",
            PyCRType::LF => "LF",
            PyCRType::UNKNOWN => "UNKNOWN",
        }
    }

    fn __repr__(&self) -> String {
        format!("PyCRType.{}", self.name())
    }

    fn __str__(&self) -> &str {
        self.name()
    }
}

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
    pub encoding: Option<PyEncoding>,
    #[pyo3(get)]
    pub cr_type: Option<PyCRType>,
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
                    notepad_parser::enums::Encoding::ANSI => PyEncoding::ANSI,
                    notepad_parser::enums::Encoding::UTF16LE => PyEncoding::UTF16LE,
                    notepad_parser::enums::Encoding::UTF16BE => PyEncoding::UTF16BE,
                    notepad_parser::enums::Encoding::UTF8BOM => PyEncoding::UTF8BOM,
                    notepad_parser::enums::Encoding::UTF8 => PyEncoding::UTF8,
                    notepad_parser::enums::Encoding::UNKNOWN(_) => PyEncoding::UNKNOWN,
                }),
                cr_type: state.cr_type.map(|c| match c {
                    notepad_parser::enums::CRType::CRLF => PyCRType::CRLF,
                    notepad_parser::enums::CRType::CR => PyCRType::CR,
                    notepad_parser::enums::CRType::LF => PyCRType::LF,
                    notepad_parser::enums::CRType::UNKNOWN(_) => PyCRType::UNKNOWN,
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
    m.add_class::<PyEncoding>()?;
    m.add_class::<PyCRType>()?;
    m.add_function(wrap_pyfunction!(parse_bin, m)?)?;
    Ok(())
}
