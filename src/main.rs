extern crate liquid;

fn main() {
use liquid::{Renderable, Context, Value};

let template = liquid::parse(
r"{% raw %}
{{ abc }}
{{ def }}
{{ ghi }}
{% endraw %}", Default::default()).unwrap();

let mut context = Context::new();
context.set_val("num", Value::Num(4f32));

let output = template.render(&mut context);
assert_eq!(output.unwrap(), Some("\n{{ abc }}\n{{ def }}\n{{ ghi }}\n".to_string()));
}