//
// This code was generated by a tool.
//
//
//   bebopc version:
//       2.4.5
//
//
//   bebopc source:
//       https://github.com/RainwayApp/bebop
//
//
// Changes to this file may cause incorrect behavior and will be lost if
// the code is regenerated.
//

#![allow(warnings)]

use bebop::FixedSized as _;
use core::convert::TryInto as _;
use std::io::Write as _;

#[derive(Clone, Debug, PartialEq, Copy)]
#[repr(packed)]
pub struct Address {
    pub x0: u32,
    pub x1: u32,
    pub x2: u32,
    pub x3: u32,
}

impl ::bebop::FixedSized for Address {}

impl<'raw> ::bebop::SubRecord<'raw> for Address {
    const MIN_SERIALIZED_SIZE: usize = Self::SERIALIZED_SIZE;
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(Self::SERIALIZED_SIZE);

    #[inline]
    fn serialized_size(&self) -> usize {
        Self::SERIALIZED_SIZE
    }

    #[allow(unaligned_references)]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.x0._serialize_chained(dest)?
            + self.x1._serialize_chained(dest)?
            + self.x2._serialize_chained(dest)?
            + self.x3._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v3) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((
            i,
            Self {
                x0: v0,
                x1: v1,
                x2: v2,
                x3: v3,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for Address {}

#[derive(Clone, Debug, PartialEq)]
pub struct Log<'raw> {
    pub address: Address,
    pub identity: &'raw str,
    pub userid: &'raw str,
    pub date: &'raw str,
    pub request: &'raw str,
    pub code: u32,
    pub size: u64,
}

impl<'raw> ::bebop::SubRecord<'raw> for Log<'raw> {
    const MIN_SERIALIZED_SIZE: usize = <Address>::MIN_SERIALIZED_SIZE
        + <&'raw str>::MIN_SERIALIZED_SIZE
        + <&'raw str>::MIN_SERIALIZED_SIZE
        + <&'raw str>::MIN_SERIALIZED_SIZE
        + <&'raw str>::MIN_SERIALIZED_SIZE
        + <u32>::MIN_SERIALIZED_SIZE
        + <u64>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.address.serialized_size()
            + self.identity.serialized_size()
            + self.userid.serialized_size()
            + self.date.serialized_size()
            + self.request.serialized_size()
            + self.code.serialized_size()
            + self.size.serialized_size()
    }

    #[allow(unaligned_references)]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.address._serialize_chained(dest)?
            + self.identity._serialize_chained(dest)?
            + self.userid._serialize_chained(dest)?
            + self.date._serialize_chained(dest)?
            + self.request._serialize_chained(dest)?
            + self.code._serialize_chained(dest)?
            + self.size._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v3) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v4) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v5) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v6) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((
            i,
            Self {
                address: v0,
                identity: v1,
                userid: v2,
                date: v3,
                request: v4,
                code: v5,
                size: v6,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for Log<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub struct Logs<'raw> {
    pub logs_: ::std::vec::Vec<Log<'raw>>,
}

impl<'raw> ::bebop::SubRecord<'raw> for Logs<'raw> {
    const MIN_SERIALIZED_SIZE: usize = <::std::vec::Vec<Log<'raw>>>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.logs_.serialized_size()
    }

    #[allow(unaligned_references)]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.logs_._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((i, Self { logs_: v0 }))
    }
}

impl<'raw> ::bebop::Record<'raw> for Logs<'raw> {}

#[cfg(feature = "bebop-owned-all")]
pub mod owned {
    #![allow(warnings)]

    use bebop::FixedSized as _;
    use core::convert::TryInto as _;
    use std::io::Write as _;

    pub use super::Address;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Log {
        pub address: Address,
        pub identity: String,
        pub userid: String,
        pub date: String,
        pub request: String,
        pub code: u32,
        pub size: u64,
    }

    impl<'raw> ::core::convert::From<super::Log<'raw>> for Log {
        fn from(value: super::Log) -> Self {
            Self {
                address: value.address,
                identity: value.identity.into(),
                userid: value.userid.into(),
                date: value.date.into(),
                request: value.request.into(),
                code: value.code,
                size: value.size,
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Log {
        const MIN_SERIALIZED_SIZE: usize = <Address>::MIN_SERIALIZED_SIZE
            + <String>::MIN_SERIALIZED_SIZE
            + <String>::MIN_SERIALIZED_SIZE
            + <String>::MIN_SERIALIZED_SIZE
            + <String>::MIN_SERIALIZED_SIZE
            + <u32>::MIN_SERIALIZED_SIZE
            + <u64>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.address.serialized_size()
                + self.identity.serialized_size()
                + self.userid.serialized_size()
                + self.date.serialized_size()
                + self.request.serialized_size()
                + self.code.serialized_size()
                + self.size.serialized_size()
        }

        #[allow(unaligned_references)]
        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.address._serialize_chained(dest)?
                + self.identity._serialize_chained(dest)?
                + self.userid._serialize_chained(dest)?
                + self.date._serialize_chained(dest)?
                + self.request._serialize_chained(dest)?
                + self.code._serialize_chained(dest)?
                + self.size._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v3) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v4) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v5) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v6) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((
                i,
                Self {
                    address: v0,
                    identity: v1,
                    userid: v2,
                    date: v3,
                    request: v4,
                    code: v5,
                    size: v6,
                },
            ))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Log {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct Logs {
        pub logs_: ::std::vec::Vec<Log>,
    }

    impl<'raw> ::core::convert::From<super::Logs<'raw>> for Logs {
        fn from(value: super::Logs) -> Self {
            Self {
                logs_: value.logs_.into_iter().map(|value| value.into()).collect(),
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Logs {
        const MIN_SERIALIZED_SIZE: usize = <::std::vec::Vec<Log>>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.logs_.serialized_size()
        }

        #[allow(unaligned_references)]
        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.logs_._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((i, Self { logs_: v0 }))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Logs {}
}
