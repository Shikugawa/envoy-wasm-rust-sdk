use std::collections::HashMap;
use std::mem::transmute;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub fn buffer_size(_pairs: &HashMap<String, String>) -> usize {
  let mut size = 4;
  size += 8 * _pairs.len(); // It is required to add the length of key and value.
  for (k, v) in _pairs {
    size += k.len();
    size += 1; // Null-termination symbol
    size += v.len();
    size += 1; // Null-termination symbol
  }
  size
}

pub fn buffer_into_hashmap(_buffer_ptr: *mut u8, _size: usize) -> HashMap<String, String> {
  let mut result: HashMap<String, String> = HashMap::new();
  unsafe {
    let buffer = Vec::from_raw_parts(_buffer_ptr, _size, _size);
    // read 4 bytes to get hashmap size illustrated as usize
    if buffer.len() < 4 {
      return result;
    }
    let pairs_size = transmute::<[u8; 4], u32>([
      buffer[0] as u8,
      buffer[1] as u8,
      buffer[2] as u8,
      buffer[3] as u8,
    ]);
    if pairs_size <= 0 {
      return result;
    }
    let mut byte_lengthes = Vec::<(u32, u32)>::new();
    let mut key_value_index_starter = 0;
    let mut key_size = 0;
    for i in 0..(2 * pairs_size) {
      if i % 2 == 0 {
        // read 4 bytes to get key size illustrated as usize
        key_size = transmute::<[u8; 4], u32>([
          buffer[4 + 4 * i as usize] as u8,
          buffer[5 + 4 * i as usize] as u8,
          buffer[6 + 4 * i as usize] as u8,
          buffer[7 + 4 * i as usize] as u8,
        ]);
      } else {
        // read 4 bytes to get value size illustrated as usize
        let value_size = transmute::<[u8; 4], u32>([
          buffer[4 + 4 * i as usize] as u8,
          buffer[5 + 4 * i as usize] as u8,
          buffer[6 + 4 * i as usize] as u8,
          buffer[7 + 4 * i as usize] as u8,
        ]);
        if key_size != 0 && value_size != 0 {
          byte_lengthes.push((key_size, value_size));
          key_size = 0;
        }
        key_value_index_starter = (7 + 4 * i) + 1;
      }
    }
    for (key_size, value_size) in byte_lengthes {
      let key_str = String::from_utf8(
        buffer[key_value_index_starter as usize..(key_value_index_starter + key_size) as usize]
          .to_vec(),
      )
      .unwrap();
      key_value_index_starter += key_size + 1;
      let value_str = String::from_utf8(
        buffer[key_value_index_starter as usize..(key_value_index_starter + value_size) as usize]
          .to_vec(),
      )
      .unwrap();
      key_value_index_starter += value_size + 1;
      result.insert(key_str, value_str);
    }
  }
  result
}

pub fn hashmap_into_buffer(_pairs: &HashMap<String, String>, _buffer: &mut Vec<c_char>) {
  let mut index = 0;
  // write length of pairs
  let pairs_len = _pairs.len() as u32;
  for b in pairs_len.to_le_bytes().iter() {
    _buffer[index] = *b as i8;
    index += 1;
  }
  // write length of keys and values
  for (key, value) in _pairs {
    let key_len = key.len();
    for b in key_len.to_le_bytes().iter() {
      _buffer[index] = *b as i8;
      index += 1;
    }
    let value_len = value.len();
    for b in value_len.to_le_bytes().iter() {
      _buffer[index] = *b as i8;
      index += 1;
    }
  }
  // write value of pairs
  for (key, value) in _pairs {
    for b in key.as_bytes().iter() {
      _buffer[index] = *b as i8;
      index += 1;
    }
    _buffer[index] = '\0' as i8;
    index += 1;
    for b in value.as_bytes().iter() {
      _buffer[index] = *b as i8;
      index += 1;
    }
    _buffer[index] = '\0' as i8;
    index += 1;
  }
}

pub fn export_hashmap(_pairs: &HashMap<String, String>) -> (*mut c_char, usize) {
  if _pairs.len() == 0 {
    let nullptr = null_mut::<c_char>();
    let zero: usize = 0;
    return (nullptr, zero);
  }
  let buffer_size = buffer_size(_pairs);
  let mut alloced_buffer = Vec::<c_char>::with_capacity(buffer_size);
  unsafe {
    alloced_buffer.set_len(buffer_size);
  }
  hashmap_into_buffer(_pairs, &mut alloced_buffer);
  (
    Box::into_raw(alloced_buffer.into_boxed_slice()) as *mut c_char,
    buffer_size,
  )
}
