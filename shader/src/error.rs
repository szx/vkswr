//! Errors produced while loading and lowering shaders.

/// An error produced while preparing a shader for execution.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The SPIR-V binary could not be parsed.
    #[error("could not parse SPIR-V shader {name:?}")]
    ParseSpirv {
        name: String,
        #[source]
        source: rspirv::binary::ParseState,
    },

    /// The SPIR-V module violates an invariant of the interpreter.
    #[error("invalid SPIR-V shader {name:?}: {message}")]
    InvalidSpirv { name: String, message: String },

    /// The SPIR-V module uses a feature that is not implemented.
    #[error("unsupported SPIR-V feature in shader {name:?}: {message}")]
    UnsupportedSpirv { name: String, message: String },

    /// The lowered shader IR uses a feature that is not implemented.
    #[error("unsupported shader IR feature in shader {name:?}: {message}")]
    UnsupportedIl { name: String, message: String },
}

impl Error {
    pub(crate) fn invalid_spirv(name: &str, message: impl Into<String>) -> Self {
        Self::InvalidSpirv {
            name: name.to_owned(),
            message: message.into(),
        }
    }

    pub(crate) fn unsupported_spirv(name: &str, message: impl Into<String>) -> Self {
        Self::UnsupportedSpirv {
            name: name.to_owned(),
            message: message.into(),
        }
    }

    pub(crate) fn unsupported_il(name: &str, message: impl Into<String>) -> Self {
        Self::UnsupportedIl {
            name: name.to_owned(),
            message: message.into(),
        }
    }
}

// `Shader::new` requires a `Send + Sync + 'static` error.
const _: () = {
    const fn assert_error_traits<E: std::error::Error + Send + Sync + 'static>() {}
    assert_error_traits::<Error>();
};
