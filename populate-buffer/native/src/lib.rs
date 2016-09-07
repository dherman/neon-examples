#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsBuffer;

fn create_buffer(call: Call) -> JsResult<JsBuffer> {
    let scope = call.scope;
    let mut buffer = try!(JsBuffer::new(scope, 16));
    buffer.grab(|mut contents| {
        let slice = contents.as_mut_slice();
        for i in 0..slice.len() {
            slice[i] = (i + 1) as u8;
        }
    });
    Ok(buffer)
}

register_module!(m, {
    m.export("createBuffer", create_buffer)
});
