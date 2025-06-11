### This project makes a port scanner in Python.
### Usage: python3 scanner.py <target>
###                 argv[0]    argv[1]

import sys
import socket
from datetime import datetime
import threading

### Function to scan a port.
def scanport(target, port):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex((target, port))
        if result == 0:
            print(f"Port {port} is open")
        sock.close()
    except socket.error as e:
        print(f"Could not connect to {target} on {port} - {e}")
    except Exception as e:
        print(f"An error occurred: {e}")

def main():
    if len(sys.argv) == 2:
        target = sys.argv[1]
    else:
        print("Invalid command")
        print("Usage: python3 scanner.py <target>")
        sys.exit(1)

    try:
        target_ip = socket.gethostbyname(target)        # This will return an IP if we give it a string of same IP instead of hostname.
    except socket.gaierror:
        print(f"Could not resolve hostname {target}")
        sys.exit(1)

    print(f"Scanning Target - {target_ip}")
    print(f"Time Started - {datetime.now()}")
    print("-" * 50)

    ### Lets use threads
    try:
        threads = []
        for port in range(1, 65536):
            thread = threading.Thread(target=scanport, args=(target_ip, port))
            threads.append(thread)
            thread.start()

        # Wait for all threads to complete.
        for thread in threads:
            thread.join()

    except KeyboardInterrupt:
        print("\nExiting program.")
        sys.exit(0)

    except socket.error as e:
        print(f"Socket error - {e}")
        sys.exit(1)

    print("\nScan Completed!")

if __name__ == "__main__":
    main()
