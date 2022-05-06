#![doc(hidden)]
#![allow(unused_variables)]
use crate::device_class::*;
use crate::extra::*;
use crate::private::Properties;
use crate::Device;
use crate::Fetch;
use pci_ids::FromId;
use std::path::PathBuf;
/// This is where PCI devices are located.
const PATH_TO_PCI_DEVICES: &str = "/sys/bus/pci/devices/";

#[derive(Debug)]
pub struct LinuxPCIDevice {
    path: PathBuf,
    address: String,
    class_id: Vec<u8>,
    class_name: String,
    subclass_name: String,
    vendor_id: Vec<u8>,
    vendor_name: String,
    device_id: Vec<u8>,
    device_name: String,
    revision: Vec<u8>,
    numa_node: isize,
    enabled: bool,
    d3cold_allowed: bool,
    subsystem_vendor_id: Vec<u8>,
    subsystem_device_id: Vec<u8>,
    subsystem_name: String,
}

impl Device for LinuxPCIDevice {
    fn new(path: &str) -> Self {
        let mut device: LinuxPCIDevice = Default::default();
        let mut path_vec = [path].to_vec();

        // One of the following two conditions will try to autocomplete the path of the
        // PCI device if the one provided doesn't point to a real path in the filesystem.
        if !PathBuf::from(path_vec.concat()).is_dir() {
            // e.g. 0000:00:00.0 ->  /sys/bus/pci/devices/0000:00:00.0
            path_vec.insert(0, PATH_TO_PCI_DEVICES);
            device.set_path(PathBuf::from(path_vec.concat()));
            if !PathBuf::from(path_vec.concat()).is_dir() {
                // e.g. 00:00.0      ->  /sys/bus/pci/devices/0000:00:00.0
                let mut id = path.to_owned();
                id.insert_str(0, "0000:");
                std::mem::swap(&mut path_vec[1], &mut id.as_str());
                device.set_path(PathBuf::from(path_vec.concat()));
            }
        } else {
            device.set_path(PathBuf::from(path_vec.concat()));
        }

        device.set_address();
        device.set_class_id();
        device.set_vendor_id();
        device.set_device_id();
        device.set_numa_node();
        device.set_enabled();
        device.set_d3cold_allowed();
        device.set_revision();
        device.set_subsystem_device_id();
        device.set_subsystem_vendor_id();
        device.set_class_name();
        device.set_device_name();
        device.set_vendor_name();
        device.set_subsystem_name();
        device.set_subclass_name();

        device
    }

    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    fn address(&self) -> String {
        self.address.to_owned()
    }

    fn class_id(&self) -> Vec<u8> {
        self.class_id.to_owned()
    }

    fn vendor_id(&self) -> Vec<u8> {
        self.vendor_id.to_owned()
    }

    fn device_id(&self) -> Vec<u8> {
        self.device_id.to_owned()
    }

    fn numa_node(&self) -> isize {
        self.numa_node
    }

    fn class_name(&self) -> String {
        self.class_name.to_owned()
    }

    fn subclass_name(&self) -> String {
        self.subclass_name.to_owned()
    }

    fn vendor_name(&self) -> String {
        self.vendor_name.to_owned()
    }

    fn device_name(&self) -> String {
        self.device_name.to_owned()
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn d3cold_allowed(&self) -> bool {
        self.d3cold_allowed
    }

    fn revision(&self) -> Vec<u8> {
        self.revision.to_owned()
    }

    fn subsystem_name(&self) -> String {
        self.subsystem_name.to_owned()
    }

    fn subsystem_vendor_id(&self) -> Vec<u8> {
        self.subsystem_vendor_id.to_owned()
    }

    fn subsystem_device_id(&self) -> Vec<u8> {
        self.subsystem_device_id.to_owned()
    }
}

impl Properties for LinuxPCIDevice {
    fn reserved_new(path: &str) -> Self {
        let mut device: LinuxPCIDevice = Default::default();
        let mut path_vec = [path].to_vec();

        // One of the following two conditions will try to autocomplete the path of the
        // PCI device if the one provided doesn't point to a real path in the filesystem.
        if !PathBuf::from(path_vec.concat()).is_dir() {
            // e.g. 0000:00:00.0 ->  /sys/bus/pci/devices/0000:00:00.0
            path_vec.insert(0, PATH_TO_PCI_DEVICES);
            device.set_path(PathBuf::from(path_vec.concat()));
            if !PathBuf::from(path_vec.concat()).is_dir() {
                // e.g. 00:00.0  ->  /sys/bus/pci/devices/0000:00:00.0
                let mut id = path.to_owned();
                id.insert_str(0, "0000:");
                std::mem::swap(&mut path_vec[1], &mut id.as_str());
                device.set_path(PathBuf::from(path_vec.concat()));
            }
        } else {
            device.set_path(PathBuf::from(path_vec.concat()));
        }

        // reserved_new tries to fetch the least amount of data at first.
        // All the other fields can be populated later on.
        device.set_class_id();
        device.set_class_name();

        device
    }

