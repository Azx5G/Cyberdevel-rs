import platform
import subprocess

def install_rust():
    os_name = platform.system().lower()
    try:
        if os_name == "windows":
            # On Windows, PowerShell is used to download and execute the rustup script
            subprocess.run(["powershell", "-Command",
                            "(Invoke-WebRequest -Uri https://sh.rustup.rs -UseBasicParsing).Content | sh"], check=True)
        elif os_name in ["linux", "darwin"]:
            # On Linux and macOS, we use curl to download and execute the rustup script
            subprocess.run(["curl", "--proto", "=https", "--tlsv1.2", "-sSf", "https://sh.rustup.rs", "|", "sh"], shell=True, check=True)
        else:
            print(f"Unsupported operating system: {os_name}")
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while installing Rust: {e}")

if __name__ == "__main__":
    install_rust()