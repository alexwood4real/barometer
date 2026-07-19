# barometer
This is the repository of a Raspberry Pi Pico 2 W with BME 280 sensor written in the Rust language.
I wanted to learn Rust by doing a project related in some capactiy to my career. There are numerous examples of 
how to do this using Python, and many tutorial/documentation relating to using Python and C/C++ on the Pico 2 W, but Rust only started being
supported on a RP2350 in January 2024, so there aren't a lot of great tutorials explaining how to do what I am trying to do with this whole project.

## Hardware
There is a big difference between using a Pico 2 W and a generic Pico 2. The difference is the use of the CYW43439 Wi-Fi chip. 
The first difference I encountered was enabling the LED light. On the standard Pico 2, the LED wired directly to the main processor. 
On a Pico 2 W, the LED is wired to the CYW434329 chip. If you are to use this repo, ensure you have the Pico 2 W. I ordered this off Amazon
using this [link](https://a.co/d/0j32BlzX). I also purchased a [BME 280 sensor](https://a.co/d/0iv6Qy2b) off of Amazon as well. This sensor is used
to measure Temperature, Humidity, and Pressure. This will require soldering which I was able to get help from a friend with. 

## Software
I am using a 2022 Macbook Pro with a M1 Max chip, and I did all my development through VSCode. If you have not installed Rust, I would suggset
exploring the offical [Rust](https://rust-lang.org) website in order to get started and going through the official [Rustlings](https://github.com/rust-lang/rustlings) course to learn basic syntax.

## How to run
With a Pico 2 W in bootsel mode, you can run ./load_pico.sh to load the pico, as the name implies, and then run ./display.sh to show the live output in the terminal.

## Resources
No tutorials were followed for this. However, a lot of frameworks and examples were used:
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Official Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [std](https://doc.rust-lang.org/std/index.html)
- [libm](https://docs.rs/libm/latest/libm/)
- [bme280_rs](https://docs.rs/bme280-rs/latest/bme280_rs/)
- [WiFi](https://github.com/ninjasource/rp-pico2w-examples)
- [embassy](https://github.com/embassy-rs/embassy) this ones huge

## Future Plans
I plan on being able to display this information via an iOS app so I can personally see this information when I am away from my home, as seen in issue [#13](https://github.com/alexwood4real/barometer/issues/13). I will compare the calculated values with the ones from the WeatherKit. After that, this will just be used as a base tool for future projects.
