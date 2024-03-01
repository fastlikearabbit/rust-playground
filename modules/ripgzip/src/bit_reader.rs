#![forbid(unsafe_code)]

use std::io::{self, BufRead};
use byteorder::ReadBytesExt;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitSequence {
    bits: u16,
    len: u8,
}

impl BitSequence {
    pub fn new(bits: u16, len: u8) -> Self {
        assert!(len <= 16);
        if len == 0 {
            return Self {
                bits: 0,
                len: 0,
            }
        }
        let bits = (bits << (16 - len)) >> (16 - len);
        Self { bits, len }
    }

    pub fn bits(&self) -> u16 {
        self.bits
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn concat(self, other: Self) -> Self {
        assert!(self.len + other.len <= 16);

        let shifted_bits = self.bits << other.len;
        let new_bits = shifted_bits | (other.bits & ((1 << other.len) - 1));

        BitSequence {
            bits: new_bits,
            len: self.len + other.len,
        }
    }

}

////////////////////////////////////////////////////////////////////////////////

pub struct BitReader<T> {
    stream: T,
    buffer: u8,
    bits_left: u8,
}

impl<T: BufRead> BitReader<T> {
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            buffer: 0,
            bits_left: 0,
        }
    }

    pub fn read_bits(&mut self, len: u8) -> io::Result<BitSequence> {
        let mut result_sequence = BitSequence::new(0, 0);
        let mut bits_to_read = len;

        while bits_to_read > 0 {
            if self.bits_left == 0 {
                self.buffer = self.stream.read_u8()?;
                self.bits_left = 8;
            }

            let read_len = std::cmp::min(self.bits_left, bits_to_read);
            
            let mask = ((1 << read_len) - 1) as u8;
            let bits = (self.buffer >> (8 - read_len)) & mask;
            let current_sequence = BitSequence::new(bits as u16, read_len);

            result_sequence = result_sequence.concat(current_sequence);

            if read_len != 8 { self.buffer <<= read_len; }
            self.bits_left -= read_len;
            bits_to_read -= read_len;
        }

        Ok(result_sequence)
    }

    /// Discard all the unread bits in the current byte and return a mutable reference
    /// to the underlying reader.
    pub fn borrow_reader_from_boundary(&mut self) -> &mut T {
        self.bits_left = 0;
        &mut self.stream
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::ReadBytesExt;

    #[test]
    fn read_bits() -> io::Result<()> {
        let data: &[u8] = &[0b10110011, 0b01101100, 0b10111110];
        let mut reader = BitReader::new(data);
        assert_eq!(reader.read_bits(1)?, BitSequence::new(0b1, 1));
        assert_eq!(reader.read_bits(2)?, BitSequence::new(0b01, 2));
        assert_eq!(reader.read_bits(3)?, BitSequence::new(0b100, 3));
        assert_eq!(reader.read_bits(4)?, BitSequence::new(0b1101, 4));
        assert_eq!(reader.read_bits(5)?, BitSequence::new(0b10110, 5));
        assert_eq!(reader.read_bits(8)?, BitSequence::new(0b01011111, 8));
        assert_eq!(
            reader.read_bits(2).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        Ok(())
    }

    #[test]
    fn borrow_reader_from_boundary() -> io::Result<()> {
        let data: &[u8] = &[0b01100011, 0b11011011, 0b10101111];
        let mut reader = BitReader::new(data);
        assert_eq!(reader.read_bits(3)?, BitSequence::new(0b011, 3));
        assert_eq!(reader.borrow_reader_from_boundary().read_u8()?, 0b11011011);
        assert_eq!(reader.read_bits(8)?, BitSequence::new(0b10101111, 8));
        Ok(())
    }
}
