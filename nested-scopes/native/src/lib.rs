#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsArray, JsNumber, Object};
use neon::scope::Scope;

fn make_a_number(call: Call) -> JsResult<JsInteger> {
    let x = JsInteger::new(call.scope, 1999);
    let y = call.scope.nested(|_| {
        17
    });
    println!("y = {}", y);
    Ok(x)
}

fn escape_example(call: Call) -> JsResult<JsArray> {
    let mut x = None;
    try!(call.scope.chained(|scope| {
        let array = JsArray::new(scope, 2);
        try!(array.set(0, JsInteger::new(scope, 42)));
        try!(array.set(1, JsNumber::new(scope, 6.28)));
        x = Some(scope.escape(array));
        Ok(())
    }));
    Ok(x.unwrap())
}


/*
// DOES NOT COMPILE: attempts to use an inactive scope
fn static_error_shadow(call: Call) -> JsResult<JsInteger> {
    let outer = call.scope;
    let mut x = JsInteger::new(outer, 0);
    let y = outer.nested(|_| {
        // error: outer is inactive!
        x = JsInteger::new(outer, 1999);
        17
    });
    println!("y = {}", y);
    Ok(x)
}
*/

/*
// DOES NOT COMPILE: attempts to escape a value out from a nested scope
use neon::js::JsUndefined;

fn static_error_escape(call: Call) -> JsResult<JsUndefined> {
    let _ = call.scope.nested(|scope| {
        JsInteger::new(scope, 17)
    });
    Ok(JsUndefined::new())
}
*/

register_module!(m, {
    try!(m.export("makeANumber", make_a_number));
    try!(m.export("escapeExample", escape_example));
    Ok(())
});
