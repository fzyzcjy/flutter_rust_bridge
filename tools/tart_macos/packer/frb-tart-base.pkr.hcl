packer {
  required_plugins {
    tart = {
      source  = "github.com/cirruslabs/tart"
      version = ">= 1.20.0"
    }
  }
}

variable "source_vm" {
  type        = string
  description = "Pinned upstream or locally mirrored Tart OCI VM used as the raw source."
  default     = "127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1"
}

variable "target_vm" {
  type        = string
  description = "Local Tart VM name to create. Use frb-tart-base-candidate first, then promote after smoke."
  default     = "frb-tart-base-candidate"
}

variable "allow_insecure" {
  type        = bool
  description = "Allow cloning from a local HTTP OCI registry such as 127.0.0.1:5000."
  default     = true
}

variable "flutter_version" {
  type        = string
  description = "Flutter SDK version to install in the Tart base. Keep this in sync with FRB_MAIN_FLUTTER_VERSION in CI."
  default     = "3.44.0"
}

variable "host_proxy_url" {
  type        = string
  description = "Optional host proxy URL reachable from the Tart VM, for example http://192.168.64.1:7897."
  default     = ""
}

source "tart-cli" "frb_tart_base" {
  vm_base_name   = var.source_vm
  vm_name        = var.target_vm
  allow_insecure = var.allow_insecure

  cpu_count    = 4
  memory_gb    = 16
  disk_size_gb = 140

  ssh_username = "admin"
  ssh_password = "admin"
  ssh_timeout  = "20m"
}

build {
  name    = "frb-tart-base"
  sources = ["source.tart-cli.frb_tart_base"]

  provisioner "shell" {
    script = "scripts/provision-frb-tart-base.sh"
    environment_vars = concat(
      [
        "FRB_TART_FLUTTER_VERSION=${var.flutter_version}",
      ],
      var.host_proxy_url == "" ? [] : [
        "http_proxy=${var.host_proxy_url}",
        "https_proxy=${var.host_proxy_url}",
        "all_proxy=${var.host_proxy_url}",
        "HTTP_PROXY=${var.host_proxy_url}",
        "HTTPS_PROXY=${var.host_proxy_url}",
        "ALL_PROXY=${var.host_proxy_url}",
        "CARGO_HTTP_PROXY=${var.host_proxy_url}",
        "NO_PROXY=localhost,127.0.0.1,::1",
        "no_proxy=localhost,127.0.0.1,::1",
      ],
    )
    execute_command = "chmod +x {{ .Path }}; {{ .Vars }} {{ .Path }}"
  }
}
