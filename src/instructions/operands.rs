use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::register::Register;
use crate::segment_register::SegmentRegister;
use crate::store::Store;
use crate::{BoxDynError, Byte, SignedByte, SignedWord, Wide};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq)]
pub enum ImmediateValue {
    SignedByte(i8),
    SignedWord(i16),
}

impl From<i16> for ImmediateValue {
    fn from(value: i16) -> Self {
        Self::SignedWord(value)
    }
}

impl From<i8> for ImmediateValue {
    fn from(value: i8) -> Self {
        Self::SignedByte(value)
    }
}

impl From<ImmediateValue> for i16 {
    fn from(value: ImmediateValue) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        }
    }
}

impl TryFrom<ImmediateValue> for i8 {
    type Error = BoxDynError;

    fn try_from(value: ImmediateValue) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(value),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

impl From<ImmediateValue> for u16 {
    fn from(value: ImmediateValue) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => u16::from_le_bytes((value as i16).to_le_bytes()),
            ImmediateValue::SignedWord(value) => u16::from_le_bytes((value).to_le_bytes()),
        }
    }
}

impl TryFrom<ImmediateValue> for u8 {
    type Error = BoxDynError;

    fn try_from(value: ImmediateValue) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(u8::from_le_bytes(value.to_le_bytes())),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

impl From<ArithmeticResult> for i16 {
    fn from(ArithmeticResult { value, .. }: ArithmeticResult) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        }
    }
}

impl TryFrom<ArithmeticResult> for i8 {
    type Error = BoxDynError;

    fn try_from(ArithmeticResult { value, .. }: ArithmeticResult) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(value),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

impl From<ArithmeticResult> for u16 {
    fn from(ArithmeticResult { value, .. }: ArithmeticResult) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => u16::from_le_bytes((value as i16).to_le_bytes()),
            ImmediateValue::SignedWord(value) => u16::from_le_bytes((value).to_le_bytes()),
        }
    }
}

impl TryFrom<ArithmeticResult> for u8 {
    type Error = BoxDynError;

