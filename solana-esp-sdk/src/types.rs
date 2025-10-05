/// SDK-wide error type. Keep tiny; map externals into this.
#[derive(Debug)]
pub enum SdkError {
    Crypto,
    Rpc,
    Serialize,
    Deserialize,
    Invalid,
    Timeout,
    Unsupported,
    NetworkError,
    ResponseParseError,
    TransactionTooLarge,
}

pub type Result<T> = core::result::Result<T, SdkError>;
