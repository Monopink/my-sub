//! Configuration type definitions used by the parser.
//! For the main proxy model definitions, see crate::models

/// Enum representing different configuration formats supported by the parser
#[derive(Debug, Clone, PartialEq)]
pub enum ConfType {
    Unknown,
    SS,
    SSR,
    V2Ray,
    SSConf,
    SSTap,
    Netch,
    SOCKS,
    HTTP,
    SUB,
    Local,
}
