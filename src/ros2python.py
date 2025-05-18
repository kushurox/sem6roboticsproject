import rclpy
from rclpy.node import Node
import sys
import tty
import termios
import select
import struct
import time
import serial

class KeyboardInputNode(Node):
    def __init__(self):
        super().__init__('keyboard_input_node')
        self.get_logger().info("Keyboard input node started. Press keys...")

        # Uncomment and set the correct port
        self.serial = serial.Serial('/dev/serial/by-id/usb-kushurox_totm_TEST-if00', baudrate=115200, timeout=1)

        # Create a timer that fires every 100ms
        self.timer = self.create_timer(0.1, self.read_key)

    def read_key(self):
        if self.kbhit():
            key = sys.stdin.read(1)
            self.get_logger().info(f"Key pressed: {repr(key)}")
            data = [0, 0, 0, 0]
            if key == 'a':
                print("sent a stuff")
                data = [0.03, 0.02, 0.1, 0.15]
            elif key == 's':
                print("sent  stuff s")
                data = [0.12, 0.02, 0.1, 0.05]
            payload = struct.pack("<ffff", *data)
            self.serial.write(payload)

    def kbhit(self):
        dr, dw, de = select.select([sys.stdin], [], [], 0)
        return dr != []

def main(args=None):
    rclpy.init(args=args)

    # Setup terminal for non-blocking key reads
    old_settings = termios.tcgetattr(sys.stdin)
    tty.setcbreak(sys.stdin.fileno())

    node = KeyboardInputNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, old_settings)
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
