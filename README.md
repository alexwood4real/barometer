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

## Steps Taken
### TODO

## Current State and Future Plans
As of now, I am connected to my home Wi-Fi network, I am able to read the temperature, pressure, and humidity and display that in my console, and 
I have a blinking LED light acting as a heartbeat to ensure everything is working when I am not connected to my computer. This is just the first step. 
I am able to calcualte much more weather related material from these three variables, as seen in issue [#9](https://github.com/alexwood4real/barometer/issues/9), 
and I plan on being able to display this information via an iOS app so I can personally see this information when I am away from my home, as seen in issue [#13](https://github.com/alexwood4real/barometer/issues/13).
The first part was easy, this second part will require more technical work because I will have to figure out where I want to store this information, as seen in issue [#14](https://github.com/alexwood4real/barometer/issues/14)
and communicating with my application at the same time. In between that work, there are sure to be bugs and things that become broken, but as of now that those are the next steps.