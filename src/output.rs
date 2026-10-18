use std::{
    io::{self, IsTerminal},
    sync::atomic::{AtomicBool, Ordering},
};

use console::{Term, style};

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

pub fn error(message: &str) {
    eprintln!("{} {}", style("Error:").red().bold(), clean(message));
}
pub fn invoice_qr(invoice: &str) {
    if !interactive() {
        return;
    }
    // Uppercase BOLT11 uses QR alphanumeric encoding, keeping the symbol compact.
    let Ok(code) = qrcode::QrCode::new(invoice.to_ascii_uppercase()) else {
        eprintln!("Could not display the invoice QR code. Use the invoice text above.");
        return;
    };
    if code.width() + 8 >= usize::from(Term::stdout().size().1) {
        eprintln!(
            "Widen your terminal to at least {} columns to display the invoice QR code. Use the invoice text above.",
            code.width() + 9
        );
        return;
    }
    let qr = code
        .render::<qrcode::render::unicode::Dense1x2>()
        .dark_color(qrcode::render::unicode::Dense1x2::Light)
        .light_color(qrcode::render::unicode::Dense1x2::Dark)
        .quiet_zone(true)
        .build();
    println!("\nScan to pay:\n");
    for line in qr.lines() {
        // Explicit colours keep the code scannable on both light and dark themes.
        println!("{}", style(line).black().on_white().force_styling(true));
    }
    println!();
}

pub fn clean(value: &str) -> String {
    console::strip_ansi_codes(value)
        .chars()
        .filter(|c| !c.is_control() || matches!(c, '\n' | '\t'))
        .collect()
}
