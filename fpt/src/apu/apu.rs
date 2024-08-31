use crate::memory::{Bus,map};
use crate::bw;

use blip_buf::BlipBuf;

struct SquareChannel {
    active: bool,
    duty_cycle: u8,
    period: u32,
    blip_buf: BlipBuf,
}

impl SquareChannel {
    pub fn new(sample_rate: u32) -> SquareChannel {
        SquareChannel {
            active: false,
            duty_cycle: 1,
            period: 2048,
            blip_buf: BlipBuf::new(sample_rate),
        }
    }
}

pub struct Apu{
    bus: Bus,  
}

impl Apu {
    pub fn new(bus: Bus) -> Apu {
        Apu {
            bus
        }
    }

    pub fn step(&mut self, cycles: u32) {
        let nr52 = self.bus.read(map::NR52);

        //let audio_on = dbg!(bw::telst_bit8::<7>(nr52));
        //let ch4_on = dbg!(bw::test_bit8::<3>(nr52));
        //let ch3_on = dbg!(bw::test_bit8::<2>(nr52));
        //let ch2_on = dbg!(bw::test_bit8::<1>(nr52));
        //let ch1_on = dbg!(bw::test_bit8::<0>(nr52));
    }
}
