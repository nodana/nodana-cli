use std::{
    io::{self, IsTerminal},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};
use console::{Term, style};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

static PLAIN: AtomicBool = AtomicBool::new(false);

pub fn configure(plain: bool) {
    PLAIN.store(plain, Ordering::Relaxed);
    if plain || !io::stdout().is_terminal() {
        console::set_colors_enabled(false);
    }
    if plain || !io::stderr().is_terminal() {
        console::set_colors_enabled_stderr(false);
    }
}

pub fn interactive() -> bool {
    io::stdout().is_terminal()
        && !PLAIN.load(Ordering::Relaxed)
        && std::env::var("TERM").as_deref() != Ok("dumb")
}

pub fn clear() {
    if interactive() {
        let _ = Term::stdout().clear_screen();
    }
}

pub fn heading(title: &str) {
    if interactive() {
        println!(
            "\n  {}  {}\n",
            style("NODANA").cyan().bold(),
            style(title).bold()
        );
    } else {
        println!("{title}");
    }
}

pub fn success(message: &str) {
    println!("{} {message}", style("✓").green().bold());
}
pub fn error(message: &str) {
    eprintln!("{} {message}", style("Error:").red().bold());
}
pub fn field(label: &str, value: &str) {
    println!("  {:<20} {}", style(label).dim(), clean(value));
}

pub fn status_field(value: &str) {
    println!("  {:<20} {}", style("Status").dim(), status(value));
}

pub fn status(value: &str) -> String {
    let value = clean(value);
    if !interactive() {
        return value;
    }
    match value.as_str() {
        "running" | "ready" => style(value).green().to_string(),
        "failed" => style(value).red().to_string(),
        "provisioning" | "starting" | "updating" => style(value).yellow().to_string(),
        _ => style(value).dim().to_string(),
    }
}

pub fn nodes(nodes: &[crate::models::Node]) {
    if !interactive() {
        println!("ID\tNAME\tSTATUS");
        for node in nodes {
            println!(
                "{}\t{}\t{}",
                clean(&node.id),
                clean(&node.name),
                clean(&node.status)
            );
        }
        return;
    }
    heading(&format!("Nodes · {}", nodes.len()));
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["ID", "NAME", "STATUS"]);
    for node in nodes {
        table.add_row([clean(&node.id), clean(&node.name), status(&node.status)]);
    }
    println!("{table}");
}

pub fn clean(value: &str) -> String {
    console::strip_ansi_codes(value)
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}

pub struct Spinner(ProgressBar);
impl Spinner {
    pub fn new(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        if !interactive() || !io::stderr().is_terminal() {
            bar.set_draw_target(ProgressDrawTarget::hidden());
        }
        if let Ok(style) = ProgressStyle::with_template("  {spinner:.cyan} {msg}") {
            bar.set_style(style);
        }
        bar.set_message(message.to_owned());
        bar.enable_steady_tick(Duration::from_millis(80));
        Self(bar)
    }
}
impl Drop for Spinner {
    fn drop(&mut self) {
        self.0.finish_and_clear();
    }
}
