use std::num::Wrapping;

use crate::machine::Machine;

use super::interrupts::TIMER_INTERRUPT_BIT;

const DIVIDE_REGISTER_ADDRESS: u16 = 0xFF04;
const TIMER_COUNTER_ADDRESS: u16 = 0xFF05;
const TIMER_MODULO_ADDRESS: u16 = 0xFF06;
const TIMER_CONTROL_ADDRESS: u16 = 0xFF07;

#[derive(Clone, Debug, Hash)]
pub struct Timers {
    pub divide_register: Wrapping<u8>,
    divide_register_dots: u16,
    // When we reset this, we must account for the fact that the reset would happen at the end of
    // the resetting instruction, rather than the beginning.  So we mark this to know to reset it
    // later.
    divide_register_to_be_reset: bool,
    pub timer_counter: Wrapping<u8>,
    timer_counter_dots: u16,
    pub timer_modulo: Wrapping<u8>,
    pub timer_control: Wrapping<u8>,
}

fn get_timer_counter_threshold(machine: &mut Machine) -> u16 {
    match machine.cpu.timers.timer_control.0 & 0x3 {
        0b00 => 1024,
        0b01 => 16,
        0b10 => 64,
        0b11 => 256,
        _ => unreachable!(),
    }
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            divide_register: Wrapping(0),
            divide_register_to_be_reset: false,
            divide_register_dots: 0,
            timer_counter: Wrapping(0),
            timer_counter_dots: 0,
            timer_modulo: Wrapping(0),
            timer_control: Wrapping(0),
        }
    }

    fn step_one_dot(machine: &mut Machine) {
        machine.cpu.timers.divide_register_dots += 1;
        if machine.cpu.timers.divide_register_dots == 256 {
            machine.cpu.timers.divide_register_dots = 0;
            machine.cpu.timers.divide_register += 1;
        }

        if (machine.cpu.timers.timer_control.0 & 0b100) != 0 {
            machine.cpu.timers.timer_counter_dots += 1;
            if machine.cpu.timers.timer_counter_dots == get_timer_counter_threshold(machine) {
                machine.cpu.timers.timer_counter_dots = 0;
                machine.cpu.timers.timer_counter += 1;
                if machine.cpu.timers.timer_counter.0 == 0 {
                    machine.cpu.timers.timer_counter = machine.cpu.timers.timer_modulo;
                    machine.request_interrupt(TIMER_INTERRUPT_BIT);
                }
            }
        }
    }

    pub fn step_dots(machine: &mut Machine, dots: u8) {
        for _ in 0..dots {
            Self::step_one_dot(machine);
        }
        if machine.cpu.timers.divide_register_to_be_reset {
            machine.cpu.timers.divide_register_to_be_reset = false;
            machine.cpu.timers.divide_register = Wrapping(0);
        }
    }

    pub fn read_u8(&self, address: Wrapping<u16>) -> Wrapping<u8> {
        match address.0 {
            DIVIDE_REGISTER_ADDRESS => self.divide_register,
            TIMER_COUNTER_ADDRESS => self.timer_counter,
            TIMER_MODULO_ADDRESS => self.timer_modulo,
            TIMER_CONTROL_ADDRESS => self.timer_control,
            _ => unreachable!(),
        }
    }

    pub fn write_u8(&mut self, address: Wrapping<u16>, value: Wrapping<u8>) {
        match address.0 {
            DIVIDE_REGISTER_ADDRESS => {
                // Writing any value to this register resets it.  However, if we were to reset it
                // here for a 4 t-cycle instruction, it would have started counting 4 by the time
                // where it should actually be reset.  So instead we mark it to be reset after
                // simulating the current instruction's t-cycles.
                self.divide_register_to_be_reset = true;
            }
            TIMER_COUNTER_ADDRESS => self.timer_counter = value,
            TIMER_MODULO_ADDRESS => self.timer_modulo = value,
            TIMER_CONTROL_ADDRESS => self.timer_control = value,
            _ => unreachable!(),
        }
    }
}