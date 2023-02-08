require "toml"
require "xdg_basedir"
require "./lib/libminiflux.cr"

module Cliflux
  VERSION = "0.1.0"

  class Main
    def self.get_config()
      config_path = XDGBasedir.full_path("cliflux/config.toml", :config, :read)
      if !(config_path) || !(File.exists?(config_path))
        puts "You need to provide a configuration file in your XDG base configuration directory (try something like ~/.config/cliflux/config.toml)"
        exit 1
      end
      raw_contents = File.read(config_path)
      return TOML.parse(raw_contents)
    end

    def self.run()
      config = get_config()
      client = LibMiniflux::Client.new(config["url"].as(String), config["api_key"].as(String))
      entries = client.get_unread_entries(10, 0)
      puts entries.to_json
    end
  end

end

Cliflux::Main.run()
