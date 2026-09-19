use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    Persian,
}

impl Language {
    pub fn toggle(&mut self) {
        *self = match self {
            Language::English => Language::Persian,
            Language::Persian => Language::English,
        };
    }

    pub fn is_persian(&self) -> bool {
        matches!(self, Language::Persian)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Persian => "فارسی",
        }
    }
}

// Letter metadata: (isolated, final, initial, medial, connects_left)
struct LetterForm {
    iso: char,
    fin: char,
    ini: char,
    med: char,
    connects_left: bool,
}

fn get_letter_form(c: char) -> Option<LetterForm> {
    match c {
        // Alif with madda / hamza / normal
        'آ' => Some(LetterForm { iso: '\u{FE81}', fin: '\u{FE82}', ini: '\u{FE81}', med: '\u{FE82}', connects_left: false }),
        'ا' | 'أ' | 'إ' | 'ٱ' => Some(LetterForm { iso: '\u{FE8D}', fin: '\u{FE8E}', ini: '\u{FE8D}', med: '\u{FE8E}', connects_left: false }),
        'ب' => Some(LetterForm { iso: '\u{FE8F}', fin: '\u{FE90}', ini: '\u{FE91}', med: '\u{FE92}', connects_left: true }),
        'پ' => Some(LetterForm { iso: '\u{FB56}', fin: '\u{FB57}', ini: '\u{FB58}', med: '\u{FB59}', connects_left: true }),
        'ت' => Some(LetterForm { iso: '\u{FE95}', fin: '\u{FE96}', ini: '\u{FE97}', med: '\u{FE98}', connects_left: true }),
        'ث' => Some(LetterForm { iso: '\u{FE99}', fin: '\u{FE9A}', ini: '\u{FE9B}', med: '\u{FE9C}', connects_left: true }),
        'ج' => Some(LetterForm { iso: '\u{FE9D}', fin: '\u{FE9E}', ini: '\u{FE9F}', med: '\u{FEA0}', connects_left: true }),
        'چ' => Some(LetterForm { iso: '\u{FB7A}', fin: '\u{FB7B}', ini: '\u{FB7C}', med: '\u{FB7D}', connects_left: true }),
        'ح' => Some(LetterForm { iso: '\u{FEA1}', fin: '\u{FEA2}', ini: '\u{FEA3}', med: '\u{FEA4}', connects_left: true }),
        'خ' => Some(LetterForm { iso: '\u{FEA5}', fin: '\u{FEA6}', ini: '\u{FEA7}', med: '\u{FEA8}', connects_left: true }),
        'د' => Some(LetterForm { iso: '\u{FEA9}', fin: '\u{FEAA}', ini: '\u{FEA9}', med: '\u{FEAA}', connects_left: false }),
        'ذ' => Some(LetterForm { iso: '\u{FEAB}', fin: '\u{FEAC}', ini: '\u{FEAB}', med: '\u{FEAC}', connects_left: false }),
        'ر' => Some(LetterForm { iso: '\u{FEAD}', fin: '\u{FEAE}', ini: '\u{FEAD}', med: '\u{FEAE}', connects_left: false }),
        'ز' => Some(LetterForm { iso: '\u{FEAF}', fin: '\u{FEB0}', ini: '\u{FEAF}', med: '\u{FEB0}', connects_left: false }),
        'ژ' => Some(LetterForm { iso: '\u{FB8A}', fin: '\u{FB8B}', ini: '\u{FB8A}', med: '\u{FB8B}', connects_left: false }),
        'س' => Some(LetterForm { iso: '\u{FEB1}', fin: '\u{FEB2}', ini: '\u{FEB3}', med: '\u{FEB4}', connects_left: true }),
        'ش' => Some(LetterForm { iso: '\u{FEB5}', fin: '\u{FEB6}', ini: '\u{FEB7}', med: '\u{FEB8}', connects_left: true }),
        'ص' => Some(LetterForm { iso: '\u{FEB9}', fin: '\u{FEBA}', ini: '\u{FEBB}', med: '\u{FEBC}', connects_left: true }),
        'ض' => Some(LetterForm { iso: '\u{FEBD}', fin: '\u{FEBE}', ini: '\u{FEBF}', med: '\u{FEC0}', connects_left: true }),
        'ط' => Some(LetterForm { iso: '\u{FEC1}', fin: '\u{FEC2}', ini: '\u{FEC3}', med: '\u{FEC4}', connects_left: true }),
        'ظ' => Some(LetterForm { iso: '\u{FEC5}', fin: '\u{FEC6}', ini: '\u{FEC7}', med: '\u{FEC8}', connects_left: true }),
        'ع' => Some(LetterForm { iso: '\u{FEC9}', fin: '\u{FECA}', ini: '\u{FECB}', med: '\u{FECC}', connects_left: true }),
        'غ' => Some(LetterForm { iso: '\u{FECD}', fin: '\u{FECE}', ini: '\u{FECF}', med: '\u{FED0}', connects_left: true }),
        'ف' => Some(LetterForm { iso: '\u{FED1}', fin: '\u{FED2}', ini: '\u{FED3}', med: '\u{FED4}', connects_left: true }),
        'ق' => Some(LetterForm { iso: '\u{FED5}', fin: '\u{FED6}', ini: '\u{FED7}', med: '\u{FED8}', connects_left: true }),
        'ک' | 'ك' => Some(LetterForm { iso: '\u{FB8E}', fin: '\u{FB8F}', ini: '\u{FB90}', med: '\u{FB91}', connects_left: true }),
        'گ' => Some(LetterForm { iso: '\u{FB92}', fin: '\u{FB93}', ini: '\u{FB94}', med: '\u{FB95}', connects_left: true }),
        'ل' => Some(LetterForm { iso: '\u{FEDD}', fin: '\u{FEDE}', ini: '\u{FEDF}', med: '\u{FEE0}', connects_left: true }),
        'م' => Some(LetterForm { iso: '\u{FEE1}', fin: '\u{FEE2}', ini: '\u{FEE3}', med: '\u{FEE4}', connects_left: true }),
        'ن' => Some(LetterForm { iso: '\u{FEE5}', fin: '\u{FEE6}', ini: '\u{FEE7}', med: '\u{FEE8}', connects_left: true }),
        'و' | 'ؤ' => Some(LetterForm { iso: '\u{FEED}', fin: '\u{FEEE}', ini: '\u{FEED}', med: '\u{FEEE}', connects_left: false }),
        'ه' | 'ة' => Some(LetterForm { iso: '\u{FEE9}', fin: '\u{FEEA}', ini: '\u{FEEB}', med: '\u{FEEC}', connects_left: true }),
        'ی' | 'ي' | 'ئ' | 'ى' => Some(LetterForm { iso: '\u{FBFC}', fin: '\u{FBFD}', ini: '\u{FBFE}', med: '\u{FBFF}', connects_left: true }),
        'ء' => Some(LetterForm { iso: '\u{FE80}', fin: '\u{FE80}', ini: '\u{FE80}', med: '\u{FE80}', connects_left: false }),
        _ => None,
    }
}

