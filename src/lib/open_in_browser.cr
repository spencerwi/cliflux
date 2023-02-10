class OpenInBrowser
  def self.open(url : String)
    {% if flag?(:darwin) %}
      `open '#{url}'`
    {% elsif flag?(:windows) %}
      `start "#{url}"`
    {% else %}
      `xdg-open '#{url}'`
    {% end %}
  end
end
