packer {
  required_plugins {
    vmware = {
      source  = "github.com/hashicorp/vmware"
      version = ">= 1.1.0"
    }
    vagrant = {
      source  = "github.com/hashicorp/vagrant"
      version = ">= 1.1.5"
    }
  }
}

variable "iso_path" {
  type        = string
  description = "Local Windows 11 Arm ISO path."
}

variable "iso_checksum" {
  type        = string
  description = "Windows 11 Arm ISO checksum, for example sha256:<digest>."
}

variable "autounattend_path" {
  type        = string
  description = "Generated autounattend.xml path outside the repository."
}

variable "output_directory" {
  type        = string
  description = "Packer output directory outside the repository."
}

variable "vm_name" {
  type        = string
  description = "Base VM name created by Packer before Vagrant box packaging."
  default     = "frb-windows11-arm64-base"
}

variable "box_output_path" {
  type        = string
  description = "Final Vagrant box path outside the transient VMware output directory."
}

variable "guest_username" {
  type        = string
  description = "Windows local administrator user created by autounattend."
  default     = "frb"
}

variable "guest_password" {
  type        = string
  description = "Windows local administrator password. Prefer PKR_VAR_guest_password from a secret source."
  sensitive   = true
}

variable "guest_proxy_url" {
  type        = string
  description = "Optional proxy URL reachable from inside the Windows guest."
  default     = ""
}

variable "winrm_host" {
  type        = string
  description = "Guest WinRM host for Packer. Defaults to the first VMware Fusion NAT lease."
}

variable "headless" {
  type        = bool
  description = "Run the VMware builder without showing the VM console."
  default     = true
}

source "vmware-iso" "windows11_arm64" {
  vm_name          = var.vm_name
  iso_url          = var.iso_path
  iso_checksum     = var.iso_checksum
  output_directory = var.output_directory

  guest_os_type     = "arm-windows11-64"
  headless          = var.headless
  cpus              = 6
  memory            = 16384
  disk_size         = 180000
  disk_adapter_type = "nvme"
  disk_type_id      = "0"

  network_adapter_type = "vmxnet3"

  boot_wait = "10s"
  boot_command = [
    "<down><down><enter>",
    "<wait1><spacebar>",
  ]

  vmx_data = {
    "scsi0.virtualDev" = "pvscsi"
  }

  communicator   = "winrm"
  winrm_username = var.guest_username
  winrm_password = var.guest_password
  winrm_host     = var.winrm_host
  winrm_timeout  = "2h"

  floppy_files = [
    var.autounattend_path,
  ]

  cd_files = [
    var.autounattend_path,
  ]
  cd_label = "AUTOUNATTEND"

  shutdown_command = "powershell -NoProfile -Command \"Stop-Computer -Force\""
}

build {
  name    = "frb-windows11-arm64"
  sources = ["source.vmware-iso.windows11_arm64"]

  provisioner "powershell" {
    elevated_user     = var.guest_username
    elevated_password = var.guest_password

    environment_vars = [
      "FRB_WINDOWS_GUEST_PROXY_URL=${var.guest_proxy_url}",
    ]
    scripts = [
      "../provision/prepare-guest-control.ps1",
      "../provision/install-frb-toolchain.ps1",
    ]
  }

  post-processor "vagrant" {
    output = var.box_output_path
  }
}
