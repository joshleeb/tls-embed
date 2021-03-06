use crate::{
    codec::{
        array::{iter::ArrayIter, Array},
        Codec, CodecSized, Decoder, Encoder, HeaderSize,
    },
    error::Result as TlsResult,
    handshake::enums::{ProtocolVersion, SignatureScheme},
};
use client::ClientExtension;
use server::ServerExtension;

pub mod client;
pub mod server;

#[macro_use]
mod macros;

#[derive(Debug, PartialEq)]
pub struct Extensions<'a, T: CodecSized<'a>>(Array<'a, T>);

impl<'a, T: CodecSized<'a>> Extensions<'a, T> {
    pub fn empty() -> Self {
        Self(Array::empty())
    }

    pub fn encode_extensions(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        self.0.encode(enc)
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a, T: CodecSized<'a>> Codec<'a> for Extensions<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }
        self.encode_extensions(enc)
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Array::<'a, T>::decode(dec).map(Self)
    }
}

impl<'a, T: CodecSized<'a>> CodecSized<'a> for Extensions<'a, T> {
    const HEADER_SIZE: HeaderSize = HeaderSize::Zero;

    fn data_size(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }
        T::HEADER_SIZE.size() + self.0.data_size()
    }
}

impl<'a, T: CodecSized<'a>> Default for Extensions<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> From<T> for Extensions<'a, ClientExtension<'a>>
where
    T: Into<Array<'a, ClientExtension<'a>>>,
{
    fn from(data: T) -> Self {
        Self(data.into())
    }
}

impl<'a, T> From<T> for Extensions<'a, ServerExtension>
where
    T: Into<Array<'a, ServerExtension>>,
{
    fn from(data: T) -> Self {
        Self(data.into())
    }
}

// TODO: Make `ext_array` macro more expressive/explicit
ext_array!(SignatureSchemes, HeaderSize::U16, SignatureScheme);
ext_array!(ProtocolVersions, HeaderSize::U8, ProtocolVersion);

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn empty_signature_schemes() {
            let schemes = SignatureSchemes::empty();
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc).unwrap();

            assert_eq!(schemes.data_size(), 0);
            assert_eq!(enc.bytes(), [0, 0]);
        }

        #[test]
        fn single_signature_scheme() {
            let schemes = SignatureSchemes::from(arr![SignatureScheme::RsaPkcs1Sha256]);
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc).unwrap();

            assert_eq!(schemes.data_size(), 2);
            assert_eq!(enc.bytes(), [0, 2, 0x04, 0x01]);
        }

        #[test]
        fn multiple_signature_schemes() {
            let schemes = SignatureSchemes::from(arr![
                SignatureScheme::RsaPkcs1Sha256,
                SignatureScheme::RsaPkcs1Sha384,
            ]);
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc).unwrap();

            assert_eq!(schemes.data_size(), 4);
            assert_eq!(enc.bytes(), [0, 4, 0x04, 0x01, 0x05, 0x01]);
        }

        #[test]
        fn empty_protocol_versions() {
            let versions = ProtocolVersions::empty();
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc).unwrap();

            assert_eq!(versions.data_size(), 0);
            assert_eq!(enc.bytes(), [0]);
        }

        #[test]
        fn single_protocol_version() {
            let versions = ProtocolVersions::from(arr![ProtocolVersion::TLSv1_3]);
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc).unwrap();

            assert_eq!(versions.data_size(), 2);
            assert_eq!(enc.bytes(), [2, 0x03, 0x04]);
        }

        #[test]
        fn multiple_protocol_versions() {
            let versions =
                ProtocolVersions::from(arr![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2]);
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc).unwrap();

            assert_eq!(versions.data_size(), 4);
            assert_eq!(enc.bytes(), [4, 0x03, 0x04, 0x03, 0x03]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn zero_length_signature_schemes() {
            let bytes = [0, 0];
            let mut dec = Decoder::new(&bytes);

            let schemes = SignatureSchemes::decode(&mut dec).unwrap();
            assert!(schemes.is_empty())
        }

        #[test]
        fn single_signature_scheme() {
            let bytes = [0, 2, 0x04, 0x01];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                SignatureSchemes::decode(&mut dec).unwrap(),
                arr![SignatureScheme::RsaPkcs1Sha256].into(),
            );
        }

        #[test]
        fn multiple_signature_schemes() {
            let bytes = [0, 4, 0x04, 0x01, 0x05, 0x01];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                SignatureSchemes::decode(&mut dec).unwrap(),
                arr![
                    SignatureScheme::RsaPkcs1Sha256,
                    SignatureScheme::RsaPkcs1Sha384,
                ]
                .into(),
            );
        }

        #[test]
        fn zero_length_protocol_versions() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);

            let versions = ProtocolVersions::decode(&mut dec).unwrap();
            assert!(versions.is_empty());
        }

        #[test]
        fn single_protocol_version() {
            let bytes = [2, 0x03, 0x04];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ProtocolVersions::decode(&mut dec).unwrap(),
                arr![ProtocolVersion::TLSv1_3].into(),
            );
        }

        #[test]
        fn multiple_protocol_versions() {
            let bytes = [4, 0x03, 0x04, 0x03, 0x03];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ProtocolVersions::decode(&mut dec).unwrap(),
                arr![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2].into(),
            );
        }
    }
}
