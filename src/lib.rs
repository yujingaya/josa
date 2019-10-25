//! # Example
//!
//! ```
//! use josa::Josa::{EunNeun, IGa};
//!
//! let user = "유진".to_owned();
//! let mackerel = "고등어".to_owned();
//!
//! let sentence = format!("{} {} 먹고싶다", user + EunNeun, mackerel + IGa);
//!
//! assert_eq!(sentence, "유진은 고등어가 먹고싶다");
//! ```

use std::error::Error;
use std::fmt;
use std::ops::{Add, AddAssign};

use hangul::HangulExt;
use hangul::ParseSyllableError;

const EUN: &str = "은";
const NEUN: &str = "는";

const I: &str = "이";
const GA: &str = "가";

const EUL: &str = "을";
const REUL: &str = "를";

const GWA: &str = "과";
const WA: &str = "와";

/// Enum of [josas](https://en.wikipedia.org/wiki/Korean_grammar#Postpositions) that are selected depending on the noun in front of it
pub enum Josa {
  /// 은, 는
  EunNeun,
  /// 이, 가
  IGa,
  /// 을, 를
  EulReul,
  /// 과, 와
  GwaWa,
}

use Josa::{EunNeun, IGa, EulReul, GwaWa};

impl Josa {
  fn open(self) -> &'static str {
    match self {
      EunNeun => NEUN,
      IGa => GA,
      EulReul => REUL,
      GwaWa => WA,
    }
  }

  fn closed(self) -> &'static str {
    match self {
      EunNeun => EUN,
      IGa => I,
      EulReul => EUL,
      GwaWa => GWA,
    }
  }
}

#[derive(Debug)]
pub enum JosaError {
  EmptyStr,
  NotHangulSyllable(char),
}

use JosaError::{EmptyStr, NotHangulSyllable};

impl fmt::Display for JosaError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        EmptyStr => "Empty string given to josa selector".to_owned(),
        NotHangulSyllable(c) => format!("{} is not a Hangul Syllable", c),
      }
    )
  }
}

impl Error for JosaError {}

impl From<ParseSyllableError> for JosaError {
  fn from(e: ParseSyllableError) -> JosaError {
    NotHangulSyllable(e.0)
  }
}

/// Select appropriate josa for the noun.
///
/// It is useful when you are trying to append a josa to formatted text such as `<span>고양이</span>`.
/// If you try to append a josa to `<span>고양이</span>`, it results in [`JosaError`](enum.JosaError.html) because of the last character `>`.
/// With this method, you can first get an appropriate josa, and then format the text with the that:
/// ```rust
/// use josa::select;
/// use josa::Josa::IGa;
/// # use josa::JosaError;
///
/// let cat = "고양이";
/// let josa = select(cat, IGa)?;
///
/// let cat = format!(r#"<span class="bold">{}</span>{}"#, cat, josa);
///
/// assert_eq!(cat, r#"<span class="bold">고양이</span>가"#);
/// # Ok::<(), JosaError>(())
/// ```
///
/// # Errors
/// If the noun is an empty String or last character is not a Haugul Syllable, it returns [`JosaError`](enum.JosaError.html)
///
/// # Example
/// ```
/// use josa::select;
/// use josa::Josa::IGa;
/// # use josa::JosaError;
///
/// assert_eq!(select("고양이", IGa)?, "가");
/// # Ok::<(), JosaError>(())
/// ```
///
/// ```
/// use josa::select;
/// use josa::Josa::EunNeun;
/// # use josa::JosaError;
///
/// assert_eq!(select("사냥꾼", EunNeun)?, "은");
/// # Ok::<(), JosaError>(())
/// ```
pub fn select(noun: &str, josa: Josa) -> Result<&str, JosaError> {
  if noun.chars().last().ok_or(EmptyStr)?.is_open()? {
    Ok(josa.open())
  } else {
    Ok(josa.closed())
  }
}

pub trait JosaExt {
  fn push_josa(&mut self, josa: Josa);
}

impl JosaExt for String {
  // Document that it won't do anything if string is empty
  // Document that it assumes open syllable if it's not a Hangul Syllable
  fn push_josa(&mut self, josa: Josa) {
    self.push_str(match self.chars().last() {
      Some(last) => match last.is_open() {
        Ok(true) => josa.open(),
        Ok(false) => josa.closed(),
        Err(_) => josa.open(),
      },
      None => "",
    });
  }
}

impl Add<Josa> for String {
  type Output = String;

  fn add(mut self, josa: Josa) -> String {
    self.push_josa(josa);
    self
  }
}

impl AddAssign<Josa> for String {
  fn add_assign(&mut self, josa: Josa) {
    self.push_josa(josa);
  }
}
