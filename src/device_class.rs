//! This module contains a crucial enum, [DeviceClass], that you can use when working with [Fetch](crate::Fetch).

use num_traits::ToPrimitive;
use pci_ids::FromId;
use std::fmt;
/// This enum holds variants that are defined as classes in <https://pci-ids.ucw.cz/read/PD/>.

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum DeviceClass {
    Unclassified,                      // ID: 00
    MassStorageController,             // ID: 01
    NetworkController,                 // ID: 02
    DisplayController,                 // ID: 03
    MultimediaController,              // ID: 04
    MemoryController,                  // ID: 05
    Bridge,                            // ID: 06
    CommunicationController,           // ID: 07
    GenericSystemPeripheral,           // ID: 08
    InputDeviceController,             // ID: 09
    DockingStation,                    // ID: 0a
    Processor,                         // ID: 0b
    SerialBusController,               // ID: 0c
    WirelessController,                // ID: 0d
    IntelligentController,             // ID: 0e
    SatelliteCommunicationsController, // ID: 0f
    EncryptionController,              // ID: 10
    SignalProcessingController,        // ID: 11
    ProcessingAccelerator,             // ID: 12
    NonEssentialInstrumentation,       // ID: 13
    Coprocessor,                       // ID: 40
    Unassigned,                        // ID: ff
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            pci_ids::Class::from_id(ToPrimitive::to_u8(self).unwrap_or(0))
                .ok_or(fmt::Error)?
                .name()
        )
    }
}
