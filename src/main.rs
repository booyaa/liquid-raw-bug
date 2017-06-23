extern crate liquid;

fn main() {
    use liquid::Renderable;

    let template = liquid::parse(r"{% raw %}
{{ abc }}
{{ def }}
{{ ghi }}
{% endraw %}",
                                 Default::default())
        .unwrap();

    let output = template.render(&mut Default::default());
    assert_eq!(output.unwrap(),
               Some("\n{{ abc }}\n{{ def }}\n{{ ghi }}\n".to_string()));
}