#!/bin/bash

# Check and display current huge pages configuration
check_hugepages() {
  echo "Current Huge Pages Configuration:"
  if [ -f "/proc/meminfo" ]; then
    grep "HugePages" /proc/meminfo
  else
    echo "Huge pages are not supported on this system."
  fi
}

# Set the number of huge pages
set_hugepages() {
  local required_pages=$1
  echo "Setting huge pages to ${required_pages}..."
  if [ "$(id -u)" -ne 0 ]; then
    echo "Permission denied: Please run this script as root to configure huge pages."
    exit 1
  fi
  echo "${required_pages}" > /proc/sys/vm/nr_hugepages
  echo "Huge pages successfully configured."
}

# Set ulimit for memory lock
configure_ulimit() {
  echo "Configuring ulimit for memory lock..."
  ulimit -l unlimited
  if [ $? -eq 0 ]; then
    echo "Ulimit successfully configured."
  else
    echo "Failed to set ulimit. Please ensure you have sufficient permissions."
  fi
}

# Parse JSON configuration file
parse_config() {
  local config_path=$1
  if [ ! -f "${config_path}" ]; then
    echo "Configuration file not found: ${config_path}"
    exit 1
  fi
  cat "${config_path}"
}

# Main script logic
main() {
  read -p "Enter the path to your configuration file: " config_path
  config=$(parse_config "${config_path}")

  if [ -z "${config}" ]; then
    echo "Invalid configuration file."
    exit 1
  fi

  # Extract buffer and optimization settings
  num_rows=$(echo "${config}" | jq '.buffer.num_rows')
  num_cols=$(echo "${config}" | jq '.buffer.num_cols')
  use_huge_pages=$(echo "${config}" | jq '.os_optimization.use_huge_pages')
  lock_memory=$(echo "${config}" | jq '.os_optimization.lock_memory')

  # Calculate memory requirements
  row_size=$((num_cols * 8))  # Assuming f64 (8 bytes per value)
  total_memory=$((num_rows * row_size))
  huge_page_size=$((2 * 1024 * 1024))  # Default huge page size: 2MB
  required_pages=$((total_memory / huge_page_size + 1))

  echo "Buffer Configuration:"
  echo "  Number of Rows: ${num_rows}"
  echo "  Number of Columns: ${num_cols}"
  echo "  Total Memory Required: $(echo "scale=2; ${total_memory} / (1024 * 1024)" | bc) MB"
  echo "  Huge Pages Required: ${required_pages}"

  # Apply huge pages if enabled
  if [ "${use_huge_pages}" = "true" ]; then
    check_hugepages
    set_hugepages "${required_pages}"
  fi

  # Apply memory lock if enabled
  if [ "${lock_memory}" = "true" ]; then
    configure_ulimit
  fi

  echo "Configuration complete. System is ready for the application."
}

# Run the main function
main
