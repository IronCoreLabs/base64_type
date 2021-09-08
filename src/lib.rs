use base64::{STANDARD, URL_SAFE};
use base64_serde::base64_serde_type;
use bytes::Bytes;
use core::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
    str::FromStr,
};
#[cfg(test)]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// use official base64_serde crate to generate a type with correct serde implementations.
base64_serde_type!(Base64StandardSerde, STANDARD);
/// Base64 newtype wrapper using `STANDARD` encoding. May be generally treated as if it
/// were a primitive Vec, e.g. `&Base64` will provide `&[u8]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Base64(pub Vec<u8>);
impl Deref for Base64 {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}
impl DerefMut for Base64 {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}
impl Serialize for Base64 {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        Base64StandardSerde::serialize(&self.0, serializer)
    }
}
impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D>(deserializer: D) -> Result<Base64, D::Error>
    where
        D: Deserializer<'de>,
    {
        Base64StandardSerde::deserialize(deserializer).map(Base64)
    }
}
impl FromStr for Base64 {
    type Err = base64::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base64::decode_config(s, STANDARD).map(Base64)
    }
}
impl From<&[u8]> for Base64 {
    fn from(value: &[u8]) -> Self {
        Base64(value.to_vec())
    }
}
impl From<&Bytes> for Base64 {
    fn from(value: &bytes::Bytes) -> Self {
        Base64(value.to_vec())
    }
}
impl From<Bytes> for Base64 {
    fn from(value: bytes::Bytes) -> Self {
        Base64(value.to_vec())
    }
}
impl From<UrlBase64> for Base64 {
    fn from(value: UrlBase64) -> Self {
        Base64(value.0)
    }
}
impl ToString for Base64 {
    fn to_string(&self) -> String {
        base64::encode_config(&self.0, STANDARD)
    }
}
impl From<Base64> for Bytes {
    fn from(b64: Base64) -> Bytes {
        Bytes::from(b64.0)
    }
}
impl From<&Base64> for Bytes {
    fn from(b64: &Base64) -> Bytes {
        Bytes::copy_from_slice(b64)
    }
}
impl TryFrom<Base64> for [u8; 32] {
    type Error = String;
    fn try_from(b64: Base64) -> Result<Self, Self::Error> {
        if b64.len() == 32 {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&b64);
            Ok(arr)
        } else {
            Err("Base64 was not 32 bytes of data.".to_string())
        }
    }
}

