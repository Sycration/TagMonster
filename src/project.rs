use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::source::Source;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub name: String,
    pub source: Source,

}
