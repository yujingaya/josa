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

use std::ops::{Add, AddAssign};

use hangul::HangulExt;

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
  GwaWa
}

use Josa::{EunNeun, IGa, EulReul, GwaWa};

impl Josa {
  fn open(self) -> &'static str {
    match self {
      EunNeun => NEUN,
      IGa => GA,
      EulReul => REUL,
      GwaWa => WA
    }
  }

  fn closed(self) -> &'static str {
    match self {
      EunNeun => EUN,
      IGa => I,
      EulReul => EUL,
      GwaWa => GWA
    }
  }
}


pub trait JosaExt {
  fn push_josa(&mut self, josa: Josa);
}

impl JosaExt for String {
  fn push_josa(&mut self, josa: Josa) {
    self.push_str(
      // Document that it'll panic if the string is empty
      match self.chars().last().unwrap().is_open() {
        Ok(true) => josa.open(),
        Ok(false) => josa.closed(),
        Err(_) => josa.open(),
      }
    );
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
