use super::*;

#[test]
fn test_new() {
    let data = b"Hello, World!".to_vec().into_boxed_slice();
    let message = Message::new(1, data.clone());
    
    assert_eq!(message.id, 1);
    assert_eq!(message.data, data);
    assert_eq!(message.hash.len(), 32); // SHA-256 produces 32 bytes
}

#[test]
fn test_deserialize() {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&100u64.to_be_bytes());
    buffer.extend_from_slice(&[0u8; 32]); // dummy hash
    buffer.extend_from_slice(b"test");
    
    let message = Message::deserialize(&buffer);
    
    assert_eq!(message.id, 100);
    assert_eq!(message.hash.len(), 32);
    assert_eq!(message.data, b"test".to_vec().into_boxed_slice());
}

#[test]
fn test_check_hash() {
    let data = b"test data".to_vec().into_boxed_slice();
    let message = Message::new(42, data);
    
    assert!(message.check_hash());
}

#[test]
fn test_display() {
    let data = b"Hello".to_vec().into_boxed_slice();
    let message = Message::new(1, data);
    
    let display_string = format!("{}", message);
    assert!(display_string.contains("#1"));
    assert!(display_string.contains("Hello"));
}
