use rlp::{RlpStream, NULL_RLP};

use crate::trie::{keccak256, EMPTY_ACCOUNT, EMPTY_CODE, EMPTY_ROOT};

#[test]
fn test_empty_value() {
    let hash = keccak256(&[]);
    assert_eq!(&hash, &EMPTY_CODE);

    let hash = keccak256(&NULL_RLP);
    assert_eq!(&hash, &EMPTY_ROOT);

    let mut stream = RlpStream::new();
    stream.begin_list(4);
    stream.append_empty_data();
    stream.append_empty_data();
    stream.append(&EMPTY_ROOT.as_ref());
    stream.append(&EMPTY_CODE.as_ref());
    let empty_account = stream.out().to_vec();
    assert_eq!(&empty_account, &EMPTY_ACCOUNT);
}
