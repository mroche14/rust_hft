import json
import os
import subprocess

def check_hugepages():
    """Check and display the current huge pages configuration."""
    try:
        with open("/proc/meminfo") as meminfo:
            lines = meminfo.readlines()
            hugepages_info = [line for line in lines if "HugePages" in line]
            print("\nCurrent Huge Pages Configuration:")
            for info in hugepages_info:
                print(info.strip())
    except FileNotFoundError:
        print("Huge pages are not supported on this system.")

def set_hugepages(required_pages):
    """Set the number of huge pages."""
    try:
        print(f"Setting huge pages to {required_pages}...")
        with open("/proc/sys/vm/nr_hugepages", "w") as hugepages_file:
            hugepages_file.write(str(required_pages))
        print("Huge pages successfully configured.")
    except PermissionError:
        print("Permission denied: Please run this script as root to configure huge pages.")

def configure_ulimit():
    """Set the ulimit for memory lock."""
    try:
        print("Configuring ulimit for memory lock...")
        # Run the command in a shell context
        subprocess.run("ulimit -l unlimited", shell=True, check=True, executable="/bin/bash")
        print("Ulimit successfully configured.")
    except subprocess.CalledProcessError:
        print("Failed to set ulimit. Please ensure you have sufficient permissions and run as root.")

def parse_config(config_path):
    """Parse the JSON configuration file."""
    try:
        with open(config_path, "r") as config_file:
            config = json.load(config_file)
        return config
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading configuration file: {e}")
        return None

def main():
    default_config_path = "config.json"
    if os.path.exists(default_config_path):
        print(f"Configuration file found: {default_config_path}")
        config_path = default_config_path
    else:
        config_path = input("Configuration file not found in the current folder. Enter the path to your configuration file: ")

    config = parse_config(config_path)

    if not config:
        return

    buffer_config = config.get("buffer", {})
    os_optimization = config.get("os_optimization", {})

    # Calculate required memory based on the buffer configuration
    row_size = buffer_config.get("num_cols", 0) * 8  # Assuming f64 (8 bytes per value)
    total_memory = buffer_config.get("num_rows", 0) * row_size
    huge_page_size = 2 * 1024 * 1024  # Default huge page size: 2MB
    required_pages = (total_memory // huge_page_size) + 1

    print("\nBuffer Configuration:")
    print(f"Number of Rows: {buffer_config.get('num_rows', 0)}")
    print(f"Number of Columns: {buffer_config.get('num_cols', 0)}")
    print(f"Total Memory Required: {total_memory / (1024 * 1024):.2f} MB")
    print(f"Huge Pages Required: {required_pages}")

    if os_optimization.get("use_huge_pages", False):
        check_hugepages()
        set_hugepages(required_pages)

    if os_optimization.get("lock_memory", False):
        configure_ulimit()

    print("\nConfiguration complete. System is ready for the application.")

if __name__ == "__main__":
    main()
