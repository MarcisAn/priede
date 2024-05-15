use annotate_snippets::{Level, Renderer, Snippet};
use anstream;
use colored::Colorize;

pub fn parser_error(unexpected: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("NEATPAZĪTS SIMBOLS `{}`", unexpected);
    common_error(error_title.to_string(), line, path);
/*
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
    anstream::println!("{}", renderer.render(message)); */
}
pub fn math_error(msg: &str, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
    //println!("Kļūda! {}\nrindiņā {}\nfailā {}", error_title, line, path);
    /* 
     
    let correct_line_endings = source.replace("\r\n", "\n");
    //let correct_line_endings = source;
    let diff = source.len() - correct_line_endings.len();
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(&source)
            .line_start(line - diff/2)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));*/
}
pub fn incorect_init_value(msg: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
    
    //println!("Kļūda! {}\nrindiņā {}\nfailā {}", error_title, line, path);
    /* 
    let correct_line_endings = source.replace("\r\n", "\r\n");
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(&correct_line_endings)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));*/
}
pub fn undefined_var(msg: String, source: &str, path: &str, line: usize, col: usize) {
    let error_title = &format!("{}", msg);
    common_error(msg.to_string(), line, path);
    
    //println!("Kļūda! {}\n{}. rindiņā \nfailā {}", error_title, line, path);
    /* 
    let correct_line_endings = source.replace("\r\n", "\r");
    let message = Level::Error.title(&error_title).snippet(
        Snippet::source(&correct_line_endings)
            .line_start(line)
            .origin(path)
            .fold(true)
            .annotation(
                Level::Error
                    .span(col-1..col)
            )
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));*/
}
fn common_error(msg: String, line: usize, path: &str){
    print!("{}\n{}\nFaila \"{}\"\n{}. rindiņā", "Kļūda: ".red(), msg.red(), path, line);
}