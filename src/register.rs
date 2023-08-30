use crate::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Register {
    Al,
    Ax,
    Cl,
    Cx,
    Dl,
    Dx,
    Bl,
    Bx,
    Ah,
    Sp,
    Ch,
    Bp,
    Dh,
    Si,
    Bh,
    Di,
}

use Register::*;

impl From<Byte> for Register {
    fn from(value: Byte) -> Self {
        match 0b0000_1111 & value {
            0b0000 => Al,
            0b0001 => Ax,
            0b0010 => Cl,
            0b0011 => Cx,
            0b0100 => Dl,
            0b0101 => Dx,
            0b0110 => Bl,
            0b0111 => Bx,
            0b1000 => Ah,
            0b1001 => Sp,
            0b1010 => Ch,
            0b1011 => Bp,
            0b1100 => Dh,
            0b1101 => Si,
            0b1110 => Bh,
            0b1111 => Di,
            _ => panic!("Invalid register bits"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Al => f.write_str("al"),
            Ax => f.write_str("ax"),
            Cl => f.write_str("cl"),
            Cx => f.write_str("cx"),
            Dl => f.write_str("dl"),
            Dx => f.write_str("dx"),
            Bl => f.write_str("bl"),
            Bx => f.write_str("bx"),
            Ah => f.write_str("ah"),
            Sp => f.write_str("sp"),
            Ch => f.write_str("ch"),
            Bp => f.write_str("bp"),
            Dh => f.write_str("dh"),
            Si => f.write_str("si"),
            Bh => f.write_str("bh"),
            Di => f.write_str("di"),
        }
    }
}
