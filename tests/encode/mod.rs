use embedded_msgpack::encode::{Binary, Serializable};

fn print_slice(data: &[u8]) {
    print!("[");
    for (i, v) in data.iter().enumerate() {
        print!("{}0x{:02x}", if i > 0 { ", " } else { "" }, v);
    }
    println!("]");
}

fn test_encode_direct<T: Serializable>(data: &T, expected: &[u8]) {
    let mut buf = [0u8; 1000];
    let len = data.write_into_slice(&mut buf[..]).unwrap();
    print_slice(&buf[..len]);
    assert_eq!(expected.len(), len);
    assert_eq!(expected, &buf[..len]);
}
#[cfg(feature = "serde")]
fn test_encode_serde<T: serde::Serialize>(data: &T, expected: &[u8]) {
    let mut buf = [0u8; 1000];
    let len = embedded_msgpack::encode::serde::to_array(data, &mut buf).unwrap();
    print_slice(&buf[..len]);
    assert_eq!(expected.len(), len);
    assert_eq!(expected, &buf[..len]);
}
#[cfg(feature = "serde")]
fn test_encode<T>(data: T, expected: &[u8])
where
    T: Serializable + serde::Serialize,
{
    test_encode_direct(&data, expected);
    test_encode_serde(&data, expected);
}
#[cfg(not(feature = "serde"))]
fn test_encode<T>(data: T, expected: &[u8])
where
    T: Serializable,
{
    test_encode_direct(&data, expected);
}

#[test]
fn encode_nil() {
    let nil: Option<u8> = None;
    test_encode(nil, &[0xc0]);
}
#[test]
fn encode_bool() {
    test_encode(true, &[0xc3]);
    test_encode(false, &[0xc2]);
}
#[cfg(feature = "timestamp")]
#[test]
fn encode_timestamp() {
    use embedded_msgpack::timestamp::Timestamp;
    test_encode_direct(
        &Timestamp::new(1514862245, 0).unwrap(),
        &[0xd6, 0xff, 0x5a, 0x4a, 0xf6, 0xa5],
    );
    test_encode_direct(
        &Timestamp::new(1514862245, 678901234).unwrap(),
        &[0xd7, 0xff, 0xa1, 0xdc, 0xd7, 0xc8, 0x5a, 0x4a, 0xf6, 0xa5],
    );
    test_encode_direct(
        &Timestamp::new(2147483647, 999999999).unwrap(),
        &[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xfc, 0x7f, 0xff, 0xff, 0xff],
    );
    test_encode_direct(
        &Timestamp::new(2147483648, 0).unwrap(),
        &[0xd6, 0xff, 0x80, 0x00, 0x00, 0x00],
    );
    test_encode_direct(
        &Timestamp::new(2147483648, 1).unwrap(),
        &[0xd7, 0xff, 0x00, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x00],
    );
    test_encode_direct(
        &Timestamp::new(4294967295, 0).unwrap(),
        &[0xd6, 0xff, 0xff, 0xff, 0xff, 0xff],
    );
    test_encode_direct(
        &Timestamp::new(4294967295, 999999999).unwrap(),
        &[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xfc, 0xff, 0xff, 0xff, 0xff],
    );
    test_encode_direct(
        &Timestamp::new(4294967296, 0).unwrap(),
        &[0xd7, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode_direct(
        &Timestamp::new(17179869183, 999999999).unwrap(),
        &[0xd7, 0xff, 0xee, 0x6b, 0x27, 0xff, 0xff, 0xff, 0xff, 0xff],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(17179869184, 0).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
            0x00,
        ],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(-1, 0).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff,
        ],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(-1, 999999999).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff,
        ],
    );
    test_encode_direct(
        &Timestamp::new(0, 0).unwrap(),
        &[0xd6, 0xff, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode_direct(
        &Timestamp::new(0, 1).unwrap(),
        &[0xd7, 0xff, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode_direct(
        &Timestamp::new(1, 0).unwrap(),
        &[0xd6, 0xff, 0x00, 0x00, 0x00, 0x01],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(-2208988801, 999999999).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7c, 0x55, 0x81,
            0x7f,
        ],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(-2208988800, 0).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x7c, 0x55, 0x81,
            0x80,
        ],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(-62167219200, 0).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xf1, 0x86, 0x8b, 0x84,
            0x00,
        ],
    );
    #[cfg(feature = "timestamp96")]
    test_encode_direct(
        &Timestamp::new(253402300799, 999999999).unwrap(),
        &[
            0xc7, 0x0c, 0xff, 0x3b, 0x9a, 0xc9, 0xff, 0x00, 0x00, 0x00, 0x3a, 0xff, 0xf4, 0x41,
            0x7f,
        ],
    );
}