// use official base64_serde crate to generate a type with correct serde implementations.
base64_serde_type!(UrlBase64Serde, URL_SAFE);
/// Base64 newtype wrapper using `URL_SAFE` encoding. Used for Azure requests and responses.
#[derive(Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UrlBase64(pub Vec<u8>);
impl Serialize for UrlBase64 {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        UrlBase64Serde::serialize(&self.0, serializer)
    }
}
impl<'de> Deserialize<'de> for UrlBase64 {
    fn deserialize<D>(deserializer: D) -> Result<UrlBase64, D::Error>
    where
        D: Deserializer<'de>,
    {
        UrlBase64Serde::deserialize(deserializer).map(UrlBase64)
    }
}
impl From<&[u8]> for UrlBase64 {
    fn from(value: &[u8]) -> Self {
        UrlBase64(value.to_vec())
    }
}
impl From<&Bytes> for UrlBase64 {
    fn from(value: &bytes::Bytes) -> Self {
        UrlBase64(value.to_vec())
    }
}
impl From<UrlBase64> for Bytes {
    fn from(b64: UrlBase64) -> Bytes {
        Bytes::from(b64.0)
    }
}
impl From<Base64> for UrlBase64 {
    fn from(value: Base64) -> Self {
        UrlBase64(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{array, collection, prelude::*};

    mod url_base64 {
        use super::*;

        #[test]
        fn deserialize_known() {
            // Azure sample
            let base64_str = "LYo7WN8-DSYHqZa9PxIVyiJpMDWyj6P4irM1QUFM3fI_pRfgbXSCNP_CWt0x49GgIFRQaN0iShf3IlxMDsLRLsKM2c5fdABpVi6L56Rfu4Vn9htGS6lXfm1Ylvds6ywcI9E6brLIMSHoJYCi8o0pH4bH_vWWD-8TEBfBm-lEtT2k2fyznMpvBEznQrixNifNS3obWmZv5VBcUBzbYJ-2QHfrOiufe9Xj8VisjNvOzsEMPOETEVFnMEY-OBY4fV1JifFtt-dR6Cst3JuHq3yeRiLVX_EQmyZZZrzCJOglcOxt85qXM5mlOnrz3M2vRQju1BYb-Cgmdho9Dg8gmKTdeQ";
            let b64 = base64::decode_config(base64_str, URL_SAFE).expect("decode worked");
            let de: UrlBase64 =
                serde_json::from_str(&format!("\"{}\"", base64_str)).expect("deserialize worked");
            assert_eq!(de.0, b64);
        }

        #[test]
        fn deserialize_fails_stdbase64() {
            let base64_str = "Cr4BCrgBAQIDAHj0ZREHq1bONJuR5ImNOlC8TTbXrFSZ5ETcue/j52IG8AEQ9A1ynzkO7801Yub6KEL/AAAAfjB8BgkqhkiG9w0BBwagbzBtAgEAMGgGCSqGSIb3DQEHATAeBglghkgBZQMEAS4wEQQMppr+vkkDynbG8fbNAgEQgDsUjU+Hr6ietkfUzpkFwf+yF5BRS7+7RHRPlQvOHiyY8Ca91GOr+QP/4qTKnJ8w5PTV6Rx5r59aByPVqxCPAw==";
            let received =
                serde_json::from_str::<UrlBase64>(&format!("\"{}\"", base64_str)).unwrap_err();
            assert!(received.is_data());
        }

        #[test]
        fn serialize_empty() {
            let se = serde_json::to_string(&UrlBase64::default()).unwrap();
            assert_eq!(se, r#""""#);
        }

        proptest! {
            #[test]
            fn serde_roundtrip(b64 in any::<UrlBase64>()) {
                let ser = serde_json::to_string(&b64).unwrap();
                let de = serde_json::from_str(&ser).unwrap();
                assert_eq!(b64, de);
            }

            #[test]
            fn from_byte_slice(arr in array::uniform3(any::<u8>())) {
                let slice = &arr[..];
                let expected = UrlBase64(arr.to_vec());
                let received = UrlBase64::from(slice);
                assert_eq!(expected, received);
            }

            #[test]
            fn from_bytes_borrow(vec in collection::vec(any::<u8>(), 0..20)) {
                let bytes = bytes::Bytes::from(vec.clone());
                let expected = UrlBase64(vec);
                let received = UrlBase64::from(&bytes);
                assert_eq!(expected, received);
            }

            #[test]
            fn to_bytes(vec in collection::vec(any::<u8>(), 0..20)) {
                let expected = bytes::Bytes::from(vec.clone());
                let b64 = UrlBase64(vec);
                let received: bytes::Bytes = b64.into();
                assert_eq!(expected, received);
            }
        }
    }

    mod std_base64 {
        use super::*;
        use core::convert::TryInto;

        #[test]
        fn deserialize_known() {
            let base64_str = "Cr4BCrgBAQIDAHj0ZREHq1bONJuR5ImNOlC8TTbXrFSZ5ETcue/j52IG8AEQ9A1ynzkO7801Yub6KEL/AAAAfjB8BgkqhkiG9w0BBwagbzBtAgEAMGgGCSqGSIb3DQEHATAeBglghkgBZQMEAS4wEQQMppr+vkkDynbG8fbNAgEQgDsUjU+Hr6ietkfUzpkFwf+yF5BRS7+7RHRPlQvOHiyY8Ca91GOr+QP/4qTKnJ8w5PTV6Rx5r59aByPVqxCPAw==";
            let b64 = base64::decode_config(base64_str, STANDARD).expect("decode worked");
            let de: Base64 =
                serde_json::from_str(&format!("\"{}\"", base64_str)).expect("deserialize worked");
            assert_eq!(de.0, b64);
        }

        #[test]
        fn deserialize_fails_urlbase64() {
            let base64_str = "LYo7WN8-DSYHqZa9PxIVyiJpMDWyj6P4irM1QUFM3fI_pRfgbXSCNP_CWt0x49GgIFRQaN0iShf3IlxMDsLRLsKM2c5fdABpVi6L56Rfu4Vn9htGS6lXfm1Ylvds6ywcI9E6brLIMSHoJYCi8o0pH4bH_vWWD-8TEBfBm-lEtT2k2fyznMpvBEznQrixNifNS3obWmZv5VBcUBzbYJ-2QHfrOiufe9Xj8VisjNvOzsEMPOETEVFnMEY-OBY4fV1JifFtt-dR6Cst3JuHq3yeRiLVX_EQmyZZZrzCJOglcOxt85qXM5mlOnrz3M2vRQju1BYb-Cgmdho9Dg8gmKTdeQ";
            let received =
                serde_json::from_str::<Base64>(&format!("\"{}\"", base64_str)).unwrap_err();
            assert!(received.is_data());
        }

        #[test]
        fn serialize_empty() {
            let se = serde_json::to_string(&Base64(Vec::new())).unwrap();
            assert_eq!(se, r#""""#);
        }

        #[test]
        fn serialize_known() {
            let b64 = Base64(vec![2, 99]);
            let expected = r#""AmM=""#;
            let received = serde_json::to_string(&b64).unwrap();
            assert_eq!(expected, received);
        }

        #[test]
        fn from_str_known() {
            let expected = Base64(vec![2, 99]);
            let test_str = "AmM=";
            let received = test_str.parse().unwrap();
            assert_eq!(expected, received);
        }

        #[test]
        fn to_str_known() {
            let b64 = Base64(vec![2, 99]);
            let expected = "AmM=";
            let received = b64.to_string();
            assert_eq!(expected, received);
        }

        #[test]
        fn short_byte_slice_fail_from_b64() {
            // too short to work
            let b64 = Base64::from(&[0u8; 12][..]);
            let received: Result<[u8; 32], _> = b64.try_into();
            assert!(received.is_err());
        }

        #[test]
        fn long_byte_slice_fail_from_b64() {
            // too long to work
            let b64 = Base64::from(&[0u8; 64][..]);
            let received: Result<[u8; 32], _> = b64.try_into();
            assert!(received.is_err());
        }

        proptest! {
            #[test]
            fn serde_roundtrip(b64 in any::<Base64>()) {
                let ser = serde_json::to_string(&b64).unwrap();
                let de = serde_json::from_str(&ser).unwrap();
                assert_eq!(b64, de);
            }

            #[test]
            fn deref(vec in collection::vec(any::<u8>(), 0..20)) {
                let testb = Base64(vec.clone());
                let mut testv = vec![2];
                testv.extend_from_slice(&testb);
                let mut expected = vec![2];
                expected.extend_from_slice(&vec);
                assert_eq!(expected, testv);
            }

            #[test]
            fn deref_mut(mut vec in collection::vec(any::<u8>(), 2..20)) {
                let mut testb = Base64(vec.clone());
                testb.swap(0, 1);
                vec.swap(0, 1);
                assert_eq!(vec, testb.0);
            }

            #[test]
            fn str_roundtrip(vec in collection::vec(any::<u8>(), 0..20)) {
                let b64 = Base64(vec);
                let b64_post = b64.to_string().parse().unwrap();
                assert_eq!(b64, b64_post);
            }

            #[test]
            fn to_bytes(vec in collection::vec(any::<u8>(), 0..20)) {
                let expected = bytes::Bytes::from(vec.clone());
                let b64 = Base64(vec);
                let received: bytes::Bytes = b64.into();
                assert_eq!(expected, received);
            }

            #[test]
            fn from_byte_slice(arr in array::uniform3(any::<u8>())) {
                let slice = &arr[..];
                let expected = Base64(arr.to_vec());
                let received = Base64::from(slice);
                assert_eq!(expected, received);
            }

            #[test]
            fn from_bytes_borrow(vec in collection::vec(any::<u8>(), 0..20)) {
                let bytes = bytes::Bytes::from(vec.clone());
                let expected = Base64(vec);
                let received = Base64::from(&bytes);
                assert_eq!(expected, received);
            }

            #[test]
            fn from_bytes_owned(vec in collection::vec(any::<u8>(), 0..20)) {
                let bytes = bytes::Bytes::from(vec.clone());
                let expected = Base64(vec);
                let received = Base64::from(bytes);
                assert_eq!(expected, received);
            }

            #[test]
            fn from_urlbase64(vec in collection::vec(any::<u8>(), 0..20)) {
                let url_base64 = UrlBase64(vec.clone());
                let expected = Base64(vec);
                let received = Base64::from(url_base64);
                assert_eq!(expected, received);
            }

            #[test]
            fn aes_key_try_from_b64(key in prop::array::uniform32(0u8..)) {
                let b64 = Base64::from(&key[..]);
                let received: [u8; 32] = b64.try_into().unwrap();
                assert_eq!(key, received);
            }
        }
    }
}
