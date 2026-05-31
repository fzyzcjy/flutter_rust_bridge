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
    script          = "scripts/provision-frb-tart-base.sh"
    execute_command = "chmod +x {{ .Path }}; {{ .Vars }} {{ .Path }}"
  }
}
