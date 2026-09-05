{
  pkgs ? import <nixpkgs> { },
  bundlePath ? ../../frb_example/flutter_via_create/build/linux/x64/debug/bundle,
}:

let
  bundle = builtins.path {
    path = bundlePath;
    name = "flutter-via-create-bundle";
  };
  runtimeLibraryPath = pkgs.lib.makeLibraryPath (
    with pkgs;
    [
      at-spi2-core
      cairo
      gdk-pixbuf
      glib
      gtk3
      harfbuzz
      libepoxy
      pango
      stdenv.cc.cc.lib
    ]
  );
in
pkgs.testers.runNixOSTest {
  name = "flutter-rust-bridge-quickstart";
  globalTimeout = 600;

  nodes.machine = { pkgs, ... }: {
    environment.systemPackages = with pkgs; [
      imagemagick
      tesseract
      xorg-server
    ];
    fonts.packages = [ pkgs.dejavu_fonts ];
    hardware.graphics.enable = true;
    virtualisation = {
      cores = 4;
      diskSize = 8192;
      graphics = false;
      memorySize = 6144;
    };
  };

  testScript = ''
    def quickstart_running():
        machine.succeed("systemctl is-active frb-quickstart.service")

    machine.start()
    machine.wait_for_unit("multi-user.target")
    machine.succeed("test -e /etc/NIXOS")
    machine.succeed(
        "systemd-run --unit=frb-xvfb ${pkgs.xorg-server}/bin/Xvfb "
        ":99 -screen 0 1280x1024x24"
    )
    machine.wait_until_succeeds("test -S /tmp/.X11-unix/X99")
    machine.succeed(
        "systemd-run --unit=frb-quickstart "
        "--property=Environment=DISPLAY=:99 "
        "--property=Environment=LIBGL_ALWAYS_SOFTWARE=1 "
        "--property=Environment=LD_LIBRARY_PATH=${runtimeLibraryPath}:/run/opengl-driver/lib "
        "${bundle}/flutter_via_create"
    )
    try:
        machine.wait_for_unit("frb-quickstart.service", timeout=30)
        with polling_condition(
            quickstart_running,
            description="check that the Flutter app is running",
        ):
            machine.wait_until_succeeds(
                "DISPLAY=:99 import -window root /tmp/quickstart.png && "
                "convert /tmp/quickstart.png -resize 300% -colorspace Gray "
                "-normalize /tmp/quickstart-processed.png && "
                "tesseract /tmp/quickstart-processed.png /tmp/quickstart && "
                "grep -Eiq 'hello' /tmp/quickstart.txt && "
                "grep -Eiq 'tom' /tmp/quickstart.txt",
                timeout=120,
            )
    except Exception:
        print(machine.execute("systemctl status --no-pager frb-quickstart.service")[1])
        print(machine.execute("journalctl -u frb-quickstart.service --no-pager")[1])
        raise
    machine.copy_from_machine("/tmp/quickstart.png", "")
    machine.copy_from_machine("/tmp/quickstart.txt", "")
    machine.succeed("systemctl stop frb-quickstart.service")
  '';
}
