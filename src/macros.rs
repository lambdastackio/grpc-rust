/*
 Copyright 2016 LambdaStack All rights reserved.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

/// repeat - Prints out a repeat of characters
///
/// Currently prints characters using the default color.
///
/// # Example
/// ```
/// sep!("=", 80);
/// ```
#[macro_export]
macro_rules! repeat {
    ($e:expr, $size:expr) => {
        let repeated: String = iter::repeat($e).take($size).collect();
        println!("{}", repeated);
    }
}

/// repeat_red - Prints out a repeat of characters in Red
///
/// Currently prints characters using the color red.
///
/// # Example
/// ```
/// repeat_red!("=", 80);
/// ```
#[macro_export]
macro_rules! repeat_red {
    ($e:expr, $size:expr) => {
        let repeated: String = iter::repeat($e).take($size).collect();
        println!("{}", Red.paint(repeated));
    }
}

/// repeat_yellow - Prints out a repeat of characters in Yellow
///
/// Currently prints characters using the color yellow.
///
/// # Example
/// ```
/// repeat_yellow!("=", 80);
/// ```
#[macro_export]
macro_rules! repeat_yellow {
    ($e:expr, $size:expr) => {
        let repeated: String = iter::repeat($e).take($size).collect();
        println!("{}", Yellow.paint(repeated));
    }
}

/// repeat_blue - Prints out a repeat of characters in Blue
///
/// Currently prints characters using the color blue.
///
/// # Example
/// ```
/// repeat_blue!("=", 80);
/// ```
#[macro_export]
macro_rules! repeat_blue {
    ($e:expr, $size:expr) => {
        let repeated: String = iter::repeat($e).take($size).collect();
        println!("{}", Blue.paint(repeated));
    }
}
