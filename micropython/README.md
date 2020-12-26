# How to use the Python parts

First get the most recent ESP-IDF v4.x firmware from https://micropython.org/download/esp32/

    pacman -S esptool
    paru -S adafruit-ampy
    esptool.py --chip esp32 --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 esp32-idf4-20201225-unstable-v1.13-268-gf7aafc062.bin
    ./upload.sh
