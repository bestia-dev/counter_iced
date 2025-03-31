<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# counter_iced

[//]: # (auto_cargo_toml_to_md start)

**counter GUI with iced**  
***version: 2025.331.1335 date: 2025-03-31 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/counter_iced)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![tutorial](https://img.shields.io/badge/tutorial-orange)
 ![iced](https://img.shields.io/badge/iced-orange)
 ![rust](https://img.shields.io/badge/rust-orange)
 ![gui](https://img.shields.io/badge/gui-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![License](https://img.shields.io/badge/license-MIT-blue.svg)
 ![counter_iced](https://bestia.dev/webpage_hit_counter/get_svg_image/748236206.svg)

Hashtags: #tutorial #iced #rust #gui  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## GUI for windows

A simple test program that creates a simple GUI for Windows with the crate `iced`.

This is an "retained mode" GUI and I will use that in my future works.
The resulting exe is heavier than with egui "immediate mode", but the exe looks like a proper GUI program. "Iced" is used to create the Cosmic Desktop of System76 and it looks nice.

## Cross compile to windows

On my machine I have Windows11 with WSL/Debian. I will cross compile to Windows, copy the exe file with `scp` and run it on Windows. 

I use `cargo-auto` for automation of the build process and to commit to GitHub. Just run `cargo auto` and follow the instructions. To work with GitHub it will need the Personal Access Token from <https://github.com/settings/tokens>.  

Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:

```bash
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/counter_iced/target/x86_64-pc-windows-gnu/release/counter_iced.exe /c/Users/Luciano/rustprojects/counter_iced/

# then run in git-bash
cd ~/rustprojects/counter_iced
./counter_iced.exe
```

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
