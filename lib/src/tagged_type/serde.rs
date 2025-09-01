// SPDX-License-Identifier: MIT

use crate::TaggedType;
use crate::TransparentDeserialize;
use crate::TransparentSerialize;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

impl<V: Serialize, T: TransparentSerialize> Serialize for TaggedType<V, T> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.v.serialize(serializer)
    }
}

impl<'de, V: Deserialize<'de>, T: TransparentDeserialize> serde::Deserialize<'de>
    for TaggedType<V, T>
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        V::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::net::IpAddr;

    #[test]
    fn test_serializer() {
        type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
        enum DefaultGatewayTag {}
        impl TransparentSerialize for DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let gw = DefaultGateway::new(IP.parse().unwrap());
        assert_eq!(serde_json::to_string(&gw).unwrap(), r#""192.168.0.1""#);
    }

    #[test]
    fn test_derializer() {
        type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
        enum DefaultGatewayTag {}
        impl TransparentDeserialize for DefaultGatewayTag {}
        impl TransparentDebug for DefaultGatewayTag {}
        impl ImplementPartialEq for DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let expected_gw = DefaultGateway::new(IP.parse().unwrap());
        #[derive(serde::Deserialize)]
        struct Route {
            gateway: DefaultGateway,
        }
        assert_eq!(
            serde_json::from_str::<Route>(r#"{"gateway":"192.168.0.1"}"#)
                .unwrap()
                .gateway,
            expected_gw,
        )
    }

    #[cfg(feature = "provide_derive")]
    #[test]
    fn test_serializer_deserializer_derive() {
        type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
        #[derive(Tag)]
        #[transparent(Serialize, Deserialize, Debug)]
        #[implement(PartialEq, Clone, Copy)]
        enum DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let expected_gw = DefaultGateway::new(IP.parse().unwrap());
        #[derive(serde::Deserialize, serde::Serialize)]
        struct Route {
            gateway: DefaultGateway,
        }

        assert_eq!(
            serde_json::from_str::<Route>(
                &serde_json::to_string(&Route {
                    gateway: expected_gw
                })
                .unwrap()
            )
            .unwrap()
            .gateway,
            expected_gw,
        )
    }
}
