// copyright 2017 Kaz Wesley

//! BLAKE, the SHA-3 hash finalist based on the ChaCha cipher

//#![no_std]
//#![cfg_attr(not(feature = "std"), no_std)]
//#[cfg(feature = "std")]
//use std as core;
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#![cfg_attr(
    any(not(feature = "std"),
    all(feature = "mesalock_sgx",
                not(target_env = "sgx"))), no_std)]

#[cfg(all(feature = "std", feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate block_buffer;
pub extern crate digest;
//#[macro_use]
pub extern crate simd;

mod consts;

use block_buffer::byteorder::{ByteOrder, BE};
use block_buffer::BlockBuffer;
use std::mem;
use digest::generic_array::typenum::{PartialDiv, Unsigned, U2};
use digest::generic_array::GenericArray;
pub use digest::Digest;
use simd::*;

use simd::{
    vec128_storage, vec256_storage, Machine, RotateEachWord32, RotateEachWord64, StoreBytes, Words4,
};

#[inline(always)]
fn round32<M: Machine>(
    (mut a, mut b, mut c, mut d): (M::u32x4, M::u32x4, M::u32x4, M::u32x4),
    m0: M::u32x4,
    m1: M::u32x4,
) -> (M::u32x4, M::u32x4, M::u32x4, M::u32x4) {
    a += m0;
    a += b;
    d ^= a;
    d = d.rotate_each_word_right16();
    c += d;
    b ^= c;
    b = b.rotate_each_word_right12();
    a += m1;
    a += b;
    d ^= a;
    d = d.rotate_each_word_right8();
    c += d;
    b ^= c;
    b = b.rotate_each_word_right7();
    (a, b, c, d)
}

#[inline(always)]
fn round64<M: Machine>(
    (mut a, mut b, mut c, mut d): (M::u64x4, M::u64x4, M::u64x4, M::u64x4),
    m0: M::u64x4,
    m1: M::u64x4,
) -> (M::u64x4, M::u64x4, M::u64x4, M::u64x4) {
    a += m0;
    a += b;
    d ^= a;
    d = d.rotate_each_word_right32();
    c += d;
    b ^= c;
    b = b.rotate_each_word_right25();
    a += m1;
    a += b;
    d ^= a;
    d = d.rotate_each_word_right16();
    c += d;
    b ^= c;
    b = b.rotate_each_word_right11();
    (a, b, c, d)
}

#[inline(always)]
fn diagonalize<X4: Words4>((a, b, c, d): (X4, X4, X4, X4)) -> (X4, X4, X4, X4) {
    (a, b.shuffle3012(), c.shuffle2301(), d.shuffle1230())
}

#[inline(always)]
fn undiagonalize<X4: Words4>((a, b, c, d): (X4, X4, X4, X4)) -> (X4, X4, X4, X4) {
    (a, b.shuffle1230(), c.shuffle2301(), d.shuffle3012())
}

macro_rules! define_compressor {
    ($compressor:ident, $storage:ident, $word:ident, $Bufsz:ty, $deserializer:path, $uval:expr, $rounds:expr, $round:ident, $X4:ident) => {
        #[derive(Clone, Copy, Default)]
        pub struct $compressor {
            h: [$storage; 2],
        }

        #[allow(non_snake_case)]
        pub mod $X4 {
            use super::*;
            #[inline(always)]
            pub fn put_block<M: Machine>(mach: M, state: &mut $compressor, block: &GenericArray<u8, $Bufsz>, t: ($word, $word)) {
                const U: [$word; 16] = $uval;

                let mut m = [0; 16];
                for (mx, b) in m
                    .iter_mut()
                    .zip(block.chunks_exact(mem::size_of::<$word>()))
                {
                    *mx = $deserializer(b);
                }

                let u = (mach.vec([U[0], U[1], U[2], U[3]]), mach.vec([U[4], U[5], U[6], U[7]]));
                let mut xs: (M::$X4, M::$X4, _, _) = (mach.unpack(state.h[0]), mach.unpack(state.h[1]), u.0, u.1);
                xs.3 ^= mach.vec([t.0, t.0, t.1, t.1]);
                for sigma in &SIGMA[..$rounds] {
                    macro_rules! m0 { ($e:expr) => (m[sigma[$e] as usize] ^ U[sigma[$e + 1] as usize]) }
                    macro_rules! m1 { ($e:expr) => (m[sigma[$e + 1] as usize] ^ U[sigma[$e] as usize]) }
                    // column step
                    let m0 = mach.vec([m0!(0), m0!(2), m0!(4), m0!(6)]);
                    let m1 = mach.vec([m1!(0), m1!(2), m1!(4), m1!(6)]);
                    xs = $round::<M>(xs, m0, m1);
                    // diagonal step
                    let m0 = mach.vec([m0!(8), m0!(10), m0!(12), m0!(14)]);
                    let m1 = mach.vec([m1!(8), m1!(10), m1!(12), m1!(14)]);
                    xs = undiagonalize($round::<M>(diagonalize(xs), m0, m1));
                }
                let h: (M::$X4, M::$X4) = (mach.unpack(state.h[0]), mach.unpack(state.h[1]));
                state.h[0] = (h.0 ^ xs.0 ^ xs.2).into();
                state.h[1] = (h.1 ^ xs.1 ^ xs.3).into();
            }
        }

        impl $compressor {
            #[inline(always)]
            fn put_block(&mut self, block: &GenericArray<u8, $Bufsz>, t: ($word, $word)) {
                dispatch!(mach, M, {
                    fn put_block(state: &mut $compressor, block: &GenericArray<u8, $Bufsz>, t: ($word, $word)) {
                            $X4::put_block(mach, state, block, t)
                    }
                });
                put_block(self, block, t);
            }

            fn finalize(self) -> GenericArray<u8, <$Bufsz as PartialDiv<U2>>::Output> {
                dispatch_light256!(mach, M, {
                    fn finalize(h: &[$storage; 2], out: &mut GenericArray<u8, <$Bufsz as PartialDiv<U2>>::Output>) {
                        let len = out.len();
                        let o = out.split_at_mut(len / 2);
                        let h0: M::$X4 = mach.unpack(h[0]);
                        let h1: M::$X4 = mach.unpack(h[1]);
                        h0.write_be(o.0);
                        h1.write_be(o.1);
                    }
                });
                let mut out = GenericArray::default();
                finalize(&self.h, &mut out);
                out
            }
        }
    };
}

