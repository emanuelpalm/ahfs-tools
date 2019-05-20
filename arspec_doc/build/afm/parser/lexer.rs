use arspec_parser::{Error, Matcher, Parser, Scanner, Text, Token};
use super::Class;

/// Create a slice of [`Tokens`][tok] from all characters accessible via given
/// [`scanner`][sca].
///
/// [sca]: ../../../arspec_parser/struct.Scanner.html
/// [tok]: ../../../arspec_parser/struct.Token.html
pub fn scan(mut scanner: Scanner) -> Vec<Token<Class>> {
    Vec::new()
}

fn scan_integer_or_number(scanner: &mut Scanner) -> Option<Class> {
    let mut is_number = false;
    let mut ch;

    // Integral.
    loop {
        ch = scanner.next()?;
        match ch {
            '0'...'9' => continue,
            _ => break,
        }
    }

    // Fraction.
    if ch == '.' {
        loop {
            ch = scanner.next()?;
            match ch {
                '0'...'9' => continue,
                _ => break,
            }
        }
        is_number = true;
    }

    scanner.unwind();

    Some(if is_number { Class::Number } else { Class::Integer })
}

fn scan_key_standard(scanner: &mut Scanner) -> Option<Class> {
    let mut is_invalid = false;
    let mut ch;
    loop {
        ch = scanner.next()?;
        match ch {
            '0'...'9' | 'A'...'Z' | 'a'...'z' => {}
            '\t' | '\n' | ' ' => {
                scanner.unwind();
                break;
            }
            _ => {
                is_invalid = true;
            }
        }
    }
    if is_invalid {
        return Some(Class::InvalidKey);
    }
    Some(match scanner.review() {
        "Comment" => Class::Comment,

        "StartFontMetrics" => Class::StartFontMetrics,
        "EndFontMetrics" => Class::EndFontMetrics,

        "FontName" => Class::FontName,
        "FullName" => Class::FullName,
        "FamilyName" => Class::FamilyName,
        "Weight" => Class::Weight,
        "FontBBox" => Class::FontBBox,
        "Version" => Class::Version,
        "Notice" => Class::Notice,
        "EncodingScheme" => Class::EncodingScheme,
        "MappingScheme" => Class::MappingScheme,
        "EscChar" => Class::EscChar,
        "CharacterSet" => Class::CharacterSet,
        "Characters" => Class::Characters,
        "IsBaseFont" => Class::IsBaseFont,
        "VVector" => Class::VVector,
        "IsFixedV" => Class::IsFixedV,
        "IsCIDFont" => Class::IsCIDFont,
        "CapHeight" => Class::CapHeight,
        "XHeight" => Class::XHeight,
        "Ascender" => Class::Ascender,
        "Descender" => Class::Descender,
        "StdHW" => Class::StdHW,
        "StdVW" => Class::StdVW,
        "WeightVector" => Class::WeightVector,

        "StartDirection" => Class::StartDirection,
        "EndDirection" => Class::EndDirection,
        "UnderlinePosition" => Class::UnderlinePosition,
        "UnderlineThickness" => Class::UnderlineThickness,
        "ItalicAngle" => Class::ItalicAngle,
        "CharWidth" => Class::CharWidth,
        "IsFixedPitch" => Class::IsFixedPitch,

        "StartCharMetrics" => Class::StartCharMetrics,
        "EndCharMetrics" => Class::EndCharMetrics,
        "C" => Class::C,
        "WX" => Class::WX,
        "N" => Class::N,
        "B" => Class::B,

        "StartKernData" => Class::StartKernData,
        "EndKernData" => Class::EndKernData,

        "StartKernPairs" => Class::StartKernPairs,
        "EndKernPairs" => Class::EndKernPairs,
        "KPX" => Class::KPX,

        _ => Class::InvalidKey,
    })
}

fn scan_name(scanner: &mut Scanner) -> Option<Class> {
    let mut class = Class::Name;
    let mut ch;
    loop {
        ch = scanner.next()?;
        match ch {
            '\t' | '\n' | ' ' => {
                scanner.unwind();
                break;
            }
            _ => {}
        }
        let b = ch as u8;
        if !ch.is_ascii() || (b < b' ') {
            class = Class::InvalidName;
        }
    }
    Some(class)
}

fn scan_string(scanner: &mut Scanner) -> Option<Class> {
    let mut class = Class::String;
    let mut ch;
    loop {
        ch = scanner.next()?;
        if ch == '\n' {
            break;
        }
        let b = ch as u8;
        if !ch.is_ascii() || (b < b' ' && b != b'\t') {
            class = Class::InvalidString;
        }
    }
    Some(class)
}