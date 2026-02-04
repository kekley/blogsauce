use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use rusqlite::{ToSql, types::FromSql};

#[repr(transparent)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TruncatedIp(u64);

impl FromSql for TruncatedIp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(TruncatedIp(value.as_i64()? as u64))
    }
}

impl ToSql for TruncatedIp {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_i64().into())
    }
}

impl TruncatedIp {
    // IPv4 tag: top byte 0xFF
    const V4_TAG: u64 = 0xFF00_0000_0000_0000;
    const V4_TAG_MASK: u64 = 0xFF00_0000_0000_0000;

    pub fn new(ip: IpAddr) -> Self {
        match ip {
            IpAddr::V4(v4) => Self::from_ipv4(v4),
            IpAddr::V6(v6) => {
                if let Some(mapped_v4) = v6.to_ipv4() {
                    return Self::from_ipv4(mapped_v4);
                }
                Self::from_ipv6(v6)
            }
        }
    }

    pub fn from_ipv4(ip: Ipv4Addr) -> Self {
        let raw = u32::from(ip);

        // Layout:
        // [0xFF][prefix_len][0x00][0x00][masked_ipv4_u32]
        let encoded = Self::V4_TAG | (32 << 48) | (raw as u64);

        TruncatedIp(encoded)
    }

    pub fn from_ipv6(ip: Ipv6Addr) -> Self {
        if ip.is_multicast() {
            // Would start with 0xFF, colliding with the IPv4 tag
            panic!("unsupported ipv6 address");
        }

        let octets = ip.octets();
        let mut high64 = u64::from_be_bytes(
            octets[0..8]
                .try_into()
                .expect("the slice should be 8 bytes"),
        );

        let mask = !0u64;
        high64 &= mask;

        TruncatedIp(high64)
    }

    pub fn to_i64(self) -> i64 {
        self.0 as i64
    }

    pub fn from_i64(v: i64) -> Self {
        TruncatedIp(v as u64)
    }

    pub fn is_ipv4(self) -> bool {
        (self.0 & Self::V4_TAG_MASK) == Self::V4_TAG
    }

    pub fn decode(self) -> IpAddr {
        if self.is_ipv4() {
            let v4 = Ipv4Addr::from((self.0 & 0xFFFF_FFFF) as u32);
            IpAddr::V4(v4)
        } else {
            let hi = self.0.to_be_bytes();
            let mut full = [0u8; 16];
            full[0..8].copy_from_slice(&hi);
            let prefix = Ipv6Addr::from(full);
            IpAddr::V6(prefix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_round_trip() {
        let id = TruncatedIp::new("203.0.113.42".parse::<IpAddr>().unwrap());
        assert!(id.is_ipv4());
        assert_eq!(id.decode().to_string(), "203.0.113.42");
    }

    #[test]
    fn v6_prefix_64() {
        let ip: IpAddr = "2001:db8:1234:5678:abcd:ef01:2345:6789".parse().unwrap();
        let id = TruncatedIp::new(ip);
        assert!(!id.is_ipv4());

        match id.decode() {
            IpAddr::V4(_) => panic!("expected v6"),
            IpAddr::V6(ipv6_addr) => {
                // lower 64 bits dropped
                assert_eq!(ipv6_addr.to_string(), "2001:db8:1234:5678::");
            }
        }
    }
}
