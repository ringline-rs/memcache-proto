//! Kani proofs for the binary protocol header boundary.

use super::{Opcode, RequestHeader, ResponseHeader, Status};

fn symbolic_opcode() -> Opcode {
    let raw = kani::any();
    kani::assume(Opcode::from_u8(raw).is_some());
    Opcode::from_u8(raw).unwrap()
}

fn symbolic_status() -> Status {
    let raw = kani::any();
    kani::assume(Status::from_u16(raw).is_some());
    Status::from_u16(raw).unwrap()
}

#[kani::proof]
fn request_header_encode_parse_roundtrip() {
    let mut header = RequestHeader::new(symbolic_opcode());
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.data_type = kani::any();
    header.vbucket_id = kani::any();
    header.total_body_length = kani::any();
    header.opaque = kani::any();
    header.cas = kani::any();

    let mut encoded = [0_u8; 24];
    assert_eq!(header.encode(&mut encoded), 24);
    assert_eq!(RequestHeader::parse(&encoded), Ok(header));
}

#[kani::proof]
fn response_header_encode_parse_roundtrip() {
    let mut header = ResponseHeader::new(symbolic_opcode(), symbolic_status());
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.data_type = kani::any();
    header.total_body_length = kani::any();
    header.opaque = kani::any();
    header.cas = kani::any();

    let mut encoded = [0_u8; 24];
    assert_eq!(header.encode(&mut encoded), 24);
    assert_eq!(ResponseHeader::parse(&encoded), Ok(header));
}

#[kani::proof]
fn request_checked_lengths_are_total_and_decompose_body() {
    let mut header = RequestHeader::new(Opcode::Set);
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.total_body_length = kani::any();

    check_lengths(
        header.total_body_length,
        header.extras_length,
        header.key_length,
        header.value_length(),
        header.packet_length(),
    );
}

#[kani::proof]
fn response_checked_lengths_are_total_and_decompose_body() {
    let mut header = ResponseHeader::new(Opcode::Get, Status::NoError);
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.total_body_length = kani::any();

    check_lengths(
        header.total_body_length,
        header.extras_length,
        header.key_length,
        header.value_length(),
        header.packet_length(),
    );
}

#[kani::proof]
fn malformed_request_lengths_are_rejected_without_wrapping() {
    let mut header = RequestHeader::new(Opcode::Set);
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.total_body_length = kani::any();
    kani::assume(
        u64::from(header.extras_length) + u64::from(header.key_length)
            > u64::from(header.total_body_length),
    );

    assert_eq!(header.value_length(), None);
}

#[kani::proof]
fn malformed_response_lengths_are_rejected_without_wrapping() {
    let mut header = ResponseHeader::new(Opcode::Get, Status::NoError);
    header.key_length = kani::any();
    header.extras_length = kani::any();
    header.total_body_length = kani::any();
    kani::assume(
        u64::from(header.extras_length) + u64::from(header.key_length)
            > u64::from(header.total_body_length),
    );

    assert_eq!(header.value_length(), None);
}

fn check_lengths(
    total_body_length: u32,
    extras_length: u8,
    key_length: u16,
    value_length: Option<usize>,
    packet_length: Option<usize>,
) {
    let components = u64::from(extras_length) + u64::from(key_length);
    let body = u64::from(total_body_length);

    match value_length {
        Some(value) => {
            assert!(components <= body);
            assert_eq!(components + value as u64, body);
        }
        None => assert!(components > body || usize::try_from(total_body_length).is_err()),
    }

    let expected_packet_length = usize::try_from(total_body_length)
        .ok()
        .and_then(|body| 24_usize.checked_add(body));
    assert_eq!(packet_length, expected_packet_length);
}
