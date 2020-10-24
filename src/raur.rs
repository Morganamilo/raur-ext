use crate::Cache;
use raur::{Package, Raur};

/// Extension functions to raur::Package
pub trait RaurExt: Raur {
    /// Perform an info request, storing the results into cache. Requests are not made
    /// for packages already in cache. If all packages are already in cache then this
    /// is effectivley a noop.
    ///
    /// The packages requested will be returned back (even if they were already in cache). Packages
    /// that could not be found will be missing from the return.
    //TODO: async
    fn cache_info<S: AsRef<str>>(
        &self,
        cache: &mut Cache,
        pkgs: &[S],
    ) -> Result<Vec<crate::Package>, Self::Err> {
        let mut ret = Vec::with_capacity(pkgs.len());
        let mut resolve = Vec::with_capacity(pkgs.len());

        for pkg in pkgs {
            if let Some(pkg) = cache.get(pkg.as_ref()) {
                ret.push(pkg.clone());
            } else {
                resolve.push(pkg.as_ref());
            }
        }

        for chunk in resolve.chunks(100) {
            let res = self.info(chunk)?;
            cache.reserve(res.len());
            ret.reserve(res.len());
            for pkg in res.into_iter() {
                let pkg = crate::Package::from(pkg);
                cache.insert(pkg.clone());
                ret.push(pkg);
            }
        }

        Ok(ret)
    }

    /// Perform an info request with package splitting to avoid the URI length limit.
    //TODO: async
    fn info_ext<S: AsRef<str>>(&self, pkgs: &[S]) -> Result<Vec<Package>, Self::Err> {
        let mut packages = Vec::with_capacity(pkgs.len());

        for chunk in pkgs.chunks(100) {
            let res = self.info(chunk)?;
            packages.extend(res.into_iter().map(Into::into));
        }

        Ok(packages)
    }
}

impl<T> RaurExt for T where T: Raur {}
