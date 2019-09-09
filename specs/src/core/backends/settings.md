## Settings Control Panel

Gets/sets tuning of A4 to 440Hz or 432Hz RADIO
Midi note 69
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tuning {
    A440 = 440,
    A432 = 432,
}
impl Tuning {
    pub const VARIANTS: [Tuning; 2] = [Tuning::A440, Tuning::A432];
    pub fn set_tuning_a440() {}
    pub fn set_tuning_a432() {}
}
impl Default for Tuning {
    fn default() -> Self {
        Tuning::A440
    }
}
```
Gets/sets current octave UP/DOWN buttons
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Octave {
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
}
impl Octave {
    pub const VARIANTS: [Octave; 8] = [
        Octave::C1,
        Octave::C2,
        Octave::C3,
        Octave::C4,
        Octave::C5,
        Octave::C6,
        Octave::C7,
        Octave::C8,
    ];
    pub fn get_current_octave() {}
    pub fn set_current_octave() {}
}
impl Default for Octave {
    fn default() -> Self {
        Octave::C4
    }
}
```
Gets/sets current sample rate RADIO
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleRate {
    KHz44100 = 44100,
    KHz48000 = 48000,
    KHz96000 = 96000,
}
impl SampleRate {
    #[rustfmt::skip]
    pub const VARIANTS: [SampleRate; 3] = [
        SampleRate::KHz44100,
        SampleRate::KHz48000,
        SampleRate::KHz96000
    ];
    pub fn get_sample_rate() {}
    pub fn set_sample_rate() {}
}
impl Default for SampleRate {
    fn default() -> Self {
        SampleRate::KHz48000
    }
}
```
Gets/sets sample bit depth RADIO
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleBitDepth {
    Bit16 = 16,
    Bit32 = 32,
    Bit64 = 64,
}
impl SampleBitDepth {
    pub const VARIANTS: [SampleBitDepth; 3] = [
        SampleBitDepth::Bit16,
        SampleBitDepth::Bit32,
        SampleBitDepth::Bit64,
    ];
    pub fn get_sample_bit_depth() {}
    pub fn set_sample_bit_depth() {}
}
impl Default for SampleBitDepth {
    fn default() -> Self {
        SampleBitDepth::Bit32
    }
}
```
Gets/sets interpolation algorithm RADIO
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interpolation {
    None,
    Linear,
    CubicLagrange,
    CubicSpline,
    FastSinc,
    PreciseSinc,
}
impl Interpolation {
    pub const VARIANTS: [Interpolation; 6] = [
        Interpolation::None,
        Interpolation::Linear,
        Interpolation::CubicLagrange,
        Interpolation::CubicSpline,
        Interpolation::FastSinc,
        Interpolation::PreciseSinc,
    ];
    pub fn get_interpolation() {}
    pub fn set_interpolation() {}
}
impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Linear
    }
}
```
Gets/sets current theme DROPDOWN
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurrentTheme {
    DarkTheme,
    DefaultTheme,
    LightTheme,
}
impl CurrentTheme {
    pub const VARIANTS: [CurrentTheme; 3] = [
        CurrentTheme::DarkTheme,
        CurrentTheme::DefaultTheme,
        CurrentTheme::LightTheme,
    ];
    pub fn get_current_theme() {}
    pub fn set_current_theme() {}
    pub fn set_default_theme() {}
}
impl Default for CurrentTheme {
    fn default() -> Self {
        CurrentTheme::DefaultTheme
    }
}
```
Gets/sets current keybindings mode DROPDOWN
TGTracker's bindings are modeled after MilkyTracker's defaults
```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeybindMode {
    Custom,
    FastTracker,
    TGTracker,
}
impl KeybindMode {
    pub const VARIANTS: [KeybindMode; 3] = [
        KeybindMode::Custom,
        KeybindMode::FastTracker,
        KeybindMode::TGTracker,
    ];
    pub fn get_keybind_mode() {}
    pub fn set_keybind_mode() {}
    pub fn set_default_mode() {}
}
impl Default for KeybindMode {
    fn default() -> Self {
        KeybindMode::TGTracker
    }
}
```