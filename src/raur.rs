use crate::Cache;
use raur::{Error, Handle, Package, Raur};

/// Extension functions to raur::Handle
pub trait RaurExt {
    /// Perform an info request, storing the results into cache. Requests are not made
    /// for packages already in cache. If all packages are already in cache then this
    /// is effectivley a noop.
    fn cache_info<S: AsRef<str>>(&self, cache: &mut Cache, pkgs: &[S]) -> Result<(), Error>;
    /// Perform an info request with package splitting to avoid the URI length limit.
    fn info_ext<S: AsRef<str>>(&self, pkgs: &[S]) -> Result<Vec<Package>, Error>;
}

impl RaurExt for Handle {
    //TODO: async
    fn cache_info<S: AsRef<str>>(&self, cache: &mut Cache, pkgs: &[S]) -> Result<(), Error> {
        let pkgs = pkgs
            .iter()
            .filter(|p| !cache.contains(p.as_ref()))
            .collect::<Vec<_>>();

        for chunk in pkgs.chunks(100) {
            let res = self.info(chunk)?;
            cache.extend(res.into_iter().map(Into::into));
        }

        Ok(())
    }

    //TODO: async
    fn info_ext<S: AsRef<str>>(&self, pkgs: &[S]) -> Result<Vec<Package>, Error> {
        let mut packages = Vec::with_capacity(pkgs.len());

        for chunk in pkgs.chunks(100) {
            let res = self.info(chunk)?;
            packages.extend(res.into_iter().map(Into::into));
        }

        Ok(packages)
    }
}
