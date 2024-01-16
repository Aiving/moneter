use std::env;
use std::fmt::Write;
use std::fmt;

use material_colors::argb_from_hex;
use material_colors::hex_from_argb;
use material_colors::theme_from_source_color;
use material_colors::Scheme;
use material_colors::Argb;

fn colors(scheme: Scheme) -> [(&'static str, Argb); 48] {
   [
        ("primary", scheme.primary),
        ("on_primary", scheme.on_primary),
        ("primary_container", scheme.primary_container),
        ("on_primary_container", scheme.on_primary_container),
        ("inverse_primary", scheme.inverse_primary),
        ("primary_fixed", scheme.primary_fixed),
        ("primary_fixed_dim", scheme.primary_fixed_dim),
        ("on_primary_fixed", scheme.on_primary_fixed),
        ("on_primary_fixed_variant", scheme.on_primary_fixed_variant),
        ("secondary", scheme.secondary),
        ("on_secondary", scheme.on_secondary),
        ("secondary_container", scheme.secondary_container),
        ("on_secondary_container", scheme.on_secondary_container),
        ("secondary_fixed", scheme.secondary_fixed),
        ("secondary_fixed_dim", scheme.secondary_fixed_dim),
        ("on_secondary_fixed", scheme.on_secondary_fixed),
        ("on_secondary_fixed_variant", scheme.on_secondary_fixed_variant),
        ("tertiary", scheme.tertiary),
        ("on_tertiary", scheme.on_tertiary),
        ("tertiary_container", scheme.tertiary_container),
        ("on_tertiary_container", scheme.on_tertiary_container),
        ("tertiary_fixed", scheme.tertiary_fixed),
        ("tertiary_fixed_dim", scheme.tertiary_fixed_dim),
        ("on_tertiary_fixed", scheme.on_tertiary_fixed),
        ("on_tertiary_fixed_variant", scheme.on_tertiary_fixed_variant),
        ("error", scheme.error),
        ("on_error", scheme.on_error),
        ("error_container", scheme.error_container),
        ("on_error_container", scheme.on_error_container),
        ("surface_dim", scheme.surface_dim),
        ("surface", scheme.surface),
        ("surface_bright", scheme.surface_bright),
        ("surface_container_lowest", scheme.surface_container_lowest),
        ("surface_container_low", scheme.surface_container_low),
        ("surface_container", scheme.surface_container),
        ("surface_container_high", scheme.surface_container_high),
        ("surface_container_highest", scheme.surface_container_highest),
        ("on_surface", scheme.on_surface),
        ("on_surface_variant", scheme.on_surface_variant),
        ("outline", scheme.outline),
        ("outline_variant", scheme.outline_variant),
        ("inverse_surface", scheme.inverse_surface),
        ("inverse_on_surface", scheme.inverse_on_surface),
        ("surface_variant", scheme.surface_variant),
        ("background", scheme.background),
        ("on_background", scheme.on_background),
        ("shadow", scheme.shadow),
        ("scrim", scheme.scrim),
    ]
}

fn main() -> fmt::Result {
    let args: Vec<String> = env::args().collect();

    let source_color = argb_from_hex(&args[1]).unwrap();
    let theme = theme_from_source_color(source_color, Default::default());
    let scheme = theme.schemes.dark;
    let colors = colors(scheme);
    let highest_color_length = colors.iter().max_by_key(|(name, _)| name.len()).unwrap().0.len();

    let mut data = String::new();

    for (key, [a, r, g, b]) in colors {
        let color = format!("#{}", hex_from_argb([a, r, g, b]));

        writeln!(data, "{key}: {color: >ident$} \x1b[48;2;{r};{g};{b}m  \x1b[0m", ident = highest_color_length + 10 - key.len())?;
    }

    println!("{data}");

    Ok(())
}
