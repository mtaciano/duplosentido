# About
DuploSentido is a crate that exposes the DualSense (PS5) controller to the
user, making it possible to interact with the controller in an ergonomic and
simplified way. This crate uses _polling_ to get the controller state. Contrary
to callbacks, polling can be more ergonomic to implement into code, as it's
possible to use controller state to drive logic in any part of the code. While
with callbacks, logic can only happen when a button is pressed.

# Meaning
The word "DuploSentido" is a play with the word "DualSense" when translated.
The phrase "dual sense" could be interpreted, in Brazilian Portuguese, as
"duplo sentido". As there were already multiple crates with names that
contained `dualsense`, I decided to name this one something different.

# External Dependencies
This crate depends on [HIDAPI](https://github.com/libusb/hidapi) to interact
with the controller. You should certify that it's installed in your system
before trying to use this crate, as it doesn't download it automatically for
you.

# Roadmap
This crate is still a major work in progress. Below you can see the "roadmap"
for this crate, in no particular order:
- [x] Linux support.
- [ ] Windows and macOS support.
- [x] USB connection support.
- [ ] Bluetooth connection support.
- [ ] Adaptive trigger support.
- [ ] Vibration support.
- [ ] No dependencies (maybe?).

# License
This crate is licensed under the MIT license. So you are basically free to do
as you please (with some minor restrictions).
