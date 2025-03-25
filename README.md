# [⚒️ Embassy workshop](https://rust.ipworkshop.ro/docs/embassy)

![PMRust Lab logo](https://gitlab.cs.pub.ro/pmrust/pmrust.pages.upb.ro/-/raw/main/website/static/img/logo.svg?ref_type=heads)




# 🙋‍♂️ What I did in this Rust Workshop

I enhanced my understanding of **Rust** and programming an **MCU** in **embedded systems**
by playing with a Rasberry Pi Pico board, the same used in the [PM Rust laboratories](https://pmrust.pages.upb.ro/).


## [💡 Task 0: Make an LED blink](./src/bin/demo.rs)

Using a **GPIO** pin on the Rasberry Pi and a **jumper wire**
to connect it to the pin of an LED,
I managed to make the LED blink,
either **RED**, **GREEN**, **BLUE** or **YELLOW** (depending on the LED's pin).

Each color corresponds to a totally different LED:
- the pin of the LEDs are labeled on the board, above the screen and in the left side of the Rasberry Pico 2 W
- The LEDs that will blink are found in the left-upper corner of the board, obve the black pin holes and below the switches (sw4-7)

### ⚠️ Attention, Amateurs!


```rs
let mut led = Output::new(p.PIN_4, embassy_rp::gpio::Level::High);
```

Please note that `PIN_4` does NOT correspond to physical pin 4 on the Raspberry Pi Pico.
Instead, it refers to **General Purpose** pin 4 (**GP4**), which is **physically located on pin 6** of the board.

Take a look at the Rasberry Pi Pico's pin layout [here](https://rust.ipworkshop.ro/assets/images/pico2w-pinout-49532ea10ab0caedc6a6f21d1bf504bf.svg).


## [🖰 Task 1: Detect when buttons are pressed](./src/bin/hi.rs)

You've seen the buttons on the top of the board, have you?!
They are labeled **SW4**, **SW5**, **SW6**, **SW7**.

In this task, I implemented an **asynchronous algorithm**
to detect when any of these buttons is pressed
(you can pick any one and press it—it doesn’t matter which!).

- I used the `defmt`, a **Rust framework (crate)** to log the activity on the board.
    It prints messages on the screen of my laptop.
- I used **jumper wires** to connect the Rasberry Pico to the pins of the buttons

    | Rasberry Pi Pico pin     | Board pin |
    | :---:                    | :---:     | 
    | **GP2** (physical pin 4) | SW4       |
    | **GP3** (physical pin 5) | SW5       |
    | **GP4** (physical pin 6) | SW6       |
    | **GP5** (physical pin 7) | SW7       |


    > The buttons's pins (switches **SW4...7**) are located to the left of the colored LEDs pins

- I set the **GPIO** pins as **Input pins** and activated the **pull-up** restistors
  - > The pins will check the buttons' states. 
- Using the **asynchronous** `select` function from the `embassy-futures` crate,
    I waited for any of the four buttons to be pressed,
    and once triggered, I logged the name of my team, *I/O CTL*, using `defmt`


## [🔊 Task 3: Sing my own tune](./src/bin/sing.rs)


<!-- TODO: describe it! -->