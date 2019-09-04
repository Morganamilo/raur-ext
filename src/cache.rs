use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

/// A common cache type for users of this library. Libraries that make use of the
/// AUR RPC should take in a cache, make sure to check the cache before making RPC
/// requests. On cache misses, the library should make an RPC request and place the
/// new packages into the cache for others to use.
pub type Cache = HashSet<Package>;

/// A wrapper around raur::Package. Adds the traits neccassery to store in a hash set
/// and look them up by pkgname.
#[derive(Clone, Debug)]
pub struct Package(Arc<raur::Package>);

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
        Package(Arc::new(pkg))
    }
}
