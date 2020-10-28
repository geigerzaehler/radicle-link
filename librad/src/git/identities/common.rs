// This file is part of radicle-link
// <https://github.com/radicle-dev/radicle-link>
//
// Copyright (C) 2019-2020 The Radicle Team <dev@radicle.xyz>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3 or
// later as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    git::{
        storage2::Storage,
        types::{namespace::Namespace, Force, Reference},
    },
    identities::git::Urn,
    peer::PeerId,
    signer::Signer,
};

pub struct IdRef<'a>(&'a Urn);

impl<'a> From<&'a Urn> for IdRef<'a> {
    fn from(urn: &'a Urn) -> Self {
        Self(urn)
    }
}

impl<'a> IdRef<'a> {
    pub fn create<S>(
        &self,
        storage: &Storage<S>,
        target: impl AsRef<git2::Oid>,
    ) -> Result<(), git2::Error>
    where
        S: Signer,
    {
        Reference::<_, PeerId, _>::rad_id(Namespace::from(self.0))
            .create(
                storage.as_raw(),
                *target.as_ref(),
                Force::False,
                "initialise",
            )
            .and(Ok(()))
    }

    pub fn update<S: Signer>(
        &self,
        storage: &Storage<S>,
        target: impl AsRef<git2::Oid>,
        msg: &str,
    ) -> Result<(), git2::Error>
    where
        S: Signer,
    {
        Reference::<_, PeerId, _>::rad_id(Namespace::from(self.0))
            .create(storage.as_raw(), *target.as_ref(), Force::True, msg)
            .and(Ok(()))
    }
}
