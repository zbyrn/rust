// compile-flags: -C opt-level=0 -C no-prepopulate-passes

// This test checks that arguments/returns in opt-level=0 builds,
// while lacking attributes used for optimization, still have ABI-affecting attributes.

#![crate_type = "lib"]
#![feature(rustc_attrs)]

pub struct S {
  _field: [i32; 8],
}

// CHECK: define zeroext i1 @boolean(i1 zeroext %x)
#[no_mangle]
pub fn boolean(x: bool) -> bool {
  x
}

// CHECK-LABEL: @boolean_call
#[no_mangle]
pub fn boolean_call(x: bool, f: fn(bool) -> bool) -> bool {
// CHECK: call zeroext i1 %f(i1 zeroext %x)
  f(x)
}

// CHECK: define align 4 i32* @borrow(i32* align 4 %x)
#[no_mangle]
pub fn borrow(x: &i32) -> &i32 {
  x
}

// CHECK-LABEL: @borrow_call
#[no_mangle]
pub fn borrow_call(x: &i32, f: fn(&i32) -> &i32) -> &i32 {
  // CHECK: call align 4 i32* %f(i32* align 4 %x)
  f(x)
}

// CHECK: define void @struct_(%S* sret(%S){{( %0)?}}, %S* %x)
#[no_mangle]
pub fn struct_(x: S) -> S {
  x
}

// CHECK-LABEL: @struct_call
#[no_mangle]
pub fn struct_call(x: S, f: fn(S) -> S) -> S {
  // CHECK: call void %f(%S* sret(%S){{( %0)?}}, %S* %{{.+}})
  f(x)
}

// CHECK: define { i8, i8 } @enum_(i1 zeroext %x.0, i8 %x.1)
#[no_mangle]
pub fn enum_(x: Option<u8>) -> Option<u8> {
  x
}

// CHECK-LABEL: @enum_call
#[no_mangle]
pub fn enum_call(x: Option<u8>, f: fn(Option<u8>) -> Option<u8>) -> Option<u8> {
  // CHECK: call { i8, i8 } %f(i1 zeroext %x.0, i8 %x.1)
  f(x)
}
