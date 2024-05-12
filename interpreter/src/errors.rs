use annotate_snippets::{Level, Renderer, Snippet};
use anstream;

pub fn parser_error(unexpected: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("NEATPAZĪTS SIMBOLS `{}`", unexpected);
    let label = match &unexpected.as_str() {
        &"" => "ŠEIT SAGAIDĀMS SIMBOLS.".to_string(),
        _ => format!("ŠAJĀ KONTEKSTĀ NEIEDERĀS `{}`", "a")
    };
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(source)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col-1)
                    .label(&label)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
pub fn math_error(msg: &str, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(&source)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
pub fn incorect_init_value(msg: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(&source)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}