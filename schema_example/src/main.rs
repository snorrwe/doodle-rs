extern crate schema;

use schema::*;

#[derive(Schema)]
struct Foo {
    pub winnie: String,
    pub pooh: i32,
    pub bar: SomeCustomType,
    pub epic: *const SomeTrait,
}

struct SomeCustomType;
trait SomeTrait {}

fn main() {
    let fields = Foo::get_fields_openapi();
    println!("\"Foo\" Schema: \n{:#?}", fields);
}

