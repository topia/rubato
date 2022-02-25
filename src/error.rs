use std::error;
use std::fmt;

/// An identifier for a cpu feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFeature {
    /// x86 sse3 cpu feature.
    #[cfg(target_arch = "x86_64")]
    Sse3,
    /// x86_64 avx cpu feature.
    #[cfg(target_arch = "x86_64")]
    Avx,
    /// the fma cpu feature.
    #[cfg(target_arch = "x86_64")]
    Fma,
    /// aarc64 neon cpu feature.
    #[cfg(all(feature = "neon", target_arch = "aarch64"))]
    Neon,
}

impl CpuFeature {
    /// Test if the given CPU feature is detected.
    pub fn is_detected(&self) -> bool {
        match *self {
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Sse3 => {
                is_x86_feature_detected!("sse3")
            }
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Avx => {
                is_x86_feature_detected!("avx")
            }
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Fma => {
                is_x86_feature_detected!("fma")
            }
            #[cfg(all(feature = "neon", target_arch = "aarch64"))]
            CpuFeature::Neon => {
                std::arch::is_aarch64_feature_detected!("neon")
            }
        }
    }
}

#[allow(unused_variables)]
impl fmt::Display for CpuFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Sse3 => {
                write!(f, "sse3")
            }
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Avx => {
                write!(f, "avx")
            }
            #[cfg(target_arch = "x86_64")]
            CpuFeature::Fma => {
                write!(f, "fma")
            }
            #[cfg(all(feature = "neon", target_arch = "aarch64"))]
            CpuFeature::Neon => {
                write!(f, "neon")
            }
        }
    }
}

/// Error raised when trying to use a CPU feature which is not supported.
#[derive(Debug, Clone, Copy)]
pub struct MissingCpuFeature(pub(crate) CpuFeature);

impl fmt::Display for MissingCpuFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing CPU feature `{}`", self.0)
    }
}

impl error::Error for MissingCpuFeature {}

/// The error type returned when constructing [Resampler](crate::Resampler)
pub enum ResamplerConstructionError {
    InvalidSampleRate { input: usize, output: usize },
    InvalidRelativeRatio(f64),
    InvalidRatio(f64),
}

impl fmt::Display for ResamplerConstructionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidSampleRate{input, output} => write!(formatter,
                "Input and output sample rates must both be > 0. Provided input: {}, provided output: {}", input, output
            ),
            Self::InvalidRatio(provided) => write!(formatter,
                "Invalid resample_ratio provided: {}. resample_ratio must be > 0", provided
            ),
            Self::InvalidRelativeRatio(provided) => write!(formatter,
                "Invalid max_resample_ratio_relative provided: {}. max_resample_ratio_relative must be >= 1", provided
            ),
        }
    }
}

impl fmt::Debug for ResamplerConstructionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

impl error::Error for ResamplerConstructionError {}

/// The error type used by `rubato`.
#[derive(Debug)]
pub enum ResampleError {
    /// Error raised when Resample::set_resample_ratio is called with a ratio
    /// that deviates for more than 10% of the original.
    BadRatioUpdate,
    /// Error raised when trying to adjust a synchronous resampler.
    SyncNotAdjustable,
    /// Error raised when the number of channels of the input buffer doesn't match expected.
    WrongNumberOfInputChannels { expected: usize, actual: usize },
    /// Error raised when the number of channels of the output buffer doesn't match expected.
    WrongNumberOfOutputChannels { expected: usize, actual: usize },
    /// Error raised when the number of channels of the mask doesn't match expected.
    WrongNumberOfMaskChannels { expected: usize, actual: usize },
    /// Error raised when the number of frames in a single channel doesn't match
    /// the expected.
    WrongNumberOfInputFrames {
        channel: usize,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for ResampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRatioUpdate => {
                write!(f, "New resample ratio is too far off from original")
            }
            Self::SyncNotAdjustable { .. } => {
                write!(f, "Not possible to adjust a synchronous resampler")
            }
            Self::WrongNumberOfInputChannels { expected, actual } => {
                write!(
                    f,
                    "Wrong number of channels {} in input, expected {}",
                    actual, expected
                )
            }
            Self::WrongNumberOfOutputChannels { expected, actual } => {
                write!(
                    f,
                    "Wrong number of channels {} in output, expected {}",
                    actual, expected
                )
            }
            Self::WrongNumberOfMaskChannels { expected, actual } => {
                write!(
                    f,
                    "Wrong number of channels {} in mask, expected {}",
                    actual, expected
                )
            }
            Self::WrongNumberOfInputFrames {
                channel,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Wrong number of frames {} in input channel {}, expected {}",
                    actual, channel, expected
                )
            }
        }
    }
}

impl error::Error for ResampleError {}

/// A result alias for the error type used by `rubato`.
pub type ResampleResult<T> = ::std::result::Result<T, ResampleError>;
