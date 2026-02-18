# NixOS configuration for sovereign deployment of rust-learning-ground
# The entire server declared in one file -- reproducible bit-for-bit
#
# Usage:
#   1. Copy this to /etc/nixos/configuration.nix on your NixOS server
#   2. Replace placeholder keys/domains with your own
#   3. Run: sudo nixos-rebuild switch

{ config, pkgs, ... }:
{
  # -- Immutable base -------------------------------------------------------
  system.stateVersion = "24.05";
  boot.loader.grub.enable = true;

  # -- No password auth, no root login --------------------------------------
  services.openssh = {
    enable = true;
    settings = {
      PasswordAuthentication = false;
      PermitRootLogin = "no";
      X11Forwarding = false;
    };
  };

  # -- Firewall: deny everything, whitelist explicitly -----------------------
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [ 80 443 ];    # Only Caddy faces internet
    allowedUDPPorts = [ 51820 ];     # WireGuard
  };

  # -- WireGuard inter-service mesh -----------------------------------------
  networking.wg-quick.interfaces.wg0 = {
    address = [ "10.0.1.1/24" ];
    privateKeyFile = "/run/secrets/wg_private_key";
    peers = [
      { publicKey = "APP_VM_PUBKEY"; allowedIPs = [ "10.0.1.2/32" ]; }
      { publicKey = "DB_VM_PUBKEY";  allowedIPs = [ "10.0.1.3/32" ]; }
    ];
  };

  # -- nsjail for code execution sandbox ------------------------------------
  environment.systemPackages = with pkgs; [
    nsjail
    rustup
    podman
    caddy
    age
    sops
  ];

  # -- Podman: rootless containers, no daemon --------------------------------
  virtualisation.podman = {
    enable = true;
    dockerCompat = true;
    defaultNetwork.settings.dns_enabled = true;
  };

  # -- AppArmor mandatory access control ------------------------------------
  security.apparmor.enable = true;

  # -- Automatic security updates (unattended) ------------------------------
  system.autoUpgrade = {
    enable = true;
    allowReboot = false;
    dates = "04:00";
  };
}
