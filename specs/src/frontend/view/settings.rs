//@ - [ ] Autosave (timed interval, lose focus, on dirty)
//@ - [ ] Buffer size 512/1024/2048/4096 (8bit/16bit/32bit/64bit)
//@ - [ ] Built-in tuner for guitars, sound files, etc.
//@ - [ ] Current octave ENUM
//@ - [ ] Current theme ENUM
//@ - [ ] Dithering
//@ - [ ] Enable Lua scripting BOOLEAN
//@ - [ ] Fadeout
//@ - [ ] Input audio device (hw:0, etc.) ENUM
//@ - [ ] Input MIDI device ENUM
//@ - [ ] Interpolation (None, Linear, Cubic) ENUM
//@ - [ ] Keybindings (setups like FT2 style?) ENUM
//@ - [ ] Kill notes on stop
//@ - [ ] Load extra formats (through rodio crate) BOOLEAN
//@ - [ ] Load/Save config to TOML or SCL DIALOG
//@ - [ ] Master volume/preamp
//@ - [ ] Max channels shown 4/6/8/12 ENUM
//@ - [ ] Multichannel/Keyjazz MIDIChannel, MIDIVelocity, MIDIAftertouch
//@ - [ ] Output audio device ENUM
//@ - [ ] Output MIDI device ENUM
//@ - [ ] Panning envelope
//@ - [ ] Quantize
//@ - [ ] Reset to defaults DIALOG FUNCTION
//@ - [ ] Sample bit-depth (32-bit default) ENUM
//@ - [ ] Sample rate (44100Hz default) ENUM
//@ - [ ] Sort files by name or extension
//@ - [ ] Sort priority
//@ - [ ] Start in fullscreen BOOLEAN
//@ - [ ] Transpose
//@ - [ ] Tuning A440 or A432 ENUM
//@ - [ ] Vibrato (rate, depth, sweep, type)
//@ - [ ] Volume envelope
//@
//@     // functions
//@     void resetConfig(void);
//@     uint8_t loadConfig(uint8_t showErrorFlag);
//@     void loadConfig2(void); /* called by "Load config" button */
//@     uint8_t saveConfig(uint8_t showErrorFlag);
//@     void saveConfig2(void); /* called by "Save config" button */
//@     void loadConfigOrSetDefaults(void);
//@     void showConfigScreen(void);
//@     void hideConfigScreen(void);
//@     void exitConfigScreen(void);
//@     void setConfigIORadioButtonStates(void);
//@     void configToggleS3MLoadWarning(void);
//@     void configToggleNotYetAppliedWarning(void);
//@     void drawAudioOutputList(void);
//@     void drawAudioInputList(void);
//@     /* PUSH BUTTONS */
//@     void configAmpDown(void);
//@     void configAmpUp(void);
//@     void configMasterVolDown(void);
//@     void configMasterVolUp(void);
//@     void configPalRDown(void);
//@     void configPalRUp(void);
//@     void configPalGDown(void);
//@     void configPalGUp(void);
//@     void configPalBDown(void);
//@     void configPalBUp(void);
//@     void configPalContDown(void);
//@     void configPalContUp(void);
//@     void configQuantizeUp(void);
//@     void configQuantizeDown(void);
//@     void configMIDIChnUp(void);
//@     void configMIDIChnDown(void);
//@     void configMIDITransUp(void);
//@     void configMIDITransDown(void);
//@     void configMIDISensDown(void);
//@     void configMIDISensUp(void);
//@     /* RADIO BUTTONS */
//@     void rbConfigIODevices(void);
//@     void rbConfigLayout(void);
//@     void rbConfigMiscellaneous(void);
//@     void rbConfigMidiInput(void);
//@     void rbConfigSbs512(void);
//@     void rbConfigSbs1024(void);
//@     void rbConfigSbs2048(void);
//@     void rbConfigSbs4096(void);
//@     void rbConfigAudio16bit(void);
//@     void rbConfigAudio24bit(void);
//@     void rbConfigAudio32kHz(void);
//@     void rbConfigAudio44kHz(void);
//@     void rbConfigAudio48kHz(void);
//@     void rbConfigAudio96kHz(void);
//@     void rbConfigFreqTableAmiga(void);
//@     void rbConfigFreqTableLinear(void);
//@     void rbConfigMouseNice(void);
//@     void rbConfigMouseUgly(void);
//@     void rbConfigMouseAwful(void);
//@     void rbConfigMouseUseable(void);
//@     void rbConfigScopeOriginal(void);
//@     void rbConfigMouseBusyVogue(void);
//@     void rbConfigMouseBusyMrH(void);
//@     void rbConfigScopeLined(void);
//@     void rbConfigPatt4Chans(void);
//@     void rbConfigPatt6Chans(void);
//@     void rbConfigPatt8Chans(void);
//@     void rbConfigPatt12Chans(void);
//@     void rbConfigFontCapitals(void);
//@     void rbConfigFontLowerCase(void);
//@     void rbConfigFontFuture(void);
//@     void rbConfigFontBold(void);
//@     void rbConfigPalPatternText(void);
//@     void rbConfigPalBlockMark(void);
//@     void rbConfigPalTextOnBlock(void);
//@     void rbConfigPalMouse(void);
//@     void rbConfigPalDesktop(void);
//@     void rbConfigPalButttons(void);
//@     void rbConfigPalArctic(void);
//@     void rbConfigPalLitheDark(void);
//@     void rbConfigPalAuroraBorealis(void);
//@     void rbConfigPalRose(void);
//@     void rbConfigPalBlues(void);
//@     void rbConfigPalSpacePigs(void);
//@     void rbConfigPalGold(void);
//@     void rbConfigPalViolent(void);
//@     void rbConfigPalHeavyMetal(void);
//@     void rbConfigPalWhyColors(void);
//@     void rbConfigPalJungle(void);
//@     void rbConfigPalUserDefined(void);
//@     void rbFileSortExt(void);
//@     void rbFileSortName(void);
//@     void rbWinSizeAuto(void);
//@     void rbWinSize1x(void);
//@     void rbWinSize2x(void);
//@     void rbWinSize3x(void);
//@     void rbWinSize4x(void);
//@     /* CHECK BOXES */
//@     void cbToggleAutoSaveConfig(void);
//@     void cbConfigInterpolation(void);
//@     void cbConfigVolRamp(void);
//@     void cbConfigDither(void);
//@     void cbConfigPattStretch(void);
//@     void cbConfigHexCount(void);
//@     void cbConfigAccidential(void);
//@     void cbConfigShowZeroes(void);
//@     void cbConfigFramework(void);
//@     void cbConfigLineColors(void);
//@     void cbConfigChanNums(void);
//@     void cbConfigShowVolCol(void);
//@     void cbSampCutToBuff(void);
//@     void cbPattCutToBuff(void);
//@     void cbKillNotesAtStop(void);
//@     void cbFileOverwriteWarn(void);
//@     void cbMultiChanRec(void);
//@     void cbMultiChanKeyJazz(void);
//@     void cbMultiChanEdit(void);
//@     void cbRecKeyOff(void);
//@     void cbQuantisize(void);
//@     void cbChangePattLenInsDel(void);
//@     void cbMIDIAllowPC(void);
//@     void cbMIDIEnable(void);
//@     void cbMIDIRecTransp(void);
//@     void cbMIDIRecAllChn(void);
//@     void cbMIDIRecVelosity(void);
//@     void cbMIDIRecAftert(void);
//@     void cbVsyncOff(void);
//@     void cbFullScreen(void);
//@     void cbPixelFilter(void);
//@     /* SCROLLBARS */
//@     void sbAmp(int32_t pos);
//@     void sbMasterVol(int32_t pos);
//@     void sbPalRPos(int32_t pos);
//@     void sbPalGPos(int32_t pos);
//@     void sbPalBPos(int32_t pos);
//@     void sbPalContrastPos(int32_t pos);
//@     void sbMIDISens(int32_t pos);
