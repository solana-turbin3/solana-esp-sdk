/// SDK-wide error type. Keep tiny; map externals into this.
#[derive(Debug)]
pub enum SdkError {
    Crypto,
    Rpc,
    Network,
    Serialize,
    Deserialize,
    Invalid,
    Timeout,
    Unsupported,
}

pub type Result<T> = core::result::Result<T, SdkError>;