macro_rules! define_hasher {
    ($name:ident, $word:ident, $buf:expr, $Bufsz:ty, $bits:expr, $Bytes:ident,
     $serializer:path, $compressor:ident, $iv:expr) => {
        #[derive(Clone)]
        pub struct $name {
            compressor: $compressor,
            buffer: BlockBuffer<$Bufsz>,
            t: ($word, $word),
        }

        impl $name {
            fn increase_count(t: &mut ($word, $word), count: $word) {
                let (new_t0, carry) = t.0.overflowing_add(count * 8);
                t.0 = new_t0;
                if carry {
                    t.1 += 1;
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                f.debug_struct("(Blake)").finish()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    compressor: $compressor {
                        h: [$iv[0].into(), $iv[1].into()],
                    },
                    buffer: BlockBuffer::default(),
                    t: (0, 0),
                }
            }
        }

        impl digest::BlockInput for $name {
            type BlockSize = $Bytes;
        }

        impl digest::Input for $name {
            fn input<T: AsRef<[u8]>>(&mut self, data: T) {
                let compressor = &mut self.compressor;
                let t = &mut self.t;
                self.buffer.input(data.as_ref(), |block| {
                    Self::increase_count(t, (mem::size_of::<$word>() * 16) as $word);
                    compressor.put_block(block, *t);
                });
            }
        }

        impl digest::FixedOutput for $name {
            type OutputSize = $Bytes;

            fn fixed_result(self) -> GenericArray<u8, $Bytes> {
                let mut compressor = self.compressor;
                let mut buffer = self.buffer;
                let mut t = self.t;

                Self::increase_count(&mut t, buffer.position() as $word);

                let mut msglen = [0u8; $buf / 8];
                $serializer(&mut msglen[..$buf / 16], t.1);
                $serializer(&mut msglen[$buf / 16..], t.0);

                let footerlen = 1 + 2 * mem::size_of::<$word>();

                // low bit indicates full-length variant
                let isfull = ($bits == 8 * mem::size_of::<[$word; 8]>()) as u8;
                // high bit indicates fit with no padding
                let exactfit = if buffer.position() + footerlen != $buf {
                    0x00
                } else {
                    0x80
                };
                let magic = isfull | exactfit;

                // if header won't fit in last data block, pad to the end and start a new one
                let extra_block = buffer.position() + footerlen > $buf;
                if extra_block {
                    let pad = $buf - buffer.position();
                    buffer.input(&PADDING[..pad], |block| compressor.put_block(block, t));
                    debug_assert_eq!(buffer.position(), 0);
                }

                // pad last block up to footer start point
                if buffer.position() == 0 {
                    // don't xor t when the block is only padding
                    t = (0, 0);
                }
                // skip begin-padding byte if continuing padding
                let x = extra_block as usize;
                let (start, end) = (x, x + ($buf - footerlen - buffer.position()));
                buffer.input(&PADDING[start..end], |_| unreachable!());
                buffer.input(&[magic], |_| unreachable!());
                buffer.input(&msglen, |block| compressor.put_block(block, t));
                debug_assert_eq!(buffer.position(), 0);

                GenericArray::clone_from_slice(&compressor.finalize()[..$Bytes::to_usize()])
            }
        }

        impl digest::Reset for $name {
            fn reset(&mut self) {
                *self = Self::default()
            }
        }
    };
}

use consts::{
    BLAKE224_IV, BLAKE256_IV, BLAKE256_U, BLAKE384_IV, BLAKE512_IV, BLAKE512_U, PADDING, SIGMA,
};
use digest::generic_array::typenum::{U128, U28, U32, U48, U64};

#[rustfmt::skip]
define_compressor!(Compressor256, vec128_storage, u32, U64, BE::read_u32, BLAKE256_U, 14, round32, u32x4);

#[rustfmt::skip]
define_hasher!(Blake224, u32, 64, U64, 224, U28, BE::write_u32, Compressor256, BLAKE224_IV);

#[rustfmt::skip]
define_hasher!(Blake256, u32, 64, U64, 256, U32, BE::write_u32, Compressor256, BLAKE256_IV);

#[rustfmt::skip]
define_compressor!(Compressor512, vec256_storage, u64, U128, BE::read_u64, BLAKE512_U, 16, round64, u64x4);

#[rustfmt::skip]
define_hasher!(Blake384, u64, 128, U128, 384, U48, BE::write_u64, Compressor512, BLAKE384_IV);

#[rustfmt::skip]
define_hasher!(Blake512, u64, 128, U128, 512, U64, BE::write_u64, Compressor512, BLAKE512_IV);
