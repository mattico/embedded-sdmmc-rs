use stm32h7xx_hal::gpio::{self, Alternate, Analog, Input, OpenDrain, Output, PullDown, AF12, Speed};
use stm32h7xx_hal::{pac, rcc, rcc::rec, prelude::*};
use stm32h7xx_hal::sdmmc;
use stm32h7xx_hal::hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub struct SdioTransport {
    sdmmc: sdmmc::Sdmmc<pac::SDMMC1>,
    detect: gpio::gpioc::PC6<Input<PullDown>>,
    led: gpio::gpioa::PA11<Output<OpenDrain>>,
}

impl SdioTransport {
    pub fn new(
        clk: gpio::gpioc::PC12<Analog>,
        cmd: gpio::gpiod::PD2<Analog>,
        d0: gpio::gpioc::PC8<Analog>,
        d1: gpio::gpioc::PC9<Analog>,
        d2: gpio::gpioc::PC10<Analog>,
        d3: gpio::gpioc::PC11<Analog>,
        detect: gpio::gpioc::PC6<Analog>,
        led: gpio::gpioa::PA11<Analog>,
        sdmmc: pac::SDMMC1,
        sdprec: rec::Sdmmc1,
        clocks: &rcc::CoreClocks,
    ) -> Self {
        let clk = clk.into_alternate_af12().set_speed(Speed::VeryHigh);
        let cmd = cmd.into_alternate_af12().set_speed(Speed::VeryHigh);
        let d0 = d0.into_alternate_af12().set_speed(Speed::VeryHigh);
        let d1 = d1.into_alternate_af12().set_speed(Speed::VeryHigh);
        let d2 = d2.into_alternate_af12().set_speed(Speed::VeryHigh);
        let d3 = d3.into_alternate_af12().set_speed(Speed::VeryHigh);

        let detect = detect.into_pull_down_input();
        let led = led.into_open_drain_output();

        let sdmmc = sdmmc.sdmmc((clk, cmd, d0, d1, d2, d3), sdprec, clocks);

        SdioTransport {
            sdmmc,
            detect,
            led,
        }
    }
}

impl super::Transport for SdioTransport {
    fn init(&mut self) -> Result<(), crate::SdMmcError> {
        loop {
            match self.sdmmc.init_card(50.mhz()) {
                Ok(_) => break,
                Err(err) => {
                    //println!("Init err: {:?}", err);
                }
            }

            //delay.delay_ms(1000u32);
            self.led.toggle();
        }

        let size = self.sdmmc.card().unwrap().size();
        //info!("Size: {}", size);
    
        let ocr = self.sdmmc.card().unwrap().ocr;
        //info!("{:?}", ocr);
    
        let scr = self.sdmmc.card().unwrap().scr;
        //info!("{:?}", scr);
    
        let cid = self.sdmmc.card().unwrap().cid;
        //info!("{:?}", cid);
    
        let csd = self.sdmmc.card().unwrap().csd;
        //info!("{:?}", csd);
    
        let status = self.sdmmc.card().unwrap().status;
        //info!("{:?}", status);

        Ok(())
    }
    fn card_command(&self, command: u8, arg: u32) -> Result<u8, crate::SdMmcError> {
        todo!()
    }
    fn receive(&self) -> Result<u8, crate::SdMmcError> {
        todo!()
    }
    fn send(&self, out: u8) -> Result<(), crate::SdMmcError> {
        todo!()
    }
}
