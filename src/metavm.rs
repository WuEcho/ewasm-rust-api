// add by echo : 扩展存储，以支持超过 32byte 的 key/value >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
extern "C" {
    pub fn ethereum_storageStore2(
        keyOffset: *const u32, keyLength: u32,
        valueOffset: *const u32, valueLength: u32,
    );
    pub fn ethereum_storageLoad2(
        keyOffset: *const u32, keyLength: u32,
        resultOffset: *const u32, resultType: u32,
    );
}

// PUT Key->Val
//pub fn storage_store2(key: &[u8], keyLen: u32, value: &[u8], valueLen: u32) {
pub fn storage_store(key: &[u8], value: &[u8]) {
    let key_len = key.len();
    let value_len = value.len();
    unsafe {
        ethereum_storageStore2(
            key.as_ptr() as *const u32,
            key_len as u32,
            value.as_ptr() as *const u32,
            value_len as u32,
        );
    }
}

pub fn storage_load(key: &[u8]) -> Vec<u8> {
    let key_len = key.len();
    let mut val_len: [u8; 32] = [0; 32];
    unsafe {
        ethereum_storageLoad2(
            key.as_ptr() as *const u32,
            key_len as u32,
            val_len.as_mut_ptr() as *const u32,
            1,
        );
    }
    let vl = utils::bytes_to_uint(&val_len[..]);
    let mut v: Vec<u8> = vec![0; vl];
    unsafe {
        ethereum_storageLoad2(
            key.as_ptr() as *const u32,
            key_len as u32,
            v.as_mut_slice().as_mut_ptr() as *const u32,
            2,
        );
    }
    v
}

pub mod utils {
    pub fn bytes_to_uint(n: &[u8]) -> usize {
        let size = n.len();
        let mut r: usize = 0;
        for i in 0..size {
            let x = n[i] as usize;
            if x > 0 {
                let m = (size - 1 - i) * 8;
                r = r + (x << m);
            }
        }
        r
    }

    pub fn u32_to_bytes(i: u32) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        r.insert(0, (i >> 24) as u8);
        r.insert(1, (i >> 16) as u8);
        r.insert(2, (i >> 8) as u8);
        r.insert(3, i as u8);
        r
    }

    pub fn u32_to_bytes32(i: u32) -> [u8; 32] {
        let new_b: Vec<u8> = u32_to_bytes(i);
        let mut val: [u8; 32] = [0; 32];
        val[32 - 4] = new_b[0];
        val[32 - 3] = new_b[1];
        val[32 - 2] = new_b[2];
        val[32 - 1] = new_b[3];
        val
    }
}
// add by echo : 扩展存储，以支持超过 32byte 的 key/value <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
