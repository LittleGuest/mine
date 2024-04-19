use crate::error::MineError;

pub type MineResult<T, E = MineError> = std::result::Result<T, E>;
