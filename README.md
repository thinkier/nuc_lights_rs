# NUC LED Drivers for Linux
Proof of concept for building a linux kernel module in pure rust.

## Currently not working
- Getting the current state of the LEDs (can't be bothered to figure this one out, the reference code at [milesp20's](https://github.com/milesp20/intel_nuc_led/blob/master/nuc_led.c) repository seems to stray quite far from [Inte's documentation](https://www.intel.com/content/www/us/en/support/articles/000023426/mini-pcs/intel-nuc-kits.html))

## Short conclusion (As of 2018-11-10)
Ecosystem is not mature enough yet for a pure Rust kernel module other than experimentation, perhaps a partially Rust / partially C module may be better suited for writing kernel modules. Such as [rust.ko](https://github.com/tsgates/rust.ko)
