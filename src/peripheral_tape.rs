use super::virtual_machine_errors::VMErrKind;
use std::cmp;
use std::collections::HashMap;

pub trait BFPeripheral {
    fn write(&mut self, value: u16) -> Result<(), VMErrKind>;
    fn read(&mut self) -> Result<u16, VMErrKind>;
}

type Write = dyn FnMut(u16) -> Result<(), VMErrKind>;
type Read = dyn FnMut() -> Result<u16, VMErrKind>;

type NativeWriteMap<'a> = HashMap<u16, &'a mut Write>;
type NativeReadMap<'a> = HashMap<u16, &'a mut Read>;

pub struct PeripheralTape<'a, T: BFPeripheral> {
    peripherals: &'a mut Vec<T>,
    native_write: &'a mut NativeWriteMap<'a>,
    native_read: &'a mut NativeReadMap<'a>,
}

impl<'a, T: BFPeripheral> PeripheralTape<'a, T> {
    pub fn new(
        peripherals: &'a mut Vec<T>,
        native_write: &'a mut NativeWriteMap<'a>,
        native_read: &'a mut NativeReadMap<'a>,
    ) -> Result<PeripheralTape<'a, T>, VMErrKind> {
        let least = native_write
            .keys()
            .chain(native_read.keys())
            .map(|a| u32::from(*a))
            .fold(0x10000u32, cmp::min);

        if peripherals.len() as u64 > least as u64 {
            return Err(VMErrKind::OverlappingPeripheralAddresses(
                peripherals.len(),
                least,
            ));
        }

        Ok(PeripheralTape {
            peripherals,
            native_write,
            native_read,
        })
    }

    pub fn write(&mut self, value: u16, address: u16) -> Result<(), VMErrKind> {
        if let Some(fun) = self.native_write.get_mut(&address) {
            return fun(value);
        }

        if let Some(periph) = self.peripherals.get_mut(usize::from(address)) {
            return periph.write(value);
        }

        Ok(())
    }

    pub fn read(&mut self, address: u16) -> Result<u16, VMErrKind> {
        if let Some(fun) = self.native_read.get_mut(&address) {
            return fun();
        }

        if let Some(periph) = self.peripherals.get_mut(usize::from(address)) {
            return periph.read();
        }

        Err(VMErrKind::InvalidPeripheralTapeAccess(address))
    }
}










