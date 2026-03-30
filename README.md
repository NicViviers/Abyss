# Abyss

A high-integrity, bare-metal monolithic kernel written in **Rust** for the **RP2350A** microcontroller. This project powers a technical diving computer with a GUI driven by **LVGL** and a high-precision mixed-gas decompression engine.

---

## 🛠 Tech Stack

* **Language:** Rust (`no_std`, bare-metal)
* **GUI Framework:** LVGL (C-based, integrated via Rust FFI)
* **Architecture:** ARM Cortex-M33 (RP2350A)
* **Decompression Model:** Bühlmann ZHL-16C with Gradient Factors (GF) and predictive Multi-Gas

## ✨ Key Features

* **Bühlmann ZHL-16C Engine:** A highly sophisticated predictive mixed-gas engine supporting Trimix, Nitrox, and Air.
* **Gradient Factors:** User-definable $GF_{low}$ and $GF_{high}$ for conservative surfacing profiles.
* **Monolithic Kernel:** Direct hardware abstraction layers (HAL) for power management and pressure sensor telemetry
* **Hybrid GUI:** Seamless integration between Rust's safety and LVGL’s powerful C-based rendering engine.
* **Multi-Threading:**: Takes advantage of both available threads for dedicated rendering and data processing loops

---

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain:** Install the `thumbv8m.main-none-eabihf` target:
    ```bash
    rustup target add thumbv8m.main-none-eabihf
    ```
2.  **C Toolchain:** Ensure `arm-none-eabi-gcc` is available for compiling the LVGL C bindings.
3.  **Pico-SDK:** Ensure the Pico-SDK from Raspberry Pi is available for compiling

### Building

To compile the kernel for the RP2350A simply execute the helper script while the RP2350 is in Mass Storage Mode:

```bash
chmod +x fast_compile.sh
./fast_compile.sh
```
