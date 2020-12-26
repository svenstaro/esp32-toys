#!/bin/bash
for file in sx127x.py ssd1306.py boot.py; do
    echo "Uploading $file"
    ampy -p /dev/ttyUSB0 put $file
done
echo "Resetting..."
ampy -p /dev/ttyUSB0 reset --hard
