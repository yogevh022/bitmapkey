
#[macro_export]
macro_rules! bitmap_key {
    ($key:ident, $bits:expr) => {
        const _: () = {
            assert!($bits > 0, "key bits must be greater than 0");
            assert!($bits % u64::BITS as usize == 0, "key bits must be a multiple of u64::BITS");
        };

        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        struct $key(pub(crate) [u64; $bits / u64::BITS as usize]);

        impl $crate::sealed::Sealed for $key {}

        impl $crate::BitmapKey for $key {
            const EMPTY: Self = $key([0; $bits / u64::BITS as usize]);

            fn words(&self) -> &[u64] {
                &self.0
            }

            fn words_mut(&mut self) -> &mut [u64] {
                &mut self.0
            }
        }

        impl std::fmt::Debug for $key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{} [", stringify!($key))?;

                for word in &self.0 {
                    writeln!(
                        f,
                        "    {:0width$b}",
                        word,
                        width = u64::BITS as usize
                    )?;
                }

                write!(f, "]")
            }
        }

        impl std::fmt::Display for $key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({})",
                    stringify!($key),
                    self.0.iter().map(|word| format!("{}", word)).collect::<Vec<_>>().join(".")
                )
            }
        }
    }
}

#[doc(hidden)]
pub mod sealed {
    pub trait Sealed {}
}

pub trait BitmapKey: Sized + sealed::Sealed {
    const EMPTY: Self;

    fn words(&self) -> &[u64];
    fn words_mut(&mut self) -> &mut [u64];

    fn with_id(mut self, id: usize) -> Self {
        let component_bit = id as u32 - 1;
        let bit = component_bit % u64::BITS;
        let word = component_bit / u64::BITS;
        self.words_mut()[word as usize] |= 1 << bit;
        self
    }

    fn without_id(mut self, id: usize) -> Self {
        let component_bit = id as u32 - 1;
        let bit = component_bit % u64::BITS;
        let word = component_bit / u64::BITS;
        self.words_mut()[word as usize] &= !(1 << bit);
        self
    }

    fn contains(&self, other: &Self) -> bool {
        self.words()
            .iter()
            .zip(other.words().iter())
            .all(|(self_word, other_word)| self_word & other_word == *other_word)
    }

    fn disjoint(&self, other: &Self) -> bool {
        self.words()
            .iter()
            .zip(other.words().iter())
            .all(|(word, other_word)| word & other_word == 0)
    }

    fn count_ones(&self) -> usize {
        self.words().iter().map(|word| word.count_ones() as usize).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    bitmap_key!(TestKey, 512);
    bitmap_key!(TestKey2, 64);

    #[test]
    fn it_works() {
        let z = TestKey::EMPTY.with_id(12).with_id(333);
        let q = TestKey2::EMPTY.with_id(10);

        println!("{:?}\n{}", z, z);
        println!("{:?}\n{}", q, q);
    }
}
