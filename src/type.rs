/// Define an enumeration for version comparison
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VersionComparison {
    Greater,
    Less,
    Equal,
}

/// Define a version structure
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Version {
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) patch: u32,
    pub(crate) pre_release: Option<String>, // Supports pre-release tags
}
