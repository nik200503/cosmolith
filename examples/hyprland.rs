use hyprland::keyword::Keyword;

fn main() -> hyprland::Result<()> {
    // 1. Enable Natural Scrolling on Touchpad
    // corresponds to `input:touchpad:natural_scroll` in hyprland.conf
    Keyword::set("input:touchpad:natural_scroll", "true")?;

    // 2. Set Mouse Sensitivity (Acceleration)
    // Range is usually -1.0 to 1.0
    Keyword::set("input:sensitivity", "0.5")?;

    // 3. Enable Left-Handed mode for the Mouse
    Keyword::set("input:left_handed", "true")?;

    println!("Input settings updated!");
    Ok(())
}
