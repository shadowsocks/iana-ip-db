use std::fmt;
use std::str::FromStr;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InvalidStatus;

impl std::error::Error for InvalidStatus { }

impl fmt::Display for InvalidStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidStatus")
    }
}


#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub enum Status {
    Allocated = 0u8,
    Assigned,
    Available,
    Reserved,
}

impl Status {
    pub fn from_index(index: u8) -> Result<Self, ()> {
        match index {
            0 => Ok(Status::Allocated),
            1 => Ok(Status::Assigned),
            2 => Ok(Status::Available),
            3 => Ok(Status::Reserved),
            _ => Err(())
        }
    }

    pub fn index(&self) -> u8 {
        match *self {
            Status::Allocated => 0,
            Status::Assigned => 1,
            Status::Available => 2,
            Status::Reserved => 3,
        }
    }
}

impl FromStr for Status {
    type Err = InvalidStatus;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allocated" => Ok(Status::Allocated),
            "assigned"  => Ok(Status::Assigned),
            "available" => Ok(Status::Available),
            "reserved"  => Ok(Status::Reserved),
                      _ => Err(InvalidStatus),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Allocated => write!(f, "allocated"),
            Status::Assigned => write!(f, "assigned"),
            Status::Available => write!(f, "available"),
            Status::Reserved => write!(f, "reserved"),
        }
    }
}
