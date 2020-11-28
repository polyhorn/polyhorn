use jni::objects::JValue;

use super::{Bitmap, Env, Object};

pub struct BitmapFactory;

impl BitmapFactory {
    pub fn decode_byte_array(env: &Env, bytes: &[u8]) -> Result<Bitmap, String> {
        unsafe {
            let reference = env.call_static_method(
                "android/graphics/BitmapFactory",
                "decodeByteArray",
                "([BII)Landroid/graphics/Bitmap;",
                &[
                    JValue::Object(env.byte_array(bytes)),
                    JValue::Int(0),
                    JValue::Int(bytes.len() as i32),
                ],
            );
            Ok(Bitmap::from_reference(
                env.retain(env.assume_object(reference)),
            ))
        }
    }
}
