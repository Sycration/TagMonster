use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::{metadata::InputField, source::Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub name: String,
    pub source: Source,
    pub fields: Vec<InputField>,
    pub entered_data: HashMap<String, Vec<InputField>>,
}
