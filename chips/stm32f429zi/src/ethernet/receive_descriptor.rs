#![deny(missing_docs)]
#![deny(dead_code)]
// Copyright 2023 OxidOS Automotive SRL
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// TODO: Add author
// Author:  <>

//! Receive descriptor for Ethernet DMA

use kernel::utilities::registers::{register_bitfields, register_structs, InMemoryRegister};
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::ErrorCode;

register_bitfields![u32,
RDES0 [
    OWN OFFSET(31) NUMBITS(1) [],
    AFM OFFSET(30) NUMBITS(1) [],
    FL OFFSET(16) NUMBITS(14) [],
    ES OFFSET(15) NUMBITS(1) [],
    DE OFFSET(14) NUMBITS(1) [],
    SAF OFFSET(13) NUMBITS(1) [],
    LE OFFSET(12) NUMBITS(1) [],
    OE OFFSET(11) NUMBITS(1) [],
    VLAN OFFSET(10) NUMBITS(1) [],
    FS OFFSET(9) NUMBITS(1) [],
    LS OFFSET(8) NUMBITS(1) [],
    IPHCE_TSV OFFSET(7) NUMBITS(1) [],
    LCO OFFSET(6) NUMBITS(1) [],
    FT OFFSET(5) NUMBITS(1) [],
    RWT OFFSET(4) NUMBITS(1) [],
    RE OFFSET(3) NUMBITS(1) [],
    DBE OFFSET(2) NUMBITS(1) [],
    CE OFFSET(1) NUMBITS(1) [],
    PCE_ESA OFFSET(0) NUMBITS(1) [],
],
RDES1 [
    DIC OFFSET(31) NUMBITS(1) [],
    RBS2 OFFSET(16) NUMBITS(13) [],
    RER OFFSET(15) NUMBITS(1) [],
    RCH OFFSET(14) NUMBITS(1) [],
    RBS1 OFFSET(0) NUMBITS(13) [],
],
];

register_structs! {
    pub(in crate::ethernet) ReceiveDescriptor {
        (0x000 => rdes0: InMemoryRegister<u32, RDES0::Register>),
        (0x004 => rdes1: InMemoryRegister<u32, RDES1::Register>),
        (0x008 => rdes2: InMemoryRegister<u32, ()>),
        (0x00C => rdes3: InMemoryRegister<u32, ()>),
        (0x010 => @END),
    }
}

#[allow(dead_code)]
impl ReceiveDescriptor {
    const MAX_RECEIVE_BUFFER_LENGTH: usize = 1 << 14;
    pub(in crate::ethernet) fn new() -> Self {
        Self {
            rdes0: InMemoryRegister::new(0),
            rdes1: InMemoryRegister::new(0),
            rdes2: InMemoryRegister::new(0),
            rdes3: InMemoryRegister::new(0),
        }
    }

    pub(in crate::ethernet) fn acquire(&self) {
        self.rdes0.modify(RDES0::OWN::SET);
    }

    pub(in crate::ethernet) fn release(&self) {
        self.rdes0.modify(RDES0::OWN::CLEAR);
    }

    pub(in crate::ethernet) fn is_acquired(&self) -> bool {
        self.rdes0.is_set(RDES0::OWN)
    }

    pub(in crate::ethernet) fn get_frame_length(&self) -> usize {
        self.rdes0.read(RDES0::FL) as usize
    }

    pub(in crate::ethernet) fn enable_interrupt_on_completion(&self) {
        self.rdes1.modify(RDES1::DIC::CLEAR);
    }

    pub(in crate::ethernet) fn disable_interrupt_on_completion(&self) {
        self.rdes1.modify(RDES1::DIC::SET);
    }

    pub(in crate::ethernet) fn is_interrupt_on_completion_enabled(&self) -> bool {
        !self.rdes1.is_set(RDES1::DIC)
    }

    pub(in crate::ethernet) fn is_last_segment(&self) -> bool {
        self.rdes0.is_set(RDES0::LS)
    }

    pub(in crate::ethernet) fn is_first_segment(&self) -> bool {
        self.rdes0.is_set(RDES0::FS)
    }

    pub(in crate::ethernet) fn get_error_summary(&self) -> bool {
        self.rdes0.is_set(RDES0::ES)
    }

    pub(in crate::ethernet) fn set_receive_end_of_ring(&self) {
        self.rdes1.modify(RDES1::RER::SET);
    }

    pub(in crate::ethernet) fn clear_receive_end_of_ring(&self) {
        self.rdes1.modify(RDES1::RER::CLEAR);
    }

    pub(in crate::ethernet) fn is_receive_end_of_ring(&self) -> bool {
        self.rdes1.is_set(RDES1::RER)
    }

    pub(in crate::ethernet) fn set_buffer1_size(&self, size: usize) -> Result<(), ErrorCode> {
        if size >= Self::MAX_RECEIVE_BUFFER_LENGTH {
            return Err(ErrorCode::SIZE);
        } else if size % 4 != 0 && size % 8 != 0 && size % 16 != 0 {
            return Err(ErrorCode::FAIL);
        }

        self.rdes1.modify(RDES1::RBS1.val(size as u32));

        Ok(())
    }

