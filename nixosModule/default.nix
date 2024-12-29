self:
{
  lib,
  config,
  pkgs,
  ...
}:
with lib;
let
  cfg = config.services.cf-ddns;
  pkg = self.defaultPackage.${pkgs.system};
in
{
  options.services.cf-ddns = with types; {
    enable = mkEnableOption "cf-ddns";

    domain = mkOption {
      type = nullOr str;
      default = null;
      example = "example.com";
      description = ''
        Cloudflare domain to update.
        Ignored if the option domainFile is set.
      '';
    };

    domainFile = mkOption {
      type = nullOr path;
      default = null;
      example = /run/secrets/domain;
      description = ''
        Path to a file containing the cloudflare domain to update.
        Override domain option.
      '';
    };

    email = mkOption {
      type = nullOr str;
      default = null;
      example = "john@doe.com";
      description = ''
        Email address used to query cloudflare.
        Ignored if the option emailFile is set.
      '';
    };

    emailFile = mkOption {
      type = nullOr path;
      default = null;
      example = /run/secrets/email;
      description = ''
        Path to a file containing the email address used to query cloudflare.
        Override email option.
      '';
    };

    token = mkOption {
      type = nullOr str;
      default = null;
      example = "cloudflaretoken";
      description = ''
        Token used to authenticate with cloudflare api.
        The option tokenFile should be prefered for security reason.
        Ignored if the options tokenFile is set.
      '';
    };

    tokenFile = mkOption {
      type = nullOr path;
      default = null;
      example = /run/secrets/cf-token;
      description = ''
        Path to a file containing the token used to authenticate with cloudflare api.
        Override token option.
      '';
    };

    environmentFile = mkOption {
      type = nullOr path;
      default = null;
      example = /run/secrets/cf.env;
      description = ''
        Path to a file containing environment variables to overrides domain, email and token options : CF_DOMAIN, CF_EMAIL and CF_TOKEN respectively.
      '';
    };

    onCalendar = mkOption {
      type = nullOr str;
      default = "daily";
      example = "*-*-* *:00:00";
      description = ''
        How often the service should try to update cloudflare domain. See systemd.time(7) for more information about the format.
        Setting it to null disables the timer, thus this instance can only be started manually. 
      '';
    };

  };

  config = mkIf cfg.enable {
    environment.systemPackages = [ pkg ];

    systemd.services.cf-ddns = {
      description = "Update cloudflare dns record ip.";
      serviceConfig =
        let
          domain =
            if cfg.domainFile != null then
              ''-D ${cfg.domainFile}''
            else
              optionalString (cfg.domain != null) ''-d ${cfg.domain}'';
          email =
            if cfg.emailFile != null then
              ''-E ${cfg.emailFile}''
            else
              optionalString (cfg.email != null) ''-e ${cfg.email}'';
          token =
            if cfg.tokenFile != null then
              ''-T ${cfg.tokenFile}''
            else
              optionalString (cfg.token != null) ''-t ${cfg.token}'';
        in
        {
          Type = "oneshot";
          RemainAfterExit = "yes";
          ExecStart = ''
            ${pkg}/bin/cf-ddns ${domain} ${email} ${token} 
          '';
        }
        // (optionalAttrs (cfg.environmentFile != null) { EnvironmentFile = cfg.environmentFile; });
    };

    systemd.timers.cf-ddns = {
      description = "Timer to update cloudflare dns record ip.";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnCalendar = cfg.onCalendar;
        Unit = "cf-ddns.service";
        Persistent = true;
      };
    };

  };

}
