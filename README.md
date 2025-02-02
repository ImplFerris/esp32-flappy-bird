# Flappy Bird Game on ESP32 with OLED Display, written in Rust

 Flappy Bird Game written in Rust for the ESP32 with an OLED display, using the Embassy framework.

## Hardware Requirements
- ESP32 (WROOM Dev Kit 1)
- SSD1306 OLED I2C 128x64 Display
- Push Button
- Jumper wires and breadboard
- \[Optiona\]: Active Buzzer

You can use the joystick module because it has a push button. I used that only because it was easy to play.
    
## Circuit

| ESP32 Pin | Component               |
|----------|-------------------------|
| GPIO 23  | SDA pin of OLED         |
| GPIO 18  | SCL pin of OLED         |
| 3.3V     | VCC pin of OLED         |
| GND      | GND pin of OLED         |
| GPIO 32   | One side of push button |
| GND      | Other side of push button |

### Optional 

If you want a buzzer sound when you score, you can add this optional circuit.

| ESP32 Pin | Component               |
|----------|-------------------------|
| GPIO 33   | Positive side of buzzer |
| GND      | Other side of buzzer |

## How to Run?

```sh
cargo run --release
```

or with Buzzer support

```sh
cargo run --release -F buzzer
```

## Related Tutorials

You can refer to the following tutorials in the "impl Rust on ESP32" book to learn how to use [OLED](https://esp32.implrust.com/oled/index.html) and [Active Buzzer](https://esp32.implrust.com/buzzer/index.html) with the ESP32.