    fn try_from(ArithmeticResult { value, .. }: ArithmeticResult) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(u8::from_le_bytes(value.to_le_bytes())),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArithmeticResult<Value = ImmediateValue> {
    value: Value,
    pub carry: bool,
    pub auxiliary_carry: bool,
    pub overflow: bool,
    pub zero: bool,
    pub sign: bool,
}

impl<Value> ArithmeticResult<Value> {
    pub fn value(self) -> Value {
        self.value
    }
}

fn compute_carry_on_addition(lhs: i16, rhs: i16, value: i16) -> (bool, bool) {
    let msb_lhs_set = ((lhs >> 15) & 1) == 1;
    let msb_rhs_set = ((rhs >> 15) & 1) == 1;

    let msb_lhs_lower_nibble = ((lhs >> 3) & 1) == 1;
    let msb_rhs_lower_nibble = ((lhs >> 3) & 1) == 1;

    let carry = (msb_lhs_set || msb_rhs_set) && ((value >> 15) & 1) == 0;
    let auxiliary_carry = (msb_lhs_lower_nibble || msb_rhs_lower_nibble) && ((value >> 3) & 1) == 0;

    (carry, auxiliary_carry)
}

fn compute_borrow_on_subtraction(lhs: i16, rhs: i16, value: i16) -> (bool, bool) {
    let msb_lhs_set = ((lhs >> 15) & 1) == 0;
    let msb_rhs_set = ((rhs >> 15) & 1) == 0;

    let msb_lhs_lower_nibble = ((lhs >> 3) & 1) == 0;
    let msb_rhs_lower_nibble = ((lhs >> 3) & 1) == 0;

    let carry = (msb_lhs_set || msb_rhs_set) && ((value >> 15) & 1) == 1;
    let auxiliary_carry = (msb_lhs_lower_nibble || msb_rhs_lower_nibble) && ((value >> 3) & 1) == 1;

    (carry, auxiliary_carry)
}

impl Add for ImmediateValue {
    type Output = ArithmeticResult;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let rhs = match rhs {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_add(rhs);
        let (carry, auxiliary_carry) = compute_carry_on_addition(lhs, rhs, value);

        ArithmeticResult {
            value: ImmediateValue::SignedWord(value),
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Sub for ImmediateValue {
    type Output = ArithmeticResult;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let rhs = match rhs {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_sub(rhs);
        let (carry, auxiliary_carry) = compute_borrow_on_subtraction(lhs, rhs, value);

        ArithmeticResult {
            value: ImmediateValue::SignedWord(value),
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Add<SignedByte> for ImmediateValue {
    type Output = ArithmeticResult<i16>;

    fn add(self, rhs: SignedByte) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_add(rhs as i16);
        let (carry, auxiliary_carry) = compute_carry_on_addition(lhs, rhs as i16, value);

        ArithmeticResult {
            value,
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Sub<SignedByte> for ImmediateValue {
    type Output = ArithmeticResult<i16>;

    fn sub(self, rhs: SignedByte) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_sub(rhs as i16);
        let (carry, auxiliary_carry) = compute_borrow_on_subtraction(lhs, rhs as i16, value);

        ArithmeticResult {
            value,
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Add<SignedWord> for ImmediateValue {
    type Output = ArithmeticResult<i16>;

    fn add(self, rhs: SignedWord) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_add(rhs);
        let (carry, auxiliary_carry) = compute_carry_on_addition(lhs, rhs, value);

        ArithmeticResult {
            value,
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Sub<SignedWord> for ImmediateValue {
    type Output = ArithmeticResult<i16>;

    fn sub(self, rhs: SignedWord) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let (value, overflow) = lhs.overflowing_sub(rhs);
        let (carry, auxiliary_carry) = compute_borrow_on_subtraction(lhs, rhs, value);

        ArithmeticResult {
            value,
            zero: value == 0,
            overflow,
            carry,
            auxiliary_carry,
            sign: value.is_negative(),
        }
    }
}

impl Display for ImmediateValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateValue::SignedByte(value) => Display::fmt(value, f),
            ImmediateValue::SignedWord(value) => Display::fmt(value, f),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Operand {
    Accumulator,
    AccumulatorWide,
    Register(Register),
    SegmentRegister(SegmentRegister),
    Memory(EffectiveAddress),
    Immediate(ImmediateValue),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Accumulator => f.write_str("al"),
            Operand::AccumulatorWide => f.write_str("ax"),
            Operand::Register(register) => register.fmt(f),
            Operand::Memory(memory) => memory.fmt(f),
            Operand::Immediate(immediate) => immediate.fmt(f),
            Operand::SegmentRegister(register) => register.fmt(f),
        }
    }
}

impl Operand {
    pub fn read(
        reader: &mut BufReader<File>,
        mode: InstructionMode,
        target_specifier_byte: Byte,
        is_wide: Wide,
    ) -> Self {
        let mem_bytes = 0b00_000_111 & target_specifier_byte;

        if let InstructionMode::Register = mode {
            let mut mem_bytes = mem_bytes << 1;

            if is_wide {
                mem_bytes += 1;
            }

            Operand::Register(Register::from(mem_bytes))
        } else {
            Operand::Memory(EffectiveAddress::read(reader, mode, mem_bytes))
        }
    }

    pub fn to_immediate_value(self, is_wide_op: bool, store: &mut Store) -> ImmediateValue {
        match self {
            Operand::Accumulator => ImmediateValue::SignedByte(i8::from_le_bytes([store
                .register_store()
                .read_byte_from_register(Register::Al)])),
            Operand::AccumulatorWide => ImmediateValue::SignedWord(i16::from_le_bytes(
                store
                    .register_store()
                    .read_word_from_register(Register::Ax)
                    .to_le_bytes(),
            )),
            Operand::Register(register) => {
                if is_wide_op {
                    ImmediateValue::SignedWord(i16::from_le_bytes(
                        store
                            .register_store()
                            .read_word_from_register(register)
                            .to_le_bytes(),
                    ))
                } else {
                    ImmediateValue::SignedByte(i8::from_le_bytes([store
                        .register_store()
                        .read_byte_from_register(register)]))
                }
            }
            Operand::Memory(address) => store.memory_store().read_memory_from_effective_address(
                address,
                is_wide_op,
                store.register_store(),
            ),
            Operand::Immediate(immediate_value) => immediate_value,
            Operand::SegmentRegister(register) => {
                if is_wide_op {
                    ImmediateValue::SignedWord(
                        store
                            .segment_register_store()
                            .read_word_from_segment_register(register)
                            as i16,
                    )
                } else {
                    ImmediateValue::SignedByte(
                        store
                            .segment_register_store()
                            .read_byte_from_segment_register(register)
                            as i8,
                    )
                }
            }
        }
    }

    pub fn write_value(self, value: ImmediateValue, is_wide_op: bool, store: &mut Store) {
        match self {
            Operand::Accumulator => match value {
                ImmediateValue::SignedByte(value) => {
                    store.register_store_mut().write_byte_to_register(
                        Register::Al,
                        u8::from_le_bytes(value.to_le_bytes()),
                    );
                }
                ImmediateValue::SignedWord(_) => {
                    panic!("Cannot write word to Al")
                }
            },
            Operand::AccumulatorWide => match value {
                ImmediateValue::SignedByte(value) => {
                    store.register_store_mut().write_word_to_register(
                        Register::Ax,
                        u8::from_le_bytes(value.to_le_bytes()) as u16,
                    );
                }
                ImmediateValue::SignedWord(value) => store
                    .register_store_mut()
                    .write_word_to_register(Register::Ax, u16::from_le_bytes(value.to_le_bytes())),
            },
            Operand::Register(register) => match value {
                ImmediateValue::SignedByte(value) => {
                    if is_wide_op {
                        store.register_store_mut().write_word_to_register(
                            register,
                            u8::from_le_bytes(value.to_le_bytes()) as u16,
                        );
                    } else {
                        store.register_store_mut().write_byte_to_register(
                            register,
                            u8::from_le_bytes(value.to_le_bytes()),
                        );
                    }
                }
                ImmediateValue::SignedWord(value) => store
                    .register_store_mut()
                    .write_word_to_register(register, u16::from_le_bytes(value.to_le_bytes())),
            },
            Operand::Memory(address) => {
                store.write_to_effective_memory_address(address, is_wide_op, value)
            }
            Operand::Immediate(_) => panic!("Cannot move a value to immediate"),
            Operand::SegmentRegister(register) => {
                if is_wide_op {
                    store
                        .segment_register_store_mut()
                        .write_word_to_segment_register(register, value.into())
                } else {
                    store
                        .segment_register_store_mut()
                        .write_word_to_segment_register(register, value.try_into().unwrap())
                }
            }
        }
    }
}