    fn set_path(&mut self, p: PathBuf) {
        self.path = p;
    }

    fn set_address(&mut self) {
        self.address = basename(
            self.path()
                .as_path()
                .display()
                .to_string()
                .replace("0000:", ""),
        );
    }

    fn set_class_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("class")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str[..4]) {
                self.class_id = decoded;
            }
        }
    }

    fn set_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("vendor")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.vendor_id = decoded;
            }
        }
    }

    fn set_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("device")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.device_id = decoded;
            }
        }
    }

    fn set_revision(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("revision")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.revision = decoded;
            }
        }
    }

    fn set_numa_node(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("numa_node")) {
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(v) = prefixless.parse::<isize>() {
                self.numa_node = v;
            }
        }
    }

    fn set_subsystem_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_vendor")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.subsystem_vendor_id = decoded;
            }
        }
    }

    fn set_subsystem_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_device")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.subsystem_device_id = decoded;
            }
        }
    }

    fn set_class_name(&mut self) {
        if self.class_id.is_empty() {
            return;
        }

        if let Some(class) = pci_ids::Class::from_id(self.class_id[0]) {
            self.class_name = class.name().to_owned()
        }
    }

    fn set_subclass_name(&mut self) {
        if self.class_id.is_empty() {
            return;
        }

        if let Some(class) = pci_ids::Class::from_id(self.class_id[0]) {
            class.subclasses().for_each(|subclass| {
                if subclass.id() == self.class_id[1] {
                    self.subclass_name = subclass.name().to_owned()
                }
            });
        }
    }

    fn set_vendor_name(&mut self) {
        if self.vendor_id.is_empty() {
            return;
        }

        let converted_vendor_id = bytes_to_u16(&self.vendor_id);
        if let Some(vendor) = pci_ids::Vendor::from_id(converted_vendor_id) {
            self.vendor_name = vendor.name().to_owned()
        }
    }

    fn set_device_name(&mut self) {
        if self.device_id.is_empty() {
            return;
        }
        let converted_vendor_id = bytes_to_u16(&self.vendor_id);
        let converted_device_id = bytes_to_u16(&self.device_id);

        if let Some(device) =
            pci_ids::Device::from_vid_pid(converted_vendor_id, converted_device_id)
        {
            self.device_name = device.name().to_owned()
        }
    }

    fn set_subsystem_name(&mut self) {
        if self.subsystem_device_id.is_empty() {
            return;
        }

        let converted_vendor_id = bytes_to_u16(&self.vendor_id);
        let converted_device_id = bytes_to_u16(&self.device_id);
        if let Some(device) =
            pci_ids::Device::from_vid_pid(converted_vendor_id, converted_device_id)
        {
            while let Some(subsystem) = device.subsystems().next() {
                let converted_subsystem_vendor_id = bytes_to_u16(&self.subsystem_vendor_id);
                let converted_subsystem_device_id = bytes_to_u16(&self.subsystem_device_id);
                if subsystem.subvendor() == converted_subsystem_vendor_id
                    && subsystem.subdevice() == converted_subsystem_device_id
                {
                    self.subsystem_name = subsystem.name().to_owned();
                    break;
                }
            }
        }
    }

    fn set_enabled(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("enable")) {
            match &str[..] {
                "0\n" => self.enabled = false,
                _ => self.enabled = true,
            }
        }
    }

    fn set_d3cold_allowed(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("d3cold_allowed")) {
            match &str[..] {
                "0\n" => self.d3cold_allowed = false,
                _ => self.d3cold_allowed = true,
            }
        }
    }
}

impl Default for LinuxPCIDevice {
    fn default() -> Self {
        LinuxPCIDevice {
            path: PathBuf::new(),
            address: String::new(),
            class_name: String::new(),
            subclass_name: String::new(),
            vendor_name: String::new(),
            device_name: String::new(),
            subsystem_name: String::new(),
            class_id: vec![],
            subsystem_vendor_id: vec![],
            subsystem_device_id: vec![],
            device_id: vec![],
            revision: vec![],
            vendor_id: vec![],
            numa_node: -1,
            d3cold_allowed: false,
            enabled: false,
        }
    }
}

