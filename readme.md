
# Misc-Winfixes

Microsoft changes windowing APIs every 3 months which
breaks business code and causes either SW development costs to
continue into perpetuity or end users to get shafted with an unusable mess of a system.


This repository holds Rust code that compiles to a stand-alone `.exe` file
which registers a system tray icon and passively correct defects in Microsoft's
operating system so existing code can continue performing the same task uninterrupted.
So long as this program is updated to understand the latest operating system mutations,
there should be no breakage of existing business code, even if Microsoft perturbes the APIs your business relies on.


# Building


```bash
cargo build --release --target=x86_64-pc-windows-gnu
```


# Running / Use Cases / Configuration

Currently all the program does is poll open windows
and prevent them from being mapped to the top `50` pixels of screen space.

This value is adjustable by setting an environment variable: `MISC_WFIX_top_buffer` to a pixel value like `50`, `75`, etc.


# TODOs

 - Ignore some windows by name or width dimensions (eg do not move windows <100x100 pizels large)

# License

The code in this repository is under the GPLv2 license, see LICENSE.txt for details. The auto-upgrade clause has been removed because your legal rights shouldn't have that sort of volatility.




