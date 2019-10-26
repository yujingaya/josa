//! Josa helps you append [josa] to a string in idiomatic way.
//! 
//! It has three approaches:
//! 
//! - [`push_josa`] method on [`String`]
//! - `+`, `+=` operator overloading
//! - A pure function [`select`] that selects appropriate josa
//! 
//! # Example: [`push_josa`] method
//!
//! [`push_josa`] method works the same way as [`push_str`].
//! It appends a given [`Josa`] onto the end of this [`String`].
//! Note that it does mutate the string,
//! so you need to declare the [`String`] as mutable.
//! 
//! ```
//! use josa::JosaExt;
//! use josa::{EulReul, EunNeun};
//!
//! let mut user = "나".to_owned();
//! let mut you = "님".to_owned();
//!
//! user.push_josa(EulReul);
//! you.push_josa(EunNeun);
//! 
//! assert_eq!(
//!   format!("{} 버리고 가시는 {}", user, you),
//!   "나를 버리고 가시는 님은"
//! );
//! ```
//! 
//! # Example: `+`, `+=` operator overloading
//! 
//! `+`, `+=` concatenates [`String`] with appropriate [`Josa`].
//! 
//! `+` consumes the [`String`] on the left-hand.
//! This is done to avoid allocating a new [`String`] and copying the entire contents.
//! 
//! ```
//! use josa::{EunNeun, IGa};
//!
//! let user = "유진".to_owned();
//! let mackerel = "고등어".to_owned();
//!
//! assert_eq!(
//!   format!("{} {} 먹고싶다", user + EunNeun, mackerel + IGa),
//!   "유진은 고등어가 먹고싶다"
//! );
//! ```
//! 
//! # Example: A pure function that selects appropriate josa
//! 
//! Sometimes we need to append a josa to formatted text like
//! `<span class="bold">곡괭이</span>`.
//! In that case, last character can be part of tag, which is not a Hangul Syllable.
//! [`select`] is used to get only josa, instead of appending it.
//! 
//! ```
//! use josa::select;
//! use josa::Eu;
//! # use josa::Error;
//!
//! let pick = "곡괭이";
//!
//! assert_eq!(
//!   format!(
//!     r#"<span class="bold">{}</span>{}로 채취하세요."#,
//!     pick,
//!     select(pick, Eu)?
//!   ),
//!   r#"<span class="bold">곡괭이</span>로 채취하세요."#
//! );
//!
//! let hand = "손";
//!
//! assert_eq!(
//!   format!(
//!     r#"<span class="bold">{}</span>{}로 채취하세요."#,
//!     hand,
//!     select(hand, Eu)?
//!   ),
//!   r#"<span class="bold">손</span>으로 채취하세요."#
//! );
//!
//! # Ok::<(), Error>(())
//! ```
//!
//! # Edge cases
//!
//! For [`push_josa`], `+`, and `+=`, since they are infallible,
//! they handle edge cases in their own way.
//! 
//! ### Empty String
//!
//! If given [`String`] is empty, it does not push any josa.
//! 
//! ```
//! use josa::{JosaExt, IGa};
//! 
//! let mut empty = "".to_owned();
//! empty.push_josa(IGa);
//! 
//! assert_eq!(empty, "");
//! ```
//! 
//! ### Non Hangul Syllable character
//! 
//! If given [`String`] ends with character other than Hangul Syllable,
//! it pushes `이(가)` formatted josa.
//! 
//! ```
//! use josa::{JosaExt, IGa, Eu};
//! 
//! let mut curry = "curry".to_owned();
//! curry.push_josa(IGa);
//! 
//! assert_eq!(curry, "curry이(가)");
//!
//!
//! let mut pioneer = "pioneer".to_owned();
//! pioneer.push_josa(Eu);
//! 
//! assert_eq!(pioneer, "pioneer(으)"); // you can append 로서
//! ```
//! 
//! # Supported josas
//!
//! Currently we support:
//!
//! - 은/는
//! - 이/가
//! - 을/를
//! - 과/와
//! - 이/(empty) (이다/다, 이나/나, 이란/란, 이든가/든가, 이나마/나마, 이야말로/야말로, 이랑/랑, 이여/여, 이며/며)
//! - 으/(empty) (으로/로, 으로서/로서, 으로써/로써, 으로부터/로부터)
//! 
//! [josa]: https://en.wikipedia.org/wiki/Korean_postpositions
//! [`push_josa`]: trait.JosaExt.html#tymethod.push_josa
//! [`push_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
//! [`select`]: fn.select.html
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`Josa`]: enum.Josa.html

use std::convert::TryFrom;
use std::ops::{Add, AddAssign};

use hangul::HangulExt;

mod error;
pub use error::{Error, Result};

pub use Josa::*;

// First group
const EUN: &str = "은";
const NEUN: &str = "는";

