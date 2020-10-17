//! Rust-wrapper around the `simctl` utility that is shipped with Xcode and that
//! can be used to install apps onto one of the iOS simulator and subsequently
//! launch them.

use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::str::FromStr;

/// Represents an error that occurs while communicating with the `simctl`
/// utility.
#[derive(Debug)]
pub enum Error {
    /// Contains an error that occurred while invoking the `simctl` utility.
    IO(std::io::Error),

    /// Contains an error that occurred while parsing the output of `simctl` as
    /// utf-8.
    Utf8(std::str::Utf8Error),

    /// Contains an error that occurred while parsing the output and
    /// encountering an unexpected sequence of tokens.
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
    /// Represents a simulator that is not currently running.
    Shutdown,

    /// Represents a simulator that is currently running. Note that simulators
    /// may even be running while the Simulator.app is closed or the window
    /// corresponding to a specific simulator is closed.
    Booted,

    /// Represents any status that could not be parsed.
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

/// Wrapper around the `simctl` utility.
#[derive(Debug)]
pub struct Simctl {
    list_output: Option<Output>,
}

impl Simctl {
    /// Returns a new instance of the Rust wrapper around the `simctl` utility.
    pub fn new() -> Simctl {
        Simctl { list_output: None }
    }

    /// Lists all simulator devices that are configured.
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

    /// Boots a simulator with the given identifier.
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

    /// Installs an application from the given path to the simulator with the
    /// given identifier.
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

    /// Launches an installed application with the given bundle identifier on
    /// the simulator with the given identifier.
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

/// Wrapper around a set of devices. This is used to parse the output of a
/// `simctl list` command.
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
    /// Parses a set of devices from the output of `simctl list` and returns the
    /// result.
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

    /// Starts a new query for this set of devices.
    pub fn query(&self) -> DeviceQuery<'_, 'a, std::slice::Iter<Device<'a>>> {
        DeviceQuery {
            iter: self.devices.iter(),
        }
    }
}

/// Represents a query of devices.
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
    /// Queries for simulators that run on the given operating system.
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

    /// Queries for simulators that have the given name.
    pub fn with_name(
        self,
        name: &'a str,
    ) -> DeviceQuery<'a, 'b, std::iter::Filter<T, impl FnMut(&&'a Device<'b>) -> bool>> {
        DeviceQuery {
            iter: self.iter.filter(move |device| device.name == name),
        }
    }

    /// Queries for simulators that are in the given state.
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
