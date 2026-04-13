use ptfs_cache::write_buffer::WriteBuffer;

#[test]
fn write_buffer_flushes_to_cache() {
    let mut buffer = WriteBuffer::new();

    buffer.write(1, vec![10, 20]);
    assert_eq!(buffer.pending_writes(), 1);

    buffer.flush();
    assert_eq!(buffer.pending_writes(), 0, "All writes should be flushed");
}
