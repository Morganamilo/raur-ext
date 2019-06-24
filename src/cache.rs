use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub type Cache = HashSet<Package>;

#[derive(Debug)]
pub struct Package(raur::Package);

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.name.partial_cmp(&other.0.name)
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.name.cmp(&other.0.name)
    }
}

impl Eq for Package {}

impl Deref for Package {
    type Target = raur::Package;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl Borrow<str> for Package {
    fn borrow(&self) -> &str {
        self.0.name.as_str()
    }
}

impl From<raur::Package> for Package {
    fn from(pkg: raur::Package) -> Package {
        Package(pkg)
    }
}
