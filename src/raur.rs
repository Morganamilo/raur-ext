use crate::Cache;
use raur::{Package, Raur};

/// Extension functions to raur::Package
pub trait RaurExt: Raur {
    /// Perform an info request, storing the results into cache. Requests are not made
    /// for packages already in cache. If all packages are already in cache then this
    /// is effectivley a noop.
    //TODO: async
    fn cache_info<S: AsRef<str>>(
        &self,
        cache: &mut Cache,
        pkgs: &[S],
    ) -> Result<Vec<crate::Package>, Self::Err> {
        let mut ret = Vec::with_capacity(pkgs.len());
        let pkgs = pkgs
            .iter()
            .filter(|p| !cache.contains(p.as_ref()))
            .collect::<Vec<_>>();

        for chunk in pkgs.chunks(200) {
            let res = self.info(chunk)?;
            cache.reserve(chunk.len());
            ret.reserve(chunk.len());
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
