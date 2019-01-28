extern crate doodle;
extern crate serde_json;

use doodle::Schema;

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
    let output = serde_json::to_string_pretty(&fields).unwrap();
    println!("\"Foo\" Schema: \n{}", output);
}

