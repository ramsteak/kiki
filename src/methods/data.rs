use sha2::{Digest, Sha256};

pub fn hash_key(key: Option<&String>) -> u64 {
    let key = match key {
        Some(key) => key.as_str(),
        None => "",
    };
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let res = hasher.finalize();

    u64::from_be_bytes(res[0..8].try_into().unwrap())
}

pub fn package_data(secret_data: &[u8]) -> Vec<u8> {
    let len_bytes = (secret_data.len() as u32).to_be_bytes();
    let crc_bytes = crc32fast::hash(secret_data).to_be_bytes();

    let total_len = len_bytes.len() + secret_data.len() + crc_bytes.len();

    let mut data = Vec::with_capacity(total_len);
    data.extend(len_bytes);
    data.extend_from_slice(secret_data);
    data.extend(crc_bytes);

    data
}

pub struct BitIterator<'a> {
    data: &'a [u8],
    byteidx: usize,
    bitidx: usize,
}

impl<'a> BitIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitIterator {
            data,
            byteidx: 0,
            bitidx: 7,
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byteidx >= self.data.len() {
            return None;
        };

        let bit = (self.data[self.byteidx] >> self.bitidx) & 1;

        (self.byteidx, self.bitidx) = match self.bitidx {
            0 => (self.byteidx + 1, 7),
            _ => (self.byteidx, self.bitidx - 1),
        };
        Some(bit)
    }
}

pub struct BatchIterator<I> {
    iter: I,
    size: usize,
}

impl<I, T> BatchIterator<I>
where
    I: Iterator<Item = T>,
{
    pub fn new(iter: I, size: usize) -> Self {
        BatchIterator { iter, size }
    }
}

impl<I, T> Iterator for BatchIterator<I>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut batch = self
            .iter
            .by_ref()
            .take(self.size)
            .map(Some)
            .collect::<Vec<_>>();
        if batch.len() == 0 {
            None
        } else if batch.len() == self.size {
            Some(batch)
        } else {
            batch.resize_with(self.size, || None);
            Some(batch)
        }

        // let tuple = Vec::from_iter((0..self.size).map(|_| self.iter.next()));
        // match tuple[0] {
        //     None => None,
        //     Some(_) => Some(tuple)
        // }
    }
}
