use bytes::Bytes;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use hash::{H96, H160, H256, H264, H512, H520};
use compact_integer::CompactInteger;
use {Serializable, Stream, Deserializable, Reader, Error};

impl Serializable for bool {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_u8(*self as u8).unwrap();
	}
}

impl Serializable for i32 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_i32::<LittleEndian>(*self).unwrap();
	}
}

impl Serializable for i64 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_i64::<LittleEndian>(*self).unwrap();
	}
}

impl Serializable for u8 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_u8(*self).unwrap();
	}
}

impl Serializable for u16 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_u16::<LittleEndian>(*self).unwrap();
	}
}

impl Serializable for u32 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_u32::<LittleEndian>(*self).unwrap();
	}
}

impl Serializable for u64 {
	#[inline]
	fn serialize(&self, s: &mut Stream) {
		s.write_u64::<LittleEndian>(*self).unwrap();
	}
}

impl Deserializable for bool {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		let value = try!(reader.read_u8());
		match value {
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(Error::MalformedData),
		}
	}
}

impl Deserializable for i32 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_i32::<LittleEndian>()))
	}
}

impl Deserializable for i64 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_i64::<LittleEndian>()))
	}
}

impl Deserializable for u8 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_u8()))
	}
}

impl Deserializable for u16 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_u16::<LittleEndian>()))
	}
}

impl Deserializable for u32 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_u32::<LittleEndian>()))
	}
}

impl Deserializable for u64 {
	#[inline]
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		Ok(try!(reader.read_u64::<LittleEndian>()))
	}
}

impl Serializable for String {
	fn serialize(&self, stream: &mut Stream) {
		let bytes: &[u8] = self.as_ref();
		stream
			.append(&CompactInteger::from(bytes.len()))
			.append_slice(bytes);
	}
}

impl Deserializable for String {
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		let bytes: Bytes = try!(reader.read());
		Ok(String::from_utf8_lossy(&bytes).into_owned())
	}
}

macro_rules! impl_ser_for_hash {
	($name: ident, $size: expr) => {
		impl Serializable for $name {
			fn serialize(&self, stream: &mut Stream) {
				stream.append_slice(&**self);
			}
		}

		impl Deserializable for $name {
			fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
				let slice = try!(reader.read_slice($size));
				let mut result = Self::default();
				result.copy_from_slice(slice);
				Ok(result)
			}
		}
	}
}

impl_ser_for_hash!(H96, 12);
impl_ser_for_hash!(H160, 20);
impl_ser_for_hash!(H256, 32);
impl_ser_for_hash!(H264, 33);
impl_ser_for_hash!(H512, 64);
impl_ser_for_hash!(H520, 65);

impl Serializable for Bytes {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&CompactInteger::from(self.len()))
			.append_slice(&self);
	}
}

impl Deserializable for Bytes {
	fn deserialize(reader: &mut Reader) -> Result<Self, Error> where Self: Sized {
		let len = try!(reader.read::<CompactInteger>());
		reader.read_slice(len.into()).map(|b| b.to_vec().into())
	}
}

#[cfg(test)]
mod tests {
	use bytes::Bytes;
	use {serialize, deserialize, Stream, Reader, Error};

	#[test]
	fn test_reader_read() {
		let buffer = vec![
			1,
			2, 0,
			3, 0, 0, 0,
			4, 0, 0, 0, 0, 0, 0, 0
		];

		let mut reader = Reader::new(&buffer);
		assert!(!reader.is_finished());
		assert_eq!(1u8, reader.read().unwrap());
		assert_eq!(2u16, reader.read().unwrap());
		assert_eq!(3u32, reader.read().unwrap());
		assert_eq!(4u64, reader.read().unwrap());
		assert!(reader.is_finished());
		assert_eq!(Error::UnexpectedEnd, reader.read::<u8>().unwrap_err());
	}

	#[test]
	fn test_stream_append() {
		let mut stream = Stream::default();

		stream
			.append(&1u8)
			.append(&2u16)
			.append(&3u32)
			.append(&4u64);

		let expected = vec![
			1u8,
			2, 0,
			3, 0, 0, 0,
			4, 0, 0, 0, 0, 0, 0, 0,
		].into();

		assert_eq!(stream.out(), expected);
	}

	#[test]
	fn test_bytes_deserialize() {
		let raw: Bytes = "020145".into();
		let expected: Bytes = "0145".into();
		assert_eq!(expected, deserialize(&raw).unwrap());
	}

	#[test]
	fn test_bytes_serialize() {
		let expected: Bytes = "020145".into();
		let bytes: Bytes = "0145".into();
		assert_eq!(expected, serialize(&bytes));
	}

	#[test]
	fn test_string_serialize() {
		let expected: Bytes = "0776657273696f6e".into();
		let s: String = "version".into();
		assert_eq!(serialize(&s), expected);
		let expected: Bytes = "00".into();
		let s: String = "".into();
		assert_eq!(serialize(&s), expected);
	}

	#[test]
	fn test_string_deserialize() {
		let raw: Bytes = "0776657273696f6e".into();
		let expected: String = "version".into();
		assert_eq!(expected, deserialize::<String>(&raw).unwrap());
		let raw: Bytes = "00".into();
		let expected: String = "".into();
		assert_eq!(expected, deserialize::<String>(&raw).unwrap());
	}

	#[test]
	fn test_steam_append_slice() {
		let mut slice = [0u8; 4];
		slice[0] = 0x64;
		let mut stream = Stream::default();
		stream.append_slice(&slice);
		assert_eq!(stream.out(), "64000000".into());
	}
}