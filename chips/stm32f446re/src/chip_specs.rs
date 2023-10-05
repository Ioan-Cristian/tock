// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OxidOS Automotive SRL.
//
// Author: Ioan-Cristian CÎRSTEA <ioan.cirstea@oxidos.io>

//! STM32F446 specifications

use stm32f4xx::chip_specific::clock_constants::{PllConstants, SystemClockConstants};
use stm32f4xx::chip_specific::flash::{FlashChipSpecific, FlashLatency16};

pub enum Stm32f446Specs {}

impl PllConstants for Stm32f446Specs {
    const MIN_FREQ_MHZ: usize = 13;
}

impl SystemClockConstants for Stm32f446Specs {
    const APB1_FREQUENCY_LIMIT_MHZ: usize = 45;
    const SYS_CLOCK_FREQUENCY_LIMIT_MHZ: usize = 168;
}

impl FlashChipSpecific for Stm32f446Specs {
    type FlashLatency = FlashLatency16;

    fn get_number_wait_cycles_based_on_frequency(frequency_mhz: usize) -> Self::FlashLatency {
        match frequency_mhz {
            0..=30 => Self::FlashLatency::Latency0,
            31..=60 => Self::FlashLatency::Latency1,
            61..=90 => Self::FlashLatency::Latency2,
            91..=120 => Self::FlashLatency::Latency3,
            121..=150 => Self::FlashLatency::Latency4,
            _ => Self::FlashLatency::Latency5,
        }
    }
}

/// STM32F446 Clock module
pub mod clocks {
    use crate::chip_specs::Stm32f446Specs;

    /// STM32F446 Clocks
    pub type Clocks<'a> = stm32f4xx::clocks::Clocks<'a, Stm32f446Specs>;
}

/// STM32F446 Flash module
pub mod flash {
    use crate::chip_specs::Stm32f446Specs;

    /// STM32F446 Flash
    pub type Flash = stm32f4xx::flash::Flash<Stm32f446Specs>;
}
