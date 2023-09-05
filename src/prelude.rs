pub type Byte = u8;
pub type Word = u16;
pub type SignedByte = i8;
pub type SignedWord = i16;
pub type Wide = bool;
pub type DestinationFirst = bool;

pub use crate::{bit_match, compute_mask, compute_val};
use std::error::Error;

pub type BoxDynError = Box<dyn Error>;
