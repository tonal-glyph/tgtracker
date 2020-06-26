#!/usr/bin/env bash

mkdir -pv ../3rdparty
cd ../3rdparty || exit
rm -rf *
git clone --recursive https://github.com/ccrma/chuck
git clone --recursive https://github.com/grame-cncm/faust
git clone --recursive https://github.com/McMartin/FRUT frut
git clone --recursive https://github.com/erikd/libsamplerate
git clone --recursive https://github.com/erikd/libsndfile
git clone --recursive https://git.code.sf.net/p/rakarrack/git rakarrack
git clone --recursive https://github.com/thestk/rtaudio
git clone --recursive https://github.com/thestk/rtmidi
git clone --recursive https://github.com/breakfastquay/rubberband
git clone --recursive https://github.com/paulbatchelor/Soundpipe soundpipe
git clone --recursive https://github.com/PaulBatchelor/Sporth.git sporth
git clone --recursive https://github.com/thestk/stk
git clone --recursive https://github.com/Tracktion/tracktion_engine
clear