const I: &str = "이";
const GA: &str = "가";

const EUL: &str = "을";
const REUL: &str = "를";

const GWA: &str = "과";
const WA: &str = "와";

// Second group
const EU: &str = "으";


enum JongseongKind {
  Open,
  Rieul,
  Closed
}

impl TryFrom<char> for JongseongKind {
  type Error = Error;

  fn try_from(value: char) -> Result<JongseongKind> {
    match value.jongseong()? {
      Some(jongseong) => match jongseong {
        'ㄹ' => Ok(JongseongKind::Rieul),
        _ => Ok(JongseongKind::Closed)
      },
      None => Ok(JongseongKind::Open)
    }
  }
}


/// Enum of [josas](https://en.wikipedia.org/wiki/Korean_grammar#Postpositions) that are selected depending on the string in front of it.
#[derive(Clone, Copy)]
pub enum Josa {
  /// 은/는 
  EunNeun,
  /// 이/가
  IGa,
  /// 을/를
  EulReul,
  /// 과/와
  GwaWa,
  /// 이다/다, 이나/나, 이란/란, 이든가/든가, 이나마/나마, 이야말로/야말로, 이랑/랑, 이여/여, 이며/며
  I,
  /// 으로/로, 으로서/로서, 으로써/로써, 으로부터/로부터
  Eu
}

impl Josa {
  fn select(self, c: char) -> Result<&'static str> {
    match JongseongKind::try_from(c)? {
      JongseongKind::Open => Ok(self.open()),
      JongseongKind::Rieul => Ok(self.rieul()),
      JongseongKind::Closed => Ok(self.closed())
    }
  }

  fn open(self) -> &'static str {
    match self {
      Josa::EunNeun => NEUN,
      Josa::IGa => GA,
      Josa::EulReul => REUL,
      Josa::GwaWa => WA,
      Josa::I => "",
      Josa::Eu => ""
    }
  }

  fn rieul(self) -> &'static str {
    match self {
      Josa::EunNeun => EUN,
      Josa::IGa => I,
      Josa::EulReul => EUL,
      Josa::GwaWa => GWA,
      Josa::I => I,
      Josa::Eu => ""
    }
  }

  fn closed(self) -> &'static str {
    match self {
      Josa::EunNeun => EUN,
      Josa::IGa => I,
      Josa::EulReul => EUL,
      Josa::GwaWa => GWA,
      Josa::I => I,
      Josa::Eu => EU
    }
  }

  fn both(self) -> &'static str {
    match self {
      Josa::EunNeun => "은(는)",
      Josa::IGa => "이(가)",
      Josa::EulReul => "을(를)",
      Josa::GwaWa => "와(과)",
      Josa::I => "(이)",
      Josa::Eu => "(으)"
    }
  }
}


/// Select appropriate josa for a string.
///
/// It is useful when you are trying to append a josa to formatted text such as `<span>고양이</span>`.
/// If you try to append a josa to `<span>고양이</span>`, it results in [`Error`](enum.Error.html) because of the last character `>`.
/// With this method, you can first get an appropriate josa, and then format the text with that josa:
/// ```rust
/// use josa::select;
/// use josa::IGa;
/// # use josa::Error;
///
/// let cat = "고양이";
/// let josa = select(cat, IGa)?;
///
/// let cat = format!(r#"<span class="bold">{}</span>{}"#, cat, josa);
///
/// assert_eq!(cat, r#"<span class="bold">고양이</span>가"#);
/// # Ok::<(), Error>(())
/// ```
///
/// # Errors
/// If given String is an empty String
/// or the last character is not a Haugul Syllable,
/// it returns [`Error`](enum.Error.html).
///
/// # Example
/// ```
/// use josa::select;
/// use josa::EunNeun;
/// # use josa::Error;
///
/// assert_eq!(select("사냥꾼", EunNeun)?, "은");
/// # Ok::<(), Error>(())
/// ```
pub fn select(noun: &str, josa: Josa) -> Result<&'static str> {
  josa.select(
    noun.chars().last().ok_or(Error::EmptyStr)?
  )
}

/// An extension trait to add [`push_josa`](trait.JosaExt.html#tymethod.push_josa) method to [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
pub trait JosaExt {
  fn push_josa(&mut self, josa: Josa);
}

impl JosaExt for String {
  /// Append a given [`Josa`] onto the end of this [`String`].
  ///
  /// Note that it has [edge cases](index.html#edge-cases).
  /// 
  /// [`Josa`]: enum.Josa.html
  /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
  fn push_josa(&mut self, josa: Josa) {
    let josa = match select(self, josa) {
      Ok(josa) => josa,
      Err(err) => match err {
        Error::EmptyStr => "",
        Error::ParseSyllable(_) => josa.both()
      }
    };

    self.push_str(josa);
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
