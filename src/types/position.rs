use bytes::{Buf, BufMut, BytesMut};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn to_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }
}


#[derive(Debug, Serialize)]
pub struct BlockLocation {
    x: i32,
    y: i32,
    z: i32
}

impl BlockLocation {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn to_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_z(&self) -> i32 {
        self.z
    }


    pub fn decode(buf: &mut BytesMut) -> Self {
        let val = buf.get_u64();

        let x = (val >> 38) as i32;  // Extract the top 26 bits for x
        let y = (val << 52 >> 52) as i32;  // Extract the bottom 12 bits for y (with sign extension)
        let z = (val << 26 >> 38) as i32;  // Extract the middle 26 bits for z
    
        let x = if x >= (1 << 25) { x - (1 << 26) } else { x };
        let z = if z >= (1 << 25) { z - (1 << 26) } else { z };

        Self { x: x, y: y, z: z }
    }

    pub fn encode(&self, buf: &mut BytesMut) {
        let x = (self.x & 0x3FFFFFF) as u64;  // Mask to 26 bits for x
        let y = (self.y & 0xFFF) as u64;      // Mask to 12 bits for y
        let z = (self.z & 0x3FFFFFF) as u64;  // Mask to 26 bits for z
    
        let position = (x << 38) | (z << 12) | y;

        let mut slice = BytesMut::with_capacity(8);
        slice.put_u64(position);

        buf.put_slice(&slice);
    }
}