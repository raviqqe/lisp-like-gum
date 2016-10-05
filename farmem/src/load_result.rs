use std::result;

use load_error::LoadError;



pub type LoadResult<T> = result::Result<T, LoadError>;
