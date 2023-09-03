use std::{marker::PhantomData, ops::Deref, sync::Arc};

use crate::nifpga::NiFpga;

pub trait StorageClone<'a> {
    type Target;

    fn storage_clone(&'a self) -> Self::Target;
}

#[derive(Clone)]
pub struct ArcStorage {
    pub(crate) api: Arc<NiFpga>,
}

impl Deref for ArcStorage {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl StorageClone<'_> for ArcStorage {
    type Target = ArcStorage;

    fn storage_clone(&self) -> ArcStorage {
        self.clone()
    }
}

pub struct InPlaceStorage<'a> {
    pub(crate) api: NiFpga,
    pub(crate) _marker: PhantomData<&'a NiFpga>,
}

impl<'a> Deref for InPlaceStorage<'a> {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl<'a> StorageClone<'a> for InPlaceStorage<'a> {
    type Target = RefStorage<'a>;

    fn storage_clone(&'a self) -> RefStorage<'a> {
        RefStorage { api: &self.api }
    }
}

impl<'a> From<&'a InPlaceStorage<'_>> for RefStorage<'a> {
    fn from(value: &'a InPlaceStorage) -> Self {
        Self { api: &value.api }
    }
}

pub struct RefStorage<'a> {
    pub(crate) api: &'a NiFpga,
}

impl<'a> Deref for RefStorage<'a> {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        self.api
    }
}

impl<'a> StorageClone<'a> for RefStorage<'a> {
    type Target = RefStorage<'a>;

    fn storage_clone(&self) -> RefStorage<'a> {
        RefStorage { api: self.api }
    }
}
