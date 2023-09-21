use crate::instructions::operands::ImmediateValue;
use crate::memory::{ByteMemory, EffectiveAddress};
use crate::{FlagRegisterManager, MemoryManager, RegisterManager, SegmentRegisterManager};

#[derive(Debug, Default)]
pub struct Store {
    register_store: RegisterManager,
    memory_store: MemoryManager,
    segment_register_store: SegmentRegisterManager,
    flag_register_store: FlagRegisterManager,
}

impl Store {
    pub fn register_store(&self) -> &RegisterManager {
        &self.register_store
    }

    pub fn memory_store(&self) -> &MemoryManager {
        &self.memory_store
    }

    pub fn segment_register_store(&self) -> &SegmentRegisterManager {
        &self.segment_register_store
    }

    pub fn flag_register_store(&self) -> &FlagRegisterManager {
        &self.flag_register_store
    }

    pub fn register_store_mut(&mut self) -> &mut RegisterManager {
        &mut self.register_store
    }

    pub fn memory_store_mut(&mut self) -> &mut MemoryManager {
        &mut self.memory_store
    }

    pub fn segment_register_store_mut(&mut self) -> &mut SegmentRegisterManager {
        &mut self.segment_register_store
    }

    pub fn flag_register_store_mut(&mut self) -> &mut FlagRegisterManager {
        &mut self.flag_register_store
    }

    pub fn write_to_effective_memory_address(
        &mut self,
        address: EffectiveAddress,
        is_wide: bool,
        value: ImmediateValue,
    ) {
        let register_store = &self.register_store;
        let memory_store = &mut self.memory_store;
        let address = memory_store.effective_address_to_address(address, register_store);

        if is_wide {
            memory_store.write_word(address, value.into())
        } else {
            memory_store.write_byte(
                address,
                value.try_into().expect("Not is wide but value is word"),
            )
        }
    }
}
