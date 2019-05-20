use std::fmt;

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// [span]: ../../../arspec_parser/struct.Span.html
/// [token]: ../../../arspec_parser/struct.Token.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
    // Delimiters.
    Newline,
    Semicolon,

    // Literals.
    Integer,
    Array,
    Boolean,
    Name,
    Number,
    String,

    // Key: Comment.
    Comment,

    // Keys: AFM File Structure.
    StartFontMetrics,
    EndFontMetrics,

    // Keys: Global Font Information.
    FontName,
    FullName,
    FamilyName,
    Weight,
    FontBBox,
    Version,
    Notice,
    EncodingScheme,
    MappingScheme,
    EscChar,
    CharacterSet,
    Characters,
    IsBaseFont,
    VVector,
    IsFixedV,
    IsCIDFont,
    CapHeight,
    XHeight,
    Ascender,
    Descender,
    StdHW,
    StdVW,
    WeightVector,

    // Keys: Writing Direction Information.
    StartDirection,
    EndDirection,
    UnderlinePosition,
    UnderlineThickness,
    ItalicAngle,
    CharWidth,
    IsFixedPitch,

    // Keys: Individual Character Metrics.
    StartCharMetrics,
    EndCharMetrics,
    C,
    WX,
    N,
    B,

    // Keys: Kerning Data.
    StartKernData,
    EndKernData,
    StartKernPairs,
    EndKernPairs,
    KPX,


    // Errors.
    InvalidKey,
    InvalidName,
    InvalidString,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Class::Newline => "Newline",
            Class::Semicolon => ";",

            Class::Integer => "Integer",
            Class::Array => "Array",
            Class::Boolean => "Boolean",
            Class::Name => "Name",
            Class::Number => "Number",
            Class::String => "String",

            Class::Comment => "Comment",

            Class::StartFontMetrics => "StartFontMetrics",
            Class::EndFontMetrics => "EndFontMetrics",

            Class::FontName => "FontName",
            Class::FullName => "FullName",
            Class::FamilyName => "FamilyName",
            Class::Weight => "Weight",
            Class::FontBBox => "FontBBox",
            Class::Version => "Version",
            Class::Notice => "Notice",
            Class::EncodingScheme => "EncodingScheme",
            Class::MappingScheme => "MappingScheme",
            Class::EscChar => "EscChar",
            Class::CharacterSet => "CharacterSet",
            Class::Characters => "Characters",
            Class::IsBaseFont => "IsBaseFont",
            Class::VVector => "VVector",
            Class::IsFixedV => "IsFixedV",
            Class::IsCIDFont => "IsCIDFont",
            Class::CapHeight => "CapHeight",
            Class::XHeight => "XHeight",
            Class::Ascender => "Ascender",
            Class::Descender => "Descender",
            Class::StdHW => "StdHW",
            Class::StdVW => "StdVW",
            Class::WeightVector => "WeightVector",

            Class::StartDirection => "StartDirection",
            Class::EndDirection => "EndDirection",
            Class::UnderlinePosition => "UnderlinePosition",
            Class::UnderlineThickness => "UnderlineThickness",
            Class::ItalicAngle => "ItalicAngle",
            Class::CharWidth => "CharWidth",
            Class::IsFixedPitch => "IsFixedPitch",

            Class::StartCharMetrics => "StartCharMetrics",
            Class::EndCharMetrics => "EndCharMetrics",
            Class::C => "C",
            Class::WX => "WX",
            Class::N => "N",
            Class::B => "B",

            Class::StartKernData => "StartKernData",
            Class::EndKernData => "EndKernData",
            Class::StartKernPairs => "StartKernPairs",
            Class::EndKernPairs => "EndKernPairs",
            Class::KPX => "KPX",

            Class::InvalidKey => "<InvalidKey>",
            Class::InvalidName => "<InvalidName>",
            Class::InvalidString => "<InvalidString>",
        })
    }
}