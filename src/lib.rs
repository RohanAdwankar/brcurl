use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use clap::{ArgAction, Parser};
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};

#[derive(Debug, Parser)]
#[command(
    name = "brcurl",
    about = "Curl-like browser rendering for JS/wasm-heavy pages"
)]
pub struct Cli {
    /// URL to open.
    pub url: String,

    /// Seconds to wait after the initial load completes.
    #[arg(short = 't', long = "time", default_value_t = 0)]
    pub time: u64,

    /// Save a screenshot of the rendered page.
    #[arg(
        short = 'o',
        long = "output",
        num_args = 0..=1,
        default_missing_value = "__AUTO__",
        require_equals = true,
        value_name = "OUTPUT"
    )]
    pub output: Option<String>,

    /// Print rendered DOM instead of visible text.
    #[arg(long = "dom", action = ArgAction::SetTrue)]
    pub dom: bool,
}

pub fn run(cli: Cli) -> Result<()> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1600, 2200)))
        .port(Some(choose_debug_port()?))
        .build()
        .map_err(|err| anyhow!("failed to build browser launch options: {err}"))?;
    let browser = Browser::new(options).context(
        "failed to launch a Chromium-compatible browser; install Chrome/Chromium to use brcurl",
    )?;
    let tab = browser.new_tab().context("failed to create browser tab")?;

    tab.navigate_to(&cli.url)
        .with_context(|| format!("failed to navigate to {}", cli.url))?;
    tab.wait_until_navigated()
        .context("page navigation did not complete")?;
    tab.wait_for_element("body")
        .context("page body did not appear")?;

    if cli.time > 0 {
        thread::sleep(Duration::from_secs(cli.time));
    }

    if let Some(path) = output_path(&cli.url, cli.output) {
        save_screenshot(&tab, &path)?;
    }

    let rendered = if cli.dom {
        evaluate_string(
            &tab,
            "document.documentElement ? document.documentElement.outerHTML : ''",
        )?
    } else {
        extract_useful_text(&tab)?
    };

    println!("{rendered}");
    Ok(())
}

fn evaluate_string(tab: &Tab, expression: &str) -> Result<String> {
    let value = tab
        .evaluate(expression, true)
        .with_context(|| format!("failed to evaluate browser expression: {expression}"))?
        .value
        .ok_or_else(|| anyhow!("browser expression returned no value"))?;

    let Some(string) = value.as_str() else {
        bail!("browser expression did not return a string");
    };
    Ok(string.to_owned())
}

fn save_screenshot(tab: &Tab, path: &std::path::Path) -> Result<()> {
    let png = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
        .context("failed to capture screenshot")?;
    std::fs::write(path, png)
        .with_context(|| format!("failed to write screenshot to {}", path.display()))?;
    Ok(())
}

fn choose_debug_port() -> Result<u16> {
    let listener =
        TcpListener::bind(("127.0.0.1", 0)).context("failed to allocate a Chrome debug port")?;
    let port = listener
        .local_addr()
        .context("failed to read allocated Chrome debug port")?
        .port();
    drop(listener);
    Ok(port)
}

fn extract_useful_text(tab: &Tab) -> Result<String> {
    let text = evaluate_string(
        tab,
        r#"
        (() => {
            const normalize = (value) =>
                (value || '')
                    .replace(/\s+/g, ' ')
                    .trim();

            const isVisible = (element) => {
                if (!(element instanceof Element)) return false;
                const style = window.getComputedStyle(element);
                if (style.display === 'none' || style.visibility === 'hidden' || Number(style.opacity) === 0) {
                    return false;
                }
                const rect = element.getBoundingClientRect();
                return rect.width > 0 && rect.height > 0;
            };

            const seen = new Set();
            const lines = [];
            const push = (prefix, value) => {
                const text = normalize(value);
                if (!text) return;
                if (text.length < 3 || text.length > 220) return;
                if (!/[A-Za-z0-9]/.test(text)) return;
                const key = text.toLowerCase();
                if (seen.has(key)) return;
                seen.add(key);
                lines.push(prefix ? `${prefix}: ${text}` : text);
            };

            push('Title', document.title);

            for (const selector of [
                'meta[property="og:title"]',
                'meta[name="twitter:title"]',
                'meta[name="description"]',
                'meta[property="og:description"]'
            ]) {
                const element = document.querySelector(selector);
                if (element) {
                    push('Meta', element.getAttribute('content') || '');
                }
            }

            for (const selector of [
                'h1',
                'h2',
                'h3',
                '[role="heading"]',
                'main a[title]',
                'main a',
                'main button',
                'main [role="button"]',
                'main [aria-label]',
                'article a',
                'article button',
                'article [aria-label]',
                'img[alt]'
            ]) {
                for (const element of document.querySelectorAll(selector)) {
                    if (!isVisible(element)) continue;
                    push('', element.innerText || element.getAttribute('aria-label') || element.getAttribute('title') || element.getAttribute('alt') || '');
                    if (lines.length >= 80) {
                        return lines.join('\n');
                    }
                }
            }

            const bodyText = normalize(document.body ? document.body.innerText : '');
            if (lines.length < 8 && bodyText) {
                for (const line of bodyText.split('\n')) {
                    push('', line);
                    if (lines.length >= 80) {
                        break;
                    }
                }
            }

            return lines.join('\n');
        })()
        "#,
    )?;

    if text.is_empty() {
        evaluate_string(
            tab,
            "document.documentElement ? document.documentElement.outerHTML : ''",
        )
    } else {
        Ok(text)
    }
}

fn output_path(url: &str, output: Option<String>) -> Option<PathBuf> {
    match output {
        None => None,
        Some(path) if path == "__AUTO__" => Some(PathBuf::from(format!("{}.png", slugify(url)))),
        Some(path) => Some(PathBuf::from(path)),
    }
}

fn slugify(input: &str) -> String {
    let mut slug = input
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    slug.truncate(80);
    while slug.ends_with('_') {
        slug.pop();
    }
    if slug.is_empty() {
        "brcurl".to_owned()
    } else {
        slug
    }
}
