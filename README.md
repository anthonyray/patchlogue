# patchlogue
[![Continuous Integration](https://github.com/anthonyray/patchlogue/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/anthonyray/patchlogue/actions/workflows/rust.yml)

[A tool](https://www.patchlogue.com) to help you visualize patches created on your [KORG Prologue synthesizer](https://www.korg.com/products/synthesizers/prologue/).

Just plug your KORG Prologue in, select the appropriate MIDI INs and OUTs, and start visualizing your patches !

In one glance, you can inspect and get a sense of the parameters used to make up the sound of your patches. You can also save your patches so that you can visualize them later.



https://github.com/anthonyray/patchlogue/assets/3779257/1a6f1578-a78a-4081-880e-168c33b185dc



# Usage
## Setting up patchlogue
1. Head over to [patchlogue.com](https://www.patchlogue.com)
2. Plug your synthesizer in using USB
3. Give patchlogue your permission for MIDI access
3. Select `PROLOGUE KBD/KNB` as your MIDI input and `PROLOGUE SND` as MIDI output
4. You're all set 🎉 !

## Visualizing your patches
Just hit the "`retrieve patch`" button and you will see all your parameters laid out !

## Saving / loading your patches
You can save your patch by hitting the "`save patch`" button. This will produce a `JSON` file that you can load later using the "`load patch`" button. 

### Supported browsers :
- Mozilla Firefox
- Google Chrome

### Supported synthesizers

`patchlogue` has been tested with the **KORG Prologue 16** synthesizer, but should work with the **KORG Prologue 8**. 

