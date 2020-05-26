use std::io::{self, Seek, Read, SeekFrom};
use byteorder::{ReadBytesExt, LE};


pub struct BattlePackReader<R: Read + Seek> {
    inner: R,
    section_count: usize,
    section_remaining: usize,
}

impl<R: Read + Seek> std::fmt::Debug for BattlePackReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BattlePackReader{{ section_count: {}, section_remaining: {} }}", self.section_count, self.section_remaining)
    }
}

impl<R: Read + Seek> BattlePackReader<R> {
    pub fn new(inner: R) -> io::Result<Self> {
        let mut inner = inner;
        inner.seek(SeekFrom::Start(4))?;
        let mut section_count = 0;
        loop {
            let offset = inner.read_u32::<LE>()?;
            if offset == 0 {
                break;
            } else {
                section_count += 1;
            }
        }
        Ok(BattlePackReader {
            inner,
            section_count,
            section_remaining: 0,
        })
    }

    pub fn section_count(&self) -> usize { self.section_count }

    pub fn section_size(&mut self, index: usize) -> io::Result<usize> {
        assert!(index < self.section_count, "index out of bounds: {} >= {}", index, self.section_count);
        let end = self.inner.seek(SeekFrom::End(0))? as u32;
        self.inner.seek(SeekFrom::Start(4 + 4 * index as u64))?;
        let offset = self.inner.read_u32::<LE>()?;

        Ok({
            let offset2 = self.inner.read_u32::<LE>()?;
            if index < self.section_count - 1 { offset2 - offset } else {
                end - offset
            }
        } as usize)
    }

    pub fn section_offset(&mut self, index: usize) -> io::Result<u32> {
        assert!(index < self.section_count, "index out of bounds: {} >= {}", index, self.section_count);
        self.inner.seek(SeekFrom::Start(4 + 4 * index as u64))?;
        self.inner.read_u32::<LE>()
    }

    pub fn begin_section(&mut self, index: usize) -> io::Result<()> {
        let offset = self.section_offset(index)? as u64;
        let size = self.section_size(index)?;
        self.section_remaining = size;
        self.inner.seek(SeekFrom::Start(offset)).map(|_| ())
    }

    pub fn read_section(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        if self.section_remaining == 0 {
            Ok(0)
        } else {
            let read_bytes = if dst.len() > self.section_remaining {
                self.section_remaining
            } else {
                dst.len()
            };
            let dst = &mut dst[..read_bytes];
            self.inner.read_exact(dst)?;
            self.section_remaining -= dst.len();
            Ok(dst.len())
        }
    }

    pub fn section_begin_to_end(&mut self, index: usize, dst: &mut Vec<u8>) -> io::Result<usize> {
        let size = self.section_size(index)?;
        if size == 0 { return Ok(0); }
        dst.reserve(size);
        self.begin_section(index)?;
        let mut read = 0;
        let mut buffer = [0; 32];
        loop {
            let buf_size = std::cmp::min(32, size);
            match self.read_section(&mut buffer[..buf_size]) {
                Ok(d) => {

                    read += d;
                    dst.extend_from_slice(&buffer[0..std::cmp::min(buf_size, d)]);

                    if read == size {
                        break Ok(read);
                    }
                }
                Err(err) => {
                    break Err(err);
                }
            }
        }
    }
}