fn is_persian_letter(c: char) -> bool {
    get_letter_form(c).is_some() || c == '‌' // ZWNJ
}

fn is_digit_char(c: char) -> bool {
    matches!(c, '0'..='9' | '۰'..='۹' | '٠'..='٩' | '.' | '%')
}

fn is_latin_or_symbol(c: char) -> bool {
    !is_persian_letter(c)
        && !is_digit_char(c)
        && !c.is_whitespace()
        && !matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '«' | '»')
}

/// Shapes a single Persian word connecting adjacent letters
pub fn shape_persian_word(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let mut shaped = Vec::with_capacity(chars.len());
    let mut prev_connects_left = false;

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        // Lam-Alif ligature check
        if c == 'ل' && i + 1 < chars.len() {
            let next_c = chars[i + 1];
            let la_form = match next_c {
                'آ' => if prev_connects_left { Some('\u{FEF6}') } else { Some('\u{FEF5}') },
                'أ' => if prev_connects_left { Some('\u{FEF8}') } else { Some('\u{FEF7}') },
                'إ' => if prev_connects_left { Some('\u{FEFA}') } else { Some('\u{FEF9}') },
                'ا' => if prev_connects_left { Some('\u{FEFC}') } else { Some('\u{FEFB}') },
                _ => None,
            };
            if let Some(la_char) = la_form {
                shaped.push(la_char);
                prev_connects_left = false;
                i += 2;
                continue;
            }
        }

        if let Some(form) = get_letter_form(c) {
            let next_connects = if i + 1 < chars.len() {
                get_letter_form(chars[i + 1]).is_some()
            } else {
                false
            };

            let rendered = match (prev_connects_left, next_connects) {
                (true, true) => if form.connects_left { form.med } else { form.fin },
                (true, false) => form.fin,
                (false, true) => if form.connects_left { form.ini } else { form.iso },
                (false, false) => form.iso,
            };

            shaped.push(rendered);
            prev_connects_left = form.connects_left;
        } else {
            shaped.push(c);
            prev_connects_left = false;
        }
        i += 1;
    }

    shaped.into_iter().collect()
}

