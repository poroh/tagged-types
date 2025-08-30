// SPDX-License-Identifier: MIT

use crate::TaggedType;
use crate::TransparentDeserialize;
use crate::TransparentSerialize;

impl<V, T> serde::Serialize for TaggedType<V, T>
where
    V: serde::Serialize,
    T: TransparentSerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.v.serialize(serializer)
    }
}

impl<'de, V, T> serde::Deserialize<'de> for TaggedType<V, T>
where
    V: serde::Deserialize<'de>,
    T: TransparentDeserialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        V::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_serializer() {
        type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
        enum DefaultGatewayTag {}
        impl TransparentSerialize for DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let gw = DefaultGateway::new(IP.parse().unwrap());
        assert_eq!(serde_json::to_string(&gw).unwrap(), r#""192.168.0.1""#);
    }

    #[test]
    fn test_derializer() {
        type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
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
        type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
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
