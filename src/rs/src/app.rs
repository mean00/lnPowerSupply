#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use cty::c_char;
use rnarduino as rn;
/**
 * \fn rnLoop
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
     }

//Box::into_raw(r) as  * mut  cty::c_void); //Box::into_raw(r) as * mut  cty::c_void);
//Box::into_raw(boxed) as  *const   cty::c_void); //Box::into_raw(r) as * mut  cty::c_void);

#[no_mangle]
pub extern "C" fn rnInit() -> ()
{
   
}
// EOF
