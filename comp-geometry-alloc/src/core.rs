use std::{
    error, fmt,
    mem::{self, Alignment},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayoutError;

impl error::Error for LayoutError {}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid params to Layout::from_size_align")
    }
}

struct Layout {
    size: usize,
    align: Alignment, // will help for simd alignment
}

impl Layout {
    #[inline]
    pub const fn from_size_align(
        size: usize,
        align: usize,
    ) -> Result<Self, LayoutError> {
        let Some(alignment) = Alignment::new(align) else {
            return Err(LayoutError);
        };
        if size > Self::max_size_for_alignment(alignment) {
            return Err(LayoutError);
        }
        unsafe {
            Ok(Layout {
                size,
                align: mem::transmute(align),
            })
        }
    }
    #[inline]
    const fn max_size_for_alignment(alignment: Alignment) -> usize {
        isize::MAX as usize + 1 - alignment.as_usize()
    }
    #[inline(always)]
    pub fn align_up(n: usize, align: usize) -> usize {
        debug_assert!(align.is_power_of_two());
        (n + align - 1) & !(align - 1)
    }

    #[inline(always)]
    pub fn align_down(n: usize, align: usize) -> usize {
        debug_assert!(align.is_power_of_two());
        n & !(align - 1)
    }
    // size = 16;
    // size = 15
    // msb = BITS - 1 - size.leading_zeros = 3
    // sub = size >> (msb - 1) & 1 = 1
    // ret 6 + 1 - 3 = 4
    //
    // size = 10;
    // size = 9;
    // msb = BITS - 1 - size.leading_zeros = 3
    // sub = 0
    // ret 3
    #[inline(always)]
    fn size_class(size: usize) -> u32 {
        if size <= 8 {
            return 0;
        }
        let size = size - 1;
        // highest bit
        let msb = usize::BITS - 1 - size.leading_zeros();
        // 2 subclasses per power of two range, need to run
        let sub: u32 = ((size >> (msb - 1)) & 1) as u32;
        msb * 2 + sub - 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn align_up_properties() {
        for align in [8usize, 16, 32, 64, 4096] {
            for n in 0..1000 {
                let r = Layout::align_up(n, align);
                assert!(r >= n);
                assert_eq!(r % align, 0);
                assert!(r - n < align);
            }
        }
    }

    #[test]
    fn size_class() {
        let mut last = 0;
        for size in (8..65536).step_by(8) {
            let c = Layout::size_class(size);
            assert!(c >= last);
            last = c;
        }
    }
}
