use annotate_snippets::{Level, Renderer, Snippet};
use anstream;

pub fn parser_error(unexpected: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("NEATPAZĪTS SIMBOLS `{}`", unexpected);
    let label = &format!("ŠAJĀ KONTEKSTĀ NEIEDERĀS `{}`", unexpected);
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(source)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
                    .label(label)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}