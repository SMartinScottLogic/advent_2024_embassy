use arrayvec::ArrayVec;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::IResult;

pub fn integer<RT>(input: &[u8]) -> IResult<&[u8], RT>
where
    RT: core::convert::TryFrom<i32>
        + core::convert::TryFrom<u8>
        + core::ops::MulAssign<RT>
        + core::ops::AddAssign,
    <RT as core::convert::TryFrom<u8>>::Error: core::fmt::Debug,
    <RT as core::convert::TryFrom<i32>>::Error: core::fmt::Debug,
{
    map_res(digit1, |digits: &[u8]| {
        Ok::<RT, &[u8]>(digits.iter().fold(0.try_into().unwrap(), |mut acc, v| {
            acc *= 10.try_into().unwrap();
            acc += (v - b'0').try_into().unwrap();
            acc
        }))
    })(input)
}

pub fn list_number<RT, const C: usize>(input: &[u8]) -> IResult<&[u8], ArrayVec<RT, C>>
where
    RT: core::convert::TryFrom<i32>
        + core::convert::TryFrom<u8>
        + core::ops::MulAssign<RT>
        + core::ops::AddAssign
        + core::default::Default,
    <RT as core::convert::TryFrom<u8>>::Error: core::fmt::Debug,
    <RT as core::convert::TryFrom<i32>>::Error: core::fmt::Debug,
{
    fold_many1(
        nom::sequence::tuple((tag(" "), integer::<RT>)),
        ArrayVec::new,
        |mut acc: ArrayVec<RT, C>, item| {
            acc.push(item.1);
            acc
        },
    )(input)
}

pub fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c| c == b'\n' || c == b'\r')(input)
}

pub fn non_newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c| c != b'\n' && c != b'\r')(input)
}

pub fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c| c == b' ' || c == b'\t')(input)
}
