use crate::Cache;
use raur::{Error, Handle, Package};

pub trait RaurExt {
    fn cache_info<S: AsRef<str>>(&self, cache: &mut Cache, pkgs: &[S]) -> Result<(), Error>;
    fn info<S: AsRef<str>>(&self, pkgs: &[S]) -> Result<Vec<Package>, Error>;
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
    fn info<S: AsRef<str>>(&self, pkgs: &[S]) -> Result<Vec<Package>, Error> {
        let mut packages = Vec::with_capacity(pkgs.len());

        for chunk in pkgs.chunks(100) {
            let res = self.info(chunk)?;
            packages.extend(res.into_iter().map(Into::into));
        }

        Ok(packages)
    }
}