    pub(in crate::ethernet) fn get_buffer1_size(&self) -> u16 {
        self.rdes1.read(RDES1::RBS1) as u16
    }

    pub(in crate::ethernet) fn set_buffer1_address(&self, address: u32) {
        self.rdes2.set(address);
    }

    pub(in crate::ethernet) fn get_buffer1_address(&self) -> u32 {
        self.rdes2.get()
    }

    pub(in crate::ethernet) fn set_buffer2_size(&self, size: usize) -> Result<(), ErrorCode> {
        if size >= 1 << 14 {
            return Err(ErrorCode::SIZE);
        } else if size % 4 != 0 && size % 8 != 0 && size % 16 != 0 {
            return Err(ErrorCode::FAIL);
        }

        self.rdes1.modify(RDES1::RBS2.val(size as u32));

        Ok(())
    }

    pub(in crate::ethernet) fn get_buffer2_size(&self) -> u16 {
        self.rdes1.read(RDES1::RBS2) as u16
    }

    pub(in crate::ethernet) fn set_buffer2_address(&self, address: u32) {
        self.rdes3.set(address);
    }

    pub(in crate::ethernet) fn get_buffer2_address(&self) -> u32 {
        self.rdes3.get()
    }
}

#[cfg(test)]
/// Tests for the receive descriptor
pub mod tests {
    use super::*;

    #[test]
    /// Test the receive descriptor
    pub fn test_receive_descriptor() {
        let receive_descriptor = ReceiveDescriptor::new();

        receive_descriptor.acquire();
        assert_eq!(true, receive_descriptor.is_acquired());
        receive_descriptor.release();
        assert_eq!(false, receive_descriptor.is_acquired());

        receive_descriptor.rdes0.modify(RDES0::FL.val(1234));
        assert_eq!(1234, receive_descriptor.get_frame_length());
        receive_descriptor.rdes0.modify(RDES0::FL.val(0));
        assert_eq!(0, receive_descriptor.get_frame_length());

        receive_descriptor.enable_interrupt_on_completion();
        assert_eq!(true, receive_descriptor.is_interrupt_on_completion_enabled());
        receive_descriptor.disable_interrupt_on_completion();
        assert_eq!(false, receive_descriptor.is_interrupt_on_completion_enabled());

        receive_descriptor.rdes0.modify(RDES0::LS::SET);
        assert_eq!(true, receive_descriptor.is_last_segment());
        receive_descriptor.rdes0.modify(RDES0::LS::CLEAR);
        assert_eq!(false, receive_descriptor.is_last_segment());

        receive_descriptor.rdes0.modify(RDES0::FS::SET);
        assert_eq!(true, receive_descriptor.is_first_segment());
        receive_descriptor.rdes0.modify(RDES0::FS::CLEAR);
        assert_eq!(false, receive_descriptor.is_first_segment());

        receive_descriptor.set_receive_end_of_ring();
        assert_eq!(true, receive_descriptor.is_receive_end_of_ring());
        receive_descriptor.clear_receive_end_of_ring();
        assert_eq!(false, receive_descriptor.is_receive_end_of_ring());

        assert_eq!(Ok(()), receive_descriptor.set_buffer1_size(1024));
        assert_eq!(1024, receive_descriptor.get_buffer1_size());
        assert_eq!(Err(ErrorCode::SIZE), receive_descriptor.set_buffer1_size(1 << 14));
        assert_eq!(1024, receive_descriptor.get_buffer1_size());
        assert_eq!(Err(ErrorCode::FAIL), receive_descriptor.set_buffer1_size(1023));
        assert_eq!(1024, receive_descriptor.get_buffer1_size());

        receive_descriptor.set_buffer1_address(0x0040000);
        assert_eq!(0x0040000, receive_descriptor.get_buffer1_address());
        let x: u32 = 2023;
        receive_descriptor.set_buffer1_address(&x as *const u32 as u32);
        assert_eq!(&x as *const u32 as u32, receive_descriptor.get_buffer1_address());

        assert_eq!(Ok(()), receive_descriptor.set_buffer2_size(1024));
        assert_eq!(1024, receive_descriptor.get_buffer2_size());
        assert_eq!(Err(ErrorCode::SIZE), receive_descriptor.set_buffer2_size(1 << 14));
        assert_eq!(1024, receive_descriptor.get_buffer2_size());
        assert_eq!(Err(ErrorCode::FAIL), receive_descriptor.set_buffer2_size(1023));
        assert_eq!(1024, receive_descriptor.get_buffer2_size());

        receive_descriptor.set_buffer2_address(0x0040000);
        assert_eq!(0x0040000, receive_descriptor.get_buffer2_address());
        receive_descriptor.set_buffer2_address(&x as *const u32 as u32);
        assert_eq!(&x as *const u32 as u32, receive_descriptor.get_buffer2_address());

        receive_descriptor.rdes0.modify(RDES0::ES::SET);
        assert_eq!(true, receive_descriptor.get_error_summary());
        receive_descriptor.rdes0.modify(RDES0::ES::CLEAR);
        assert_eq!(false, receive_descriptor.get_error_summary());
    }
}
