use crate::format::scenario::instructions::NumberSpec;
use binrw::{BinRead, BinResult, BinWrite, Endian, VecArgs};
use smallvec::SmallVec;
use std::fmt;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;

// TODO: make lists generic over the type of length
#[derive(Debug)]
pub struct U8List<T>(pub Vec<T>);

#[derive(Debug)]
pub struct U16List<T>(pub Vec<T>);

pub struct SmallList<L: Into<usize> + TryFrom<usize> + 'static, A: smallvec::Array>(
    pub SmallVec<A>,
    pub PhantomData<L>,
);

pub type U8SmallList<A> = SmallList<u8, A>;
pub type U16SmallList<A> = SmallList<u16, A>;

pub type U8SmallNumberList<A = [NumberSpec; 6]> = U8SmallList<A>;
pub type U16SmallNumberList<A = [NumberSpec; 6]> = U16SmallList<A>;

impl<T: for<'a> BinRead<Args<'a> = ()> + 'static> BinRead for U8List<T> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _: ()) -> BinResult<Self> {
        let len = u8::read_options(reader, endian, ())?;

        Ok(Self(<_>::read_options(
            reader,
            endian,
            VecArgs {
                count: len as usize,
                inner: (),
            },
        )?))
    }
}
impl<T: for<'a> BinWrite<Args<'a> = ()>> BinWrite for U8List<T> {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        _writer: &mut W,
        _endian: Endian,
        _: (),
    ) -> BinResult<()> {
        todo!()
    }
}

impl<T: for<'a> BinRead<Args<'a> = ()> + 'static> BinRead for U16List<T> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _: ()) -> BinResult<Self> {
        let len = u16::read_options(reader, endian, ())?;

        Ok(Self(<_>::read_options(
            reader,
            endian,
            VecArgs {
                count: len as usize,
                inner: (),
            },
        )?))
    }
}
impl<T: for<'a> BinWrite<Args<'a> = ()>> BinWrite for U16List<T> {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        _writer: &mut W,
        _endian: Endian,
        _: (),
    ) -> BinResult<()> {
        todo!()
    }
}

impl<L: Into<usize> + TryFrom<usize>, A: smallvec::Array> fmt::Debug for SmallList<L, A>
where
    A::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("U8SmallList").field(&self.0).finish()
    }
}

impl<
        L: Into<usize> + TryFrom<usize> + for<'a> BinRead<Args<'a> = ()>,
        A: smallvec::Array<Item = T> + 'static,
        T: for<'a> BinRead<Args<'a> = ()>,
    > BinRead for SmallList<L, A>
{
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _: ()) -> BinResult<Self> {
        let len = L::read_options(reader, endian, ())?.into();

        let mut res = SmallVec::new();
        res.reserve(len);
        for _ in 0..len {
            res.push(<_>::read_options(reader, endian, ())?);
        }

        Ok(Self(res, PhantomData {}))
    }
}

impl<
        L: Into<usize> + TryFrom<usize> + for<'a> BinWrite<Args<'a> = ()>,
        A: smallvec::Array<Item = T> + 'static,
        T: for<'a> BinWrite<Args<'a> = ()>,
    > BinWrite for SmallList<L, A>
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        _writer: &mut W,
        _endian: Endian,
        _: (),
    ) -> BinResult<()> {
        todo!()
    }
}
