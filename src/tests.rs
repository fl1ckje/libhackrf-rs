#[cfg(test)]
use crate::freq_params;
#[test]
fn nominal() {
    assert_eq!(freq_params(915_000_000), [0x93, 0x03, 0, 0, 0, 0, 0, 0]);
    assert_eq!(freq_params(915_000_001), [0x93, 0x03, 0, 0, 1, 0, 0, 0]);
    assert_eq!(
        freq_params(123456789),
        [0x7B, 0, 0, 0, 0x55, 0xF8, 0x06, 0x00]
    );
}

#[test]
fn min() {
    assert_eq!(freq_params(0), [0; 8]);
}

#[test]
fn max() {
    assert_eq!(freq_params(u64::MAX), [0xFF; 8]);
}
