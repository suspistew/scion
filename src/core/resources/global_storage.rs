use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct GlobalStorage{
    pub(crate) flags : HashMap<String, bool>
}

