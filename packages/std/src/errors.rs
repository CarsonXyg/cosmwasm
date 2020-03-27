use serde::Serialize;
use snafu::Snafu;

#[derive(Debug, Snafu, Serialize)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Invalid Base64 string: {}", source))]
    Base64Err {
        #[serde(serialize_with = "serialize_as_string")]
        source: base64::DecodeError,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Contract error: {}", msg))]
    ContractErr {
        msg: &'static str,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Contract error: {}", msg))]
    DynContractErr {
        msg: String,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("{} not found", kind))]
    NotFound {
        kind: &'static str,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Received null pointer, refuse to use"))]
    NullPointer {
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error parsing {}: {}", kind, source))]
    ParseErr {
        kind: &'static str,
        #[serde(serialize_with = "serialize_as_string")]
        source: serde_json_wasm::de::Error,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error serializing {}: {}", kind, source))]
    SerializeErr {
        kind: &'static str,
        #[serde(serialize_with = "serialize_as_string")]
        source: serde_json_wasm::ser::Error,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    // This is used for std::str::from_utf8, which we may well deprecate
    #[snafu(display("UTF8 encoding error: {}", source))]
    Utf8Err {
        #[serde(serialize_with = "serialize_as_string")]
        source: std::str::Utf8Error,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    // This is used for String::from_utf8, which does zero-copy from Vec<u8>, moving towards this
    #[snafu(display("UTF8 encoding error: {}", source))]
    Utf8StringErr {
        #[serde(serialize_with = "serialize_as_string")]
        source: std::string::FromUtf8Error,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Unauthorized"))]
    Unauthorized {
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Invalid {}: {}", field, msg))]
    ValidationErr {
        field: &'static str,
        msg: &'static str,
        #[serde(skip)]
        backtrace: snafu::Backtrace,
    },
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn invalid<T>(field: &'static str, msg: &'static str) -> Result<T> {
    ValidationErr { field, msg }.fail()
}

pub fn contract_err<T>(msg: &'static str) -> Result<T> {
    ContractErr { msg }.fail()
}

pub fn dyn_contract_err<T>(msg: String) -> Result<T> {
    DynContractErr { msg }.fail()
}

pub fn unauthorized<T>() -> Result<T> {
    Unauthorized {}.fail()
}

/// serialize_as_string allows us to serialize source errors with the important info
fn serialize_as_string<T, S>(err: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: std::fmt::Display,
{
    let msg = format!("{}", err);
    serde::Serialize::serialize(&msg, serializer)
}

pub use api::ApiError;

// place this in a submod, so the auto-generated contexts don't conflict with same-named context from above
mod api {
    use serde::{Deserialize, Serialize};
    use snafu::{ResultExt, Snafu};
    use std::convert::TryFrom;

    use super::Error;
    use crate::serde::{from_slice, to_vec};

    /// ApiError is a "rehydrated" Error after it has been Serialized and restored.
    /// This will not contain all information of the original (source error and backtrace cannot be serialized),
    /// but we aim to ensure the following:
    /// 1. A rehydrated ApiError will have the same type as the original Error
    /// 2. A rehydrated ApiError will have the same display as the original
    /// 3. Serializing and Deserializing an ApiError will give you an identical struct
    #[derive(Debug, Snafu, Serialize, Deserialize, PartialEq)]
    pub enum ApiError {
        #[snafu(display("Invalid Base64 string: {}", source))]
        Base64Err {
            #[snafu(source(false))]
            source: String,
        },
        #[snafu(display("Contract error: {}", msg))]
        ContractErr { msg: String },
        #[snafu(display("Contract error: {}", msg))]
        DynContractErr { msg: String },
        #[snafu(display("{} not found", kind))]
        NotFound { kind: String },
        #[snafu(display("Received null pointer, refuse to use"))]
        NullPointer {},
        #[snafu(display("Error parsing {}: {}", kind, source))]
        ParseErr {
            kind: String,
            #[snafu(source(false))]
            source: String,
        },
        #[snafu(display("Error serializing {}: {}", kind, source))]
        SerializeErr {
            kind: String,
            #[snafu(source(false))]
            source: String,
        },
        // This is used for std::str::from_utf8, which we may well deprecate
        #[snafu(display("UTF8 encoding error: {}", source))]
        Utf8Err {
            #[snafu(source(false))]
            source: String,
        },
        // This is used for String::from_utf8, which does zero-copy from Vec<u8>, moving towards this
        #[snafu(display("UTF8 encoding error: {}", source))]
        Utf8StringErr {
            #[snafu(source(false))]
            source: String,
        },
        #[snafu(display("Unauthorized"))]
        Unauthorized {},
        #[snafu(display("Invalid {}: {}", field, msg))]
        ValidationErr { field: String, msg: String },
    }

    impl TryFrom<Error> for ApiError {
        type Error = Error;

        fn try_from(value: Error) -> Result<Self, Self::Error> {
            let ser = to_vec(&value).context(super::SerializeErr { kind: "Error" })?;
            from_slice(&ser).context(super::ParseErr { kind: "ApiError" })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::serde::{from_slice, to_vec};
    use snafu::ResultExt;
    use std::convert::TryInto;

    #[test]
    fn use_invalid() {
        let e: Result<()> = invalid("demo", "not implemented");
        match e {
            Err(Error::ValidationErr { field, msg, .. }) => {
                assert_eq!(field, "demo");
                assert_eq!(msg, "not implemented");
            }
            Err(e) => panic!("unexpected error, {:?}", e),
            Ok(_) => panic!("invalid must return error"),
        }
    }

    #[test]
    // example of reporting static contract errors
    fn contract_helper() {
        let e: Result<()> = contract_err("not implemented");
        match e {
            Err(Error::ContractErr { msg, .. }) => {
                assert_eq!(msg, "not implemented");
            }
            Err(e) => panic!("unexpected error, {:?}", e),
            Ok(_) => panic!("contract_err must return error"),
        }
    }

    #[test]
    // example of reporting contract errors with format!
    fn dyn_contract_helper() {
        let guess = 7;
        let e: Result<()> = dyn_contract_err(format!("{} is too low", guess));
        match e {
            Err(Error::DynContractErr { msg, .. }) => {
                assert_eq!(msg, String::from("7 is too low"));
            }
            Err(e) => panic!("unexpected error, {:?}", e),
            Ok(_) => panic!("dyn_contract_err must return error"),
        }
    }

    fn assert_serializable(r: Result<()>) {
        let error = r.unwrap_err();
        let orig_ser = to_vec(&error).unwrap();
        let rehydrated: ApiError = from_slice(&orig_ser).unwrap();
        assert_eq!(format!("{}", error), format!("{}", rehydrated));
        let round_trip: ApiError = from_slice(&to_vec(&rehydrated).unwrap()).unwrap();
        assert_eq!(round_trip, rehydrated);
    }

    #[test]
    fn base64_serializable() {
        let source = Err(base64::DecodeError::InvalidLength);
        assert_serializable(source.context(Base64Err {}));
    }

    #[test]
    fn contract_serializable() {
        assert_serializable(contract_err("foobar"));
    }

    #[test]
    fn dyn_contract_serializable() {
        assert_serializable(dyn_contract_err("dynamic".to_string()));
    }

    #[test]
    fn invalid_serializable() {
        assert_serializable(invalid("name", "too short"));
    }

    #[test]
    fn unauthorized_serializable() {
        assert_serializable(unauthorized());
    }

    #[test]
    fn null_pointer_serializable() {
        assert_serializable(NullPointer {}.fail());
    }

    #[test]
    fn not_found_serializable() {
        assert_serializable(NotFound { kind: "State" }.fail());
    }

    #[test]
    fn parse_err_serializable() {
        let err = from_slice::<String>(b"123")
            .context(ParseErr { kind: "String" })
            .map(|_| ());
        assert_serializable(err);
    }

    #[test]
    fn serialize_err_serializable() {
        let source = Err(serde_json_wasm::ser::Error::BufferFull);
        assert_serializable(source.context(SerializeErr { kind: "faker" }));
    }

    #[test]
    fn try_from_error_conversion() {
        let source: Result<(), _> = Err(serde_json_wasm::ser::Error::BufferFull);
        let error = source.context(SerializeErr { kind: "faker" }).unwrap_err();
        let msg = format!("{}", error);
        let api_error: ApiError = error.try_into().unwrap();
        assert_eq!(msg, format!("{}", api_error));
    }
}
