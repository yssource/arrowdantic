use pyo3::prelude::*;

use arrow2::io::ipc;
use arrow2::io::parquet;

use super::file_like;
use super::Chunk;
use super::Error;

#[pyclass]
pub struct ArrowFileReader(ipc::read::FileReader<file_like::FileOrFileLike>);

#[pymethods]
impl ArrowFileReader {
    #[new]
    fn new(obj: PyObject) -> PyResult<Self> {
        let mut reader = file_like::FileOrFileLike::from_pyobject(obj)?;

        let metadata = ipc::read::read_file_metadata(&mut reader).map_err(Error)?;
        let reader = ipc::read::FileReader::new(reader, metadata, None);

        Ok(Self(reader))
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Chunk>> {
        let chunk = slf.0.next().transpose().map_err(Error)?;
        Ok(chunk.map(Chunk))
    }
}

#[pyclass]
pub struct ParquetFileReader(parquet::read::FileReader<file_like::FileOrFileLike>);

#[pymethods]
impl ParquetFileReader {
    #[new]
    fn new(obj: PyObject) -> PyResult<Self> {
        let reader = file_like::FileOrFileLike::from_pyobject(obj)?;

        let reader =
            parquet::read::FileReader::try_new(reader, None, None, None, None).map_err(Error)?;

        Ok(Self(reader))
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Chunk>> {
        let chunk = slf.0.next().transpose().map_err(Error)?;
        Ok(chunk.map(Chunk))
    }
}