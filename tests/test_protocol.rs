use redisclient::connection::Reply;
use redisclient::protocol::{RedisDeserializationProtocol, RedisSerializationProtocol};

// #[test]
// pub fn test_vector_serialization() {
//     let data = b"Hello world".to_vec();
//
//     let got = data.serialization();
//
//     let expected = Vec::from("$11\r\nHello world\r\n");
//     assert_eq!(expected, got);
// }

#[test]
pub fn test_string_serialization() {
    let s = String::from("Hello world");

    let got = s.serialization();

    let expected = vec![
        36, 49, 49, 13, 10, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 13, 10,
    ];
    assert_eq!(expected, got);
}

#[test]
pub fn test_u64_serialization() {
    let num: u64 = 132;

    let got = num.serialization();

    let expected = Vec::from("$3\r\n132\r\n");
    assert_eq!(expected, got);
}

#[test]
pub fn test_u64_deserialization() {
    let reply = Reply::Integers(vec![54, 48]);

    let got = <u64>::deserialization(reply).unwrap();

    let expected = 60_u64;
    assert_eq!(expected, got);
}

#[test]
pub fn test_i64_serialization() {
    let num: i64 = -321;

    let got = num.serialization();

    let expected = Vec::from("$4\r\n-321\r\n");
    assert_eq!(expected, got);
}

#[test]
pub fn test_f32_serialization() {
    let fnum: f32 = -1.23;

    let got = fnum.serialization();

    let expected = Vec::from("$5\r\n-1.23\r\n");
    assert_eq!(expected, got);
}

#[test]
pub fn test_f64_serialization() {
    let fnum: f64 = -1.23;

    let got = fnum.serialization();

    let expected = Vec::from("$5\r\n-1.23\r\n");
    assert_eq!(expected, got);
}