impl Fetch for LinuxPCIDevice {
    fn fetch(maximum_devices: Option<u8>) -> Vec<LinuxPCIDevice> {
        let mut devices = Vec::new();
        let entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        let mut i = 0u8;
        for dir in entries {
            if let Some(d) = dir.to_str() {
                if let Some(m) = maximum_devices {
                    i = i + 1;
                    if i > m {
                        continue;
                    }
                }

                let device = LinuxPCIDevice::new(d);
                devices.push(device);
            }
        }
        return devices;
    }

    fn fetch_by_class(class: DeviceClass, maximum_devices: Option<u8>) -> Vec<LinuxPCIDevice> {
        let mut devices = Vec::new();
        let dir_entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        let mut i = 0u8;

        for dir in dir_entries {
            if let Some(d) = dir.to_str() {
                if let Some(m) = maximum_devices {
                    i = i + 1;
                    if i > m {
                        continue;
                    }
                }

                // We're using `PCIDevice::reserved_new()` to initialize a PCIDevice
                // with as little data as possible to avoid performance issues.
                let mut device = LinuxPCIDevice::reserved_new(d);
                if device.class_name() == class.to_string() {
                    // We can now proceed to get and set the rest of the data
                    // after having confirmed that the current PCIDevice's class matches
                    // that provided by the user through a variant of the `DeviceClass` enum.
                    device.set_address();
                    device.set_vendor_id();
                    device.set_device_id();
                    device.set_numa_node();
                    device.set_enabled();
                    device.set_d3cold_allowed();
                    device.set_revision();
                    device.set_subsystem_device_id();
                    device.set_subsystem_vendor_id();
                    device.set_device_name();
                    device.set_vendor_name();
                    device.set_subsystem_name();
                    device.set_subclass_name();
                    devices.push(device);
                }
            }
        }

        return devices;
    }

    fn fetch_gpus(maximum_devices: Option<u8>) -> Vec<String> {
        let mut gpus: Vec<String> = vec![];
        let devices: Vec<LinuxPCIDevice> = Vec::new();
        let dir_entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        let mut i = 0u8;

        for dir in dir_entries {
            if let Some(d) = dir.to_str() {
                if let Some(m) = maximum_devices {
                    i = i + 1;
                    if i > m {
                        continue;
                    }
                }

                // We're using `PCIDevice::reserved_new()` to initialize a PCIDevice
                // with as little data as possible to avoid performance issues.
                let mut device = LinuxPCIDevice::reserved_new(d);
                if device.class_name() == DeviceClass::DisplayController.to_string() {
                    // We can now proceed to get and set the rest of the data
                    // after having confirmed that the current PCIDevice's class matches
                    // that provided by the user through a variant of the `DeviceClass` enum.
                    device.set_enabled();
                    // We're only going to return enabled gpus.
                    if device.enabled {
                        device.set_vendor_id();
                        device.set_device_id();
                        device.set_device_name();
                        device.set_vendor_name();

                        let whole_name = device.device_name();
                        // Extracting text within brackets from device_name.
                        if let Some(start_bytes) = whole_name.find("[") {
                            if let Some(end_bytes) = whole_name.rfind("]") {
                                device.device_name =
                                    whole_name[start_bytes + 1..end_bytes].to_owned();
                            }
                        }

                        if device.vendor_name().contains("Corporation") {
                            device.vendor_name = device.vendor_name().replace(" Corporation", "");
                        }

                        let str = String::from(device.vendor_name + " " + &device.device_name);
                        gpus.push(str);
                    }
                }
            }
        }

        return gpus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLACEHOLDER_PCI_DEVICE: &str = "00:00.0";

    #[test]
    fn test_path() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.path(), PathBuf::new());
    }

    #[test]
    fn test_address() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.address(), "");
    }

    #[test]
    fn test_class_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device_id(), Vec::new());
    }

    #[test]
    fn test_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.vendor_id(), Vec::new());
    }

    #[test]
    fn test_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device_id(), Vec::new());
    }

    #[test]
    fn test_numa_node() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.numa_node().to_string(), "");
    }

    #[test]
    fn test_revision() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.revision(), Vec::new());
    }

    #[test]
    fn test_subsystem_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_vendor_id(), Vec::new());
    }

    #[test]
    fn test_subsystem_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_device_id(), Vec::new());
    }

    #[test]
    fn test_class_name() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.class_name(), "");
    }
}
