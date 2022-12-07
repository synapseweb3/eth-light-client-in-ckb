use alloc::{borrow::ToOwned as _, vec::Vec};
use core::cmp;

use rlp::decode_list;
use tiny_keccak::{Hasher as _, Keccak};

// Root hash of an empty trie.
// 0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421
pub const EMPTY_ROOT: [u8; 32] = [
    86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153,
    108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33,
];
// Hash of the empty EVM bytecode:
// 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470.
pub const EMPTY_CODE: [u8; 32] = [
    197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202,
    130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112,
];

pub const EMPTY_ACCOUNT: [u8; 70] = [
    248, 68, 128, 128, 160, 86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248,
    110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 160, 197, 210, 70,
    1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59,
    123, 250, 216, 4, 93, 133, 164, 112,
];

pub fn keccak256(slice: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(slice);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

pub fn verify_proof(proof: &[Vec<u8>], root: &[u8], path: &[u8], value: &[u8]) -> bool {
    let mut expected_hash = root.to_owned();
    let mut path_offset = 0;

    for (i, node) in proof.iter().enumerate() {
        if expected_hash != keccak256(node).to_vec() {
            return false;
        }

        let node_list: Vec<Vec<u8>> = decode_list(node);

        if node_list.len() == 17 {
            if i == proof.len() - 1 {
                let nibble = get_nibble(path, path_offset);
                let node = &node_list[nibble as usize];

                if node.is_empty() && is_empty_value(value) {
                    return true;
                }
            } else {
                let nibble = get_nibble(path, path_offset);
                expected_hash = node_list[nibble as usize].clone();

                path_offset += 1;
            }
        } else if node_list.len() == 2 {
            if i == proof.len() - 1 {
                if !paths_match(&node_list[0], skip_length(&node_list[0]), path, path_offset)
                    && is_empty_value(value)
                {
                    return true;
                }

                if node_list[1] == value {
                    return paths_match(
                        &node_list[0],
                        skip_length(&node_list[0]),
                        path,
                        path_offset,
                    );
                }
            } else {
                let node_path = &node_list[0];
                let prefix_length = shared_prefix_length(path, path_offset, node_path);
                if prefix_length < node_path.len() * 2 - skip_length(node_path) {
                    return false;
                }
                path_offset += prefix_length;
                expected_hash = node_list[1].clone();
            }
        } else {
            return false;
        }
    }

    false
}

fn paths_match(p1: &[u8], s1: usize, p2: &[u8], s2: usize) -> bool {
    let len1 = p1.len() * 2 - s1;
    let len2 = p2.len() * 2 - s2;
    if len1 != len2 {
        return false;
    }
    for offset in 0..len1 {
        let n1 = get_nibble(p1, s1 + offset);
        let n2 = get_nibble(p2, s2 + offset);
        if n1 != n2 {
            return false;
        }
    }
    true
}

fn shared_prefix_length(path: &[u8], path_offset: usize, node_path: &[u8]) -> usize {
    let skip_length = skip_length(node_path);

    let len = cmp::min(
        node_path.len() * 2 - skip_length,
        path.len() * 2 - path_offset,
    );
    let mut prefix_len = 0;

    for i in 0..len {
        let path_nibble = get_nibble(path, i + path_offset);
        let node_path_nibble = get_nibble(node_path, i + skip_length);

        if path_nibble == node_path_nibble {
            prefix_len += 1;
        } else {
            break;
        }
    }

    prefix_len
}

fn skip_length(node: &[u8]) -> usize {
    if node.is_empty() {
        return 0;
    }

    let nibble = get_nibble(node, 0);
    match nibble {
        0 => 2,
        1 => 1,
        2 => 2,
        3 => 1,
        _ => 0,
    }
}

fn get_nibble(path: &[u8], offset: usize) -> u8 {
    let byte = path[offset / 2];
    if offset % 2 == 0 {
        byte >> 4
    } else {
        byte & 0xF
    }
}

fn is_empty_value(value: &[u8]) -> bool {
    let is_empty_slot = value.len() == 1 && value[0] == 0x80;
    let is_empty_account = value == EMPTY_ACCOUNT;
    is_empty_slot || is_empty_account
}
