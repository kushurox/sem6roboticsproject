import serial
import struct
import time

# Open the serial port
ser = serial.Serial('COM6', baudrate=115200, timeout=1)
time.sleep(2)  # Let the STM32 reset if it does on USB connection

# Define your four float32 values
values = [0.1, 0.5, 0.9, 0.3]

# Pack them into bytes (little-endian, float32)
payload = struct.pack('<ffff', *values)

# Send the packed byte array
ser.write(payload)
print(f"Sent floats: {values}")
print(f"Sent bytes: {[b for b in payload]}")

ser.close()
