// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` errors
error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseChar(::std::char::ParseCharError);
        ParseInt(::std::num::ParseIntError);
        Recv(::std::sync::mpsc::RecvError);
        RecvTimeout(::std::sync::mpsc::RecvTimeoutError);
        SendError(::std::sync::mpsc::SendError<i64>);
        Shape(::ndarray::ShapeError);
        Regex(::regex::Error);
        TryFromInt(::std::num::TryFromIntError);
    }

    errors {
        InvalidIdx
    }
}