#[derive(Debug, Clone)]
enum Token {
    PersianWord(String),
    Number(String),
    LatinBlock(String),
    Whitespace(String),
    Punctuation(char),
}

/// Shapes and visually reverses Persian text with true BiDi token handling for LTR canvas renderers (egui)
pub fn shape_text(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // 1. Whitespace
        if c.is_whitespace() {
            let mut ws = String::new();
            while i < chars.len() && chars[i].is_whitespace() {
                ws.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::Whitespace(ws));
            continue;
        }

        // 2. Parenthesized English block check e.g. "(Motion Blur)" or "(BiRefNet / MediaPipe)"
        if c == '(' {
            // Peek ahead to see if it's an English/Latin parenthesized phrase
            let mut j = i + 1;
            let mut is_latin_block = true;
            let mut has_close = false;
            while j < chars.len() && chars[j] != ')' {
                if is_persian_letter(chars[j]) {
                    is_latin_block = false;
                    break;
                }
                j += 1;
            }
            if j < chars.len() && chars[j] == ')' && is_latin_block && j > i + 1 {
                has_close = true;
            }

            if has_close && is_latin_block {
                let block: String = chars[i..=j].iter().collect();
                tokens.push(Token::LatinBlock(block));
                i = j + 1;
                continue;
            }
        }

        // 3. Persian Word
        if is_persian_letter(c) {
            let mut word = String::new();
            while i < chars.len() && is_persian_letter(chars[i]) {
                word.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::PersianWord(word));
            continue;
        }

        // 4. Number (Persian or English digits)
        if is_digit_char(c) {
            let mut num = String::new();
            while i < chars.len() && (is_digit_char(chars[i]) || chars[i] == '/') {
                num.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::Number(num));
            continue;
        }

        // 5. Latin word/symbol
        if is_latin_or_symbol(c) {
            let mut latin = String::new();
            while i < chars.len() && (is_latin_or_symbol(chars[i]) || chars[i] == '/' || chars[i] == '-') {
                latin.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::LatinBlock(latin));
            continue;
        }

        // 6. Punctuation & Brackets
        tokens.push(Token::Punctuation(c));
        i += 1;
    }

    let has_persian = tokens.iter().any(|t| matches!(t, Token::PersianWord(_)));
    if !has_persian {
        return text.to_string();
    }

    let mut result = String::new();
    for token in tokens.into_iter().rev() {
        match token {
            Token::PersianWord(w) => {
                let shaped = shape_persian_word(&w);
                let rev: String = shaped.chars().rev().collect();
                result.push_str(&rev);
            }
            Token::Number(n) => {
                // Numbers are read Left-to-Right, so don't reverse digits
                result.push_str(&n);
            }
            Token::LatinBlock(l) => {
                // Latin phrases are read Left-to-Right, so don't reverse
                result.push_str(&l);
            }
            Token::Whitespace(ws) => {
                result.push_str(&ws);
            }
            Token::Punctuation(p) => {
                // Flip parentheses in RTL flow
                let flipped = match p {
                    '(' => ')',
                    ')' => '(',
                    '[' => ']',
                    ']' => '[',
                    '{' => '}',
                    '}' => '{',
                    '<' => '>',
                    '>' => '<',
                    '«' => '»',
                    '»' => '«',
                    other => other,
                };
                result.push(flipped);
            }
        }
    }

    result
}

/// Helper that translates a text based on current active language
pub fn tr(en: &'static str, fa: &'static str, lang: Language) -> String {
    match lang {
        Language::English => en.to_string(),
        Language::Persian => shape_text(fa),
    }
}
