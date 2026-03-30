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
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Print rendered DOM instead of visible text.
    #[arg(long = "dom", action = ArgAction::SetTrue)]
    pub dom: bool,
}

pub fn run(cli: Cli) -> Result<()> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .build()
        .map_err(|err| anyhow!("failed to build browser launch options: {err}"))?;
    let browser = Browser::new(options)
        .context("failed to launch a Chromium-compatible browser; install Chrome/Chromium to use brcurl")?;
    let tab = browser.new_tab().context("failed to create browser tab")?;

    tab.navigate_to(&cli.url)
        .with_context(|| format!("failed to navigate to {}", cli.url))?;
    tab.wait_until_navigated()
        .context("page navigation did not complete")?;

    if cli.time > 0 {
        thread::sleep(Duration::from_secs(cli.time));
    }

    if let Some(path) = cli.output.as_deref() {
        save_screenshot(&tab, path)?;
    }

    let rendered = if cli.dom {
        evaluate_string(
            &tab,
            "document.documentElement ? document.documentElement.outerHTML : ''",
        )?
    } else {
        let text = evaluate_string(
            &tab,
            r#"
            (() => {
                const body = document.body;
                if (!body) return '';
                const text = body.innerText || '';
                return text.trim();
            })()
            "#,
        )?;

        if text.is_empty() {
            evaluate_string(
                &tab,
                "document.documentElement ? document.documentElement.outerHTML : ''",
            )?
        } else {
            text
        }
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
