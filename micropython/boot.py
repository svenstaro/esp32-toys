import ssd1306
import sx127x
from machine import Pin, I2C, SoftI2C, reset, SPI
import time
import network


def flash_boot_led():
    led = Pin(25, Pin.OUT)
    for _ in range(5):
        led.on()
        time.sleep_ms(100)
        led.off()
        time.sleep_ms(100)


def init_display():
    oled_reset = Pin(16, Pin.OUT)
    oled_reset.value(0)
    time.sleep_ms(20)
    oled_reset.value(1)

    i2c = SoftI2C(scl=Pin(15, Pin.OUT, Pin.PULL_UP), sda=Pin(4, Pin.OUT, Pin.PULL_UP))
    display_width = 128
    display_height = 64
    display = ssd1306.SSD1306_I2C(display_width, display_height, i2c)
    return display


def connect_wifi():
    start = time.ticks_ms()
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    connection_timeout = 10000
    wlan.connect("my essid", "my pw")
    print("Starting")
    while not wlan.isconnected() and time.ticks_diff(time.ticks_ms(), start) < 10000:
        pass

    if not wlan.isconnected():
        return False
    return wlan.ifconfig()


flash_boot_led()
display = init_display()
display.text("Booting...", 0, 0)
display.show()
display.text("Connecting wifi", 0, 10)
display.show()
wifi = connect_wifi()
if wifi:
    display.text("Got IP:", 0, 20)
    display.text(wifi[0], 0, 30)
    display.show()
else:
    display.text("Wifi failed", 0, 20)
    display.show()
    time.sleep(10)
    reset()

device_spi = SPI(
    baudrate=10000000,
    polarity=0,
    phase=0,
    bits=8,
    firstbit=SPI.MSB,
    sck=Pin(5, Pin.OUT, Pin.PULL_DOWN),
    mosi=Pin(27, Pin.OUT, Pin.PULL_UP),
    miso=Pin(19, Pin.IN, Pin.PULL_UP),
)

device_config = {
    "miso": 19,
    "mosi": 27,
    "ss": 18,
    "sck": 5,
    "dio_0": 26,
    "reset": 14,
    "led": 2,
}

lora_parameters = {
    "frequency": 868e6,
    "tx_power_level": 2,
    "signal_bandwidth": 125e3,
    "spreading_factor": 8,
    "coding_rate": 5,
    "preamble_length": 8,
    "implicit_header": False,
    "sync_word": 0x12,
    "enable_CRC": False,
    "invert_IQ": False,
}

lora = sx127x.SX127x(device_spi, pins=device_config, parameters=lora_parameters)

button = Pin(0, Pin.IN, Pin.PULL_UP)
send_counter = 0
recv_counter = 0

while True:
    if button.value() == 0:
        send_counter += 1
        lora.println("lol")

    if lora.received_packet():
        payload = lora.read_payload()
        if payload == b"lol":
            recv_counter += 1

    display.fill_rect(0, 50, 128, 8, 0)
    display.text("R: {} | S: {}".format(recv_counter, send_counter), 0, 50)
    display.show()
