use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Utf8(std::str::Utf8Error),
    UnexpectedOutput,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Error::Utf8(value)
    }
}

/// This is the state of a device as provided by `simctl`. Currently, only two
/// states are supported ([DeviceState::Shutdown] and [DeviceState::Booted]). All
/// other states are not recognized and mapped to [DeviceState::Unknown] instead.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeviceState {
    Shutdown,
    Booted,
    Unknown,
}

impl FromStr for DeviceState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Shutdown" => DeviceState::Shutdown,
            "Booted" => DeviceState::Booted,
            _ => DeviceState::Unknown,
        })
    }
}

/// This is a device as provided by `simctl --list`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device<'a> {
    operating_system: &'a str,
    name: &'a str,
    identifier: &'a str,
    state: DeviceState,
}

impl<'a> Device<'a> {
    /// This is the operating system of this device, including its version
    /// number. For example: "iOS 13.6" or "watchOS 6.2".
    pub fn operating_system(&self) -> &str {
        self.operating_system
    }

    /// This is the name of this device. For example "iPhone 11 Pro" or "Apple
    /// Watch Series 5 - 44mm".
    pub fn name(&self) -> &str {
        self.name
    }

    /// This is the UUID of this device (including dashes).
    pub fn identifier(&self) -> &str {
        self.identifier
    }

    /// This is the state of this device.
    pub fn state(&self) -> DeviceState {
        self.state
    }
}

#[derive(Debug)]
pub struct Simctl {
    list_output: Option<Output>,
}

impl Simctl {
    pub fn new() -> Simctl {
        Simctl { list_output: None }
    }

    pub fn list(&mut self) -> Result<DeviceManager, Error> {
        let output = Command::new("xcrun")
            .arg("simctl")
            .arg("list")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;

        self.list_output.replace(output);

        Ok(DeviceManager::new(std::str::from_utf8(
            self.list_output
                .as_ref()
                .ok_or(Error::UnexpectedOutput)?
                .stdout
                .as_slice(),
        )?))
    }

    pub fn boot(&mut self, identifier: &str) -> Result<(), Error> {
        Command::new("xcrun")
            .arg("simctl")
            .arg("boot")
            .arg(identifier)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn install(&mut self, identifier: &str, path: impl AsRef<Path>) -> Result<(), Error> {
        Command::new("xcrun")
            .arg("simctl")
            .arg("install")
            .arg(identifier)
            .arg(path.as_ref())
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn launch(&mut self, identifier: &str, bundle_id: &str) -> Result<(), Error> {
        Command::new("xcrun")
            .arg("simctl")
            .arg("launch")
            .arg("--console-tty")
            .arg(identifier)
            .arg(bundle_id)
            .env("SIMCTL_CHILD_RUST_BACKTRACE", "full")
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeviceManager<'a> {
    devices: Vec<Device<'a>>,
}

fn parse_operating_system(line: &str) -> Option<&str> {
    line.strip_prefix("-- ")?.strip_suffix(" --")
}

fn parse_device<'a>(line: &'a str, operating_system: &'a str) -> Option<Device<'a>> {
    let mut line = line.trim_start().trim_end().rsplitn(3, " (");

    let state = DeviceState::from_str(line.next()?.strip_suffix(")")?).ok()?;
    let id = line.next()?.strip_suffix(")")?;

    Some(Device {
        operating_system,
        name: line.next()?,
        identifier: id,
        state,
    })
}

impl<'a> DeviceManager<'a> {
    pub fn new(source: &'a str) -> DeviceManager<'a> {
        let mut lines = source.split("\n").into_iter();

        while let Some(line) = lines.next() {
            if line == "== Devices ==" {
                break;
            }
        }

        let mut devices = vec![];

        let mut operating_system = "";

        while let Some(line) = lines.next() {
            if line.starts_with("--") {
                if let Some(parsed) = parse_operating_system(line) {
                    operating_system = parsed;
                }
            } else if line.starts_with("==") {
                break;
            } else {
                if let Some(device) = parse_device(line, operating_system) {
                    devices.push(device);
                }
            }
        }

        DeviceManager { devices }
    }

    pub fn query(&self) -> DeviceQuery<'_, 'a, std::slice::Iter<Device<'a>>> {
        DeviceQuery {
            iter: self.devices.iter(),
        }
    }
}

pub struct DeviceQuery<'a, 'b, T>
where
    T: Iterator<Item = &'a Device<'b>>,
    'b: 'a,
{
    iter: T,
}

impl<'a, 'b, T> DeviceQuery<'a, 'b, T>
where
    T: Iterator<Item = &'a Device<'b>>,
    'b: 'a,
{
    pub fn with_operating_system(
        self,
        operating_system: &'a str,
    ) -> DeviceQuery<'a, 'b, std::iter::Filter<T, impl FnMut(&&'a Device<'b>) -> bool>> {
        DeviceQuery {
            iter: self
                .iter
                .filter(move |device| device.operating_system == operating_system),
        }
    }

    pub fn with_name(
        self,
        name: &'a str,
    ) -> DeviceQuery<'a, 'b, std::iter::Filter<T, impl FnMut(&&'a Device<'b>) -> bool>> {
        DeviceQuery {
            iter: self.iter.filter(move |device| device.name == name),
        }
    }

    pub fn with_state(
        self,
        state: DeviceState,
    ) -> DeviceQuery<'a, 'b, std::iter::Filter<T, impl FnMut(&&'a Device<'b>) -> bool>> {
        DeviceQuery {
            iter: self.iter.filter(move |device| device.state == state),
        }
    }
}

impl<'a, 'b, T> IntoIterator for DeviceQuery<'a, 'b, T>
where
    T: Iterator<Item = &'a Device<'b>>,
{
    type Item = &'a Device<'b>;
    type IntoIter = T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}
