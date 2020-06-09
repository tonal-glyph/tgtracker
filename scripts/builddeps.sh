#!/usr/bin/env bash

export FROOT="$HOME/proj/tgtracker/3rdparty"
echo "Building tgtracker dependencies..."
figlet -t -f doom samplerate
cd "$FROOT"/libsamplerate || exit
git pull
mkdir -p build
cd build || exit
cmake -GNinja -Wno-dev -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=ON -DLIBSAMPLERATE_INSTALL=ON -DCMAKE_INSTALL_PREFIX=/usr/local ..
/usr/bin/ninja
sudo /usr/bin/ninja install
cd ..
rm -rf build
figlet -t -f doom sndfile
cd "$FROOT"/libsndfile || exit
git pull
mkdir -p build
cd build || exit
cmake -GNinja -DCMAKE_BUILD_TYPE=RELEASE -DENABLE_STATIC_RUNTIME=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_INSTALL_PREFIX=/usr/local ..
/usr/bin/ninja
sudo /usr/bin/ninja install
cd ..
rm -rf build
figlet -t -f doom rtaudio
cd "$FROOT"/rtaudio || exit
git pull
mkdir -p build
cd build || exit
cmake -GNinja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local ..
/usr/bin/ninja
sudo /usr/bin/ninja install
cd ..
rm -rf build
figlet -t -f doom rtmidi
cd "$FROOT"/rtmidi || exit
git pull
mkdir -p build
cd build || exit
cmake -GNinja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local ..
/usr/bin/ninja
sudo /usr/bin/ninja install
cd ..
rm -rf build
figlet -t -f doom rubberband
cd "$FROOT"/rubberband || exit
git pull
./configure
make
sudo make install
make clean
rm -rf config.log config.status Makefile bin/rubberband lib
figlet -t -f doom chuck
cd "$FROOT"/chuck/src || exit
git pull
make linux-alsa
sudo make install
make clean
figlet -t -f doom faust
cd "$FROOT"/faust || exit
git pull
make world
sudo make install
make clean
figlet -t -f doom frut
cd "$FROOT"/frut || exit
git pull
mkdir -p build
cd build || exit
cmake -GNinja .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local -DJUCE_ROOT=../../tracktion_engine/modules/juce
/usr/bin/ninja
sudo /usr/bin/ninja install
cd ..
rm -rf build
figlet -t -f doom rakarrack
cd "$FROOT"/rakarrack || exit
git pull
./autogen.sh
./configure
make
sudo make install
make clean
figlet -t -f doom soundpipe
cd "$FROOT"/soundpipe || exit
git pull
git checkout dev
make
sudo make install
make clean
rm config.mk
figlet -t -f doom sporth
cd "$FROOT"/sporth || exit
git pull
make
sudo make install
make clean
figlet -t -f doom stk
cd "$FROOT"/stk || exit
git pull
autoconf
./configure --with-alsa --with-jack
make
sudo make install
make distclean
rm -f configure
echo "done."