#[test]
fn encode_int() {
    test_encode(-1i32, &[0xff]);
    test_encode(-32i32, &[0xe0]);
    test_encode(-33i32, &[0xd0, 0xdf]);
    test_encode(-128i32, &[0xd0, 0x80]);
    test_encode(-256i32, &[0xd1, 0xff, 0x00]);
    test_encode(-32768i32, &[0xd1, 0x80, 0x00]);
    test_encode(-65536i32, &[0xd2, 0xff, 0xff, 0x00, 0x00]);
    test_encode(-2147483648i32, &[0xd2, 0x80, 0x00, 0x00, 0x00]);
}
#[test]
fn encode_uint() {
    test_encode(4u32, &[4]);
    test_encode(4u8, &[4]);
    test_encode(255u8, &[0xcc, 0xff]);
    test_encode(255u16, &[0xcc, 0xff]);
    test_encode(255u32, &[0xcc, 0xff]);
    test_encode(256u16, &[0xcd, 0x01, 0x00]);
    test_encode(256u32, &[0xcd, 0x01, 0x00]);
    test_encode(65535u16, &[0xcd, 0xff, 0xff]);
    test_encode(65535u32, &[0xcd, 0xff, 0xff]);
    test_encode(65536u32, &[0xce, 0x00, 0x01, 0x00, 0x00]);
    test_encode(2147483647u32, &[0xce, 0x7f, 0xff, 0xff, 0xff]);
    test_encode(2147483648u32, &[0xce, 0x80, 0x00, 0x00, 0x00]);
    test_encode(4294967295u32, &[0xce, 0xff, 0xff, 0xff, 0xff]);

    test_encode(4i32, &[4]);
    test_encode(255i32, &[0xcc, 0xff]);
    test_encode(256i32, &[0xcd, 0x01, 0x00]);
    test_encode(65535i32, &[0xcd, 0xff, 0xff]);
    test_encode(65536i32, &[0xce, 0x00, 0x01, 0x00, 0x00]);
    test_encode(2147483647i32, &[0xce, 0x7f, 0xff, 0xff, 0xff]);
}
#[cfg(feature = "u64")]
#[test]
fn encode_u64() {
    test_encode(
        4294967296u64,
        &[0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode(
        281474976710656u64,
        &[0xcf, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode(
        9223372036854775807u64,
        &[0xcf, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    );
    test_encode(
        9223372036854775808u64,
        &[0xcf, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode(
        18446744073709551615u64,
        &[0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    );
}
#[cfg(feature = "i64")]
#[test]
fn encode_i64() {
    test_encode(2147483648i64, &[0xce, 0x80, 0x00, 0x00, 0x00]);
    test_encode(4294967295i64, &[0xce, 0xff, 0xff, 0xff, 0xff]);
    test_encode(
        -4294967296i64,
        &[0xd3, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode(
        -281474976710656i64,
        &[0xd3, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    test_encode(
        9223372036854775807i64,
        &[0xcf, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    );
    test_encode(
        -9223372036854775807i64,
        &[0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    );
    test_encode(
        -9223372036854775808i64,
        &[0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
}
#[test]
fn encode_float() {
    test_encode(0.5f32, &[0xca, 0x3f, 0x00, 0x00, 0x00]);
    test_encode(-0.5f32, &[0xca, 0xbf, 0x00, 0x00, 0x00]);
}
#[test]
fn encode_map() {
    let map: &[(&str, u32)] = &[("abc", 34), ("def", 128)];
    test_encode_direct(
        &map,
        &[
            0x82, 0xA3, 0x61, 0x62, 0x63, 0x22, 0xA3, 0x64, 0x65, 0x66, 0xCC, 0x80,
        ],
    );
}
#[test]
fn encode_slice() {
    test_encode(
        &["abc", "def"][..],
        &[0x92, 0xA3, 0x61, 0x62, 0x63, 0xA3, 0x64, 0x65, 0x66],
    );
    test_encode(&[1u32, 2, 3][..], &[0x93, 1, 2, 3]);
}
#[test]
fn encode_str() {
    test_encode("", &[0xa0]);
    test_encode("a", &[0xa1, 0x61]);
    test_encode(
        "1234567890123456789012345678901",
        &[
            0xbf, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33,
            0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x30, 0x31,
        ],
    );
    test_encode(
        "12345678901234567890123456789012",
        &[
            0xd9, 0x20, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32,
            0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
            0x37, 0x38, 0x39, 0x30, 0x31, 0x32,
        ],
    );
}
#[test]
fn encode_bin() {
    test_encode_direct(&Binary::new(&[]), &[0xc4, 0x00]);
    test_encode_direct(&Binary::new(&[1]), &[0xc4, 0x01, 0x01]);
    test_encode_direct(&Binary::new(&[0, 0xff]), &[0xc4, 0x02, 0x00, 0xff]);
    test_encode_direct(
        &Binary::new(&[1u8, 2, 3, 4, 5, 6, 7]),
        &[0xc4, 7, 1, 2, 3, 4, 5, 6, 7],
    );
}

#[cfg(feature = "serde")]
#[test]
fn encode_struct() {
    use serde::Serialize;
    #[derive(Serialize)]
    struct Test {
        a: Option<i32>,
        b: u32,
    }
    test_encode_serde(
        &Test { a: None, b: 1 },
        &[0x82, 0xa1, 0x61, 0xc0, 0xa1, 0x62, 0x01],
    );
    test_encode_serde(
        &Test { a: Some(1), b: 2 },
        &[0x82, 0xa1, 0x61, 0x01, 0xa1, 0x62, 0x02],
    );
}
#[cfg(feature = "serde")]
#[test]
fn encode_complex_struct() {
    use serde::Serialize;
    #[derive(Serialize)]
    struct Test {
        a: Option<i32>,
        b: u32,
        c: (i8, [u8; 3]),
    }
    test_encode_serde(
        &Test {
            a: None,
            b: 1,
            c: (2, [3, 4, 5]),
        },
        &[
            0x83, 0xa1, 0x61, 0xc0, 0xa1, 0x62, 0x01, 0xa1, 0x63, 0x92, 0x02, 0x93, 0x03, 0x04,
            0x05,
        ],
    );
    test_encode_serde(
        &Test {
            a: Some(1),
            b: 2,
            c: (2, [3, 4, 5]),
        },
        &[
            0x83, 0xa1, 0x61, 0x01, 0xa1, 0x62, 0x02, 0xa1, 0x63, 0x92, 0x02, 0x93, 0x03, 0x04,
            0x05,
        ],
    );
}
