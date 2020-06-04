use std::io::{self, Seek, Read, SeekFrom, Write};
use byteorder::{ReadBytesExt, LE, WriteBytesExt};


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

pub struct BattlePackWriter<W: Write + Seek> {
    inner: W,
    index: usize,
    count: usize
}

impl<W: Write + Seek> BattlePackWriter<W> {
    pub fn new(count: usize, output: W) -> io::Result<BattlePackWriter<W>> {
        let mut output = output;
        // magic
        output.write_all(&[0x47, 0, 0, 0])?;

        let mut sizes: Vec<u8> = Vec::new();
        for _ in 0..count + 1 {
            sizes.extend_from_slice(&[0,0,0,0])
        }
        output.write_all(&sizes)?;

        Ok(Self { inner: output, index: 0, count })
    }

    pub fn write_section(&mut self, data: &[u8]) -> io::Result<()> {
        if self.index == self.count { return Err(io::ErrorKind::WriteZero.into()) }
        let offset = self.inner.seek(SeekFrom::Current(0))? as u32;
        self.inner.write_all(data)?;
        self.inner.seek(SeekFrom::Start(size_offset(self.index) as u64))?;
        self.inner.write_u32::<LE>(offset)?;
        self.inner.seek(SeekFrom::End(0))?;
        self.index += 1;
        Ok(())
    }

    #[allow(unused)]
    pub fn into_inner(self) -> W { self.inner }

}

const fn size_offset(index: usize) -> usize {
    4 + 4 * index
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::battle_pack::io::BattlePackWriter;

    #[test]
    fn writer_test() {
        let mut sections: Vec<Vec<u8>> = vec![vec![0x45, 0x65, 0x99, 0x12], vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]];
        let mut output_data = Cursor::new(Vec::new());
        let mut writer = BattlePackWriter::new(2, output_data).expect("creating writer - writing header");
        writer.write_section(&sections[0]).expect("writing section 1");
        writer.write_section(&sections[1]).expect("writing section 2");
        let output = writer.into_inner();
        assert_eq!(output.into_inner(), vec![0x47u8, 0, 0, 0, 0x10, 0, 0, 0, 0x14, 0, 0, 0, 0, 0, 0, 0, 0x45, 0x65, 0x99, 0x12, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7])
    }

}
