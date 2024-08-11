use image::{ImageBuffer, Rgba};
use rand::{rngs::StdRng, Rng};
use std::collections::HashSet;

pub struct RandomPixelIterator {
    imgwh: (u32, u32),
    rng: StdRng,
    used: HashSet<(u32, u32)>,
}

impl RandomPixelIterator {
    pub fn new(imgwh: (u32, u32), rng: StdRng) -> Self {
        RandomPixelIterator {
            imgwh,
            rng,
            used: HashSet::new(),
        }
    }
}

impl Iterator for RandomPixelIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut widx;
        let mut hidx;
        loop {
            widx = self.rng.gen_range(0..self.imgwh.0);
            hidx = self.rng.gen_range(0..self.imgwh.1);

            if !self.used.contains(&(widx, hidx)) {
                self.used.insert((widx, hidx));
                break;
            }
        }

        Some((widx, hidx))
    }
}

pub struct SequentialPixelIterator {
    imgwh: (u32, u32),
    idx: (u32, u32),
}

impl SequentialPixelIterator {
    pub fn new(imgwh: (u32, u32)) -> Self {
        SequentialPixelIterator { imgwh, idx: (0, 0) }
    }
}

impl Iterator for SequentialPixelIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx.1 >= self.imgwh.1 {
            None
        } else {
            let pix = self.idx;

            self.idx.0 += 1;
            if self.idx.0 >= self.imgwh.0 {
                self.idx = (0, self.idx.1 + 1);
            }
            Some(pix)
        }
    }
}

pub enum PixelIterator {
    Sequential(SequentialPixelIterator),
    Random(RandomPixelIterator),
}

impl Iterator for PixelIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PixelIterator::Sequential(iter) => iter.next(),
            PixelIterator::Random(iter) => iter.next(),
        }
    }
}
