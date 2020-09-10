use super::virtual_machine_errors::VMErrKind;
use std::cmp;
use std::collections::HashMap;

//TODO: Special behavior
/// A peripheral can be written to or read from.
pub trait BFPeripheral {
    /// Send a value to the peripheral.
    fn write(&mut self, value: u16) -> Result<(), VMErrKind>;
    /// Read a value from the peripheral.
    fn read(&mut self) -> Result<u16, VMErrKind>;
}

/// A function that changes the internal state of the machine.
type Write = dyn FnMut(u16) -> Result<(), VMErrKind>;
/// A function that reads from and updates the internal state of the machine.
type Read = dyn FnMut() -> Result<u16, VMErrKind>;

/// A map from peripheral tape addresses to native write functions.
type NativeWriteMap<'a> = HashMap<u16, &'a mut Write>;
/// A map from peripheral tape addresses to native read functions.
type NativeReadMap<'a> = HashMap<u16, &'a mut Read>;

/// A tape with peripherals on it.
/// Real peripherals are continuously indexed from 0, and processor native operations are stored
/// in maps with the tape address as a key.
pub struct PeripheralTape<'a> {
    peripherals: &'a mut Vec<&'a dyn BFPeripheral>,
    native_write: &'a mut NativeWriteMap<'a>,
    native_read: &'a mut NativeReadMap<'a>,
}

impl<'a> PeripheralTape<'a> {
    /// Creates a new peripheral tape with the specified peripherals.
    /// # Errors
    /// If the processor native peripherals overlap with the addresses of real peripherals, then a
    /// `VMErrKind::OverlappingPeripheralAddresses` error is returned.
    pub fn new(
        peripherals: &'a mut Vec<&'a dyn BFPeripheral>,
        native_write: &'a mut NativeWriteMap<'a>,
        native_read: &'a mut NativeReadMap<'a>,
    ) -> Result<PeripheralTape<'a>, VMErrKind> {
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

    /// Writes a value to the peripheral at the specified address.
    /// # Errors
    /// If there is no peripheral at the specified address, then a
    /// `VMErrKind::InvalidPeripheralTapeAccess` error is returned.
    /// Other errors are propogated from the peripherals.
    pub fn write(&mut self, value: u16, address: u16) -> Result<(), VMErrKind> {
        if let Some(fun) = self.native_write.get_mut(&address) {
            return fun(value);
        }

        if let Some(periph) = self.peripherals.get_mut(usize::from(address)) {
            return periph.write(value);
        }

        Err(VMErrKind::InvalidPeripheralTapeAccess(address))
    }

    /// Reads a value from the peripheral at the specified address.
    /// # Errors
    /// If there is no peripheral at the specified address, then a
    /// `VMErrKind::InvalidPeripheralTapeAccess` error is returned.
    /// Other errors are propogated from the peripherals.
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
