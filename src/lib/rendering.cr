require "hydra"
require "textwrap"
require "./libminiflux.cr"

module Rendering
  enum Views
    Loading
    FeedEntries
    ReadEntry
  end

  class MainWindow < Hydra::Application
    @feed_entries : Array(LibMiniflux::FeedEntry) = Array(LibMiniflux::FeedEntry).new

    def start
      # Set up event listeners
      self.bind("keypress.q") do |event_hub|
        event_hub.trigger("application", "stop")
        true
      end
      self.show_loading_view()

      # Then start the event loop
      self.run

      # And when the event loop gets the "application","quit" event, it'll stop, and we'll hit this point
      self.teardown
    end

    def show_loading_view
      self.current_view = Views::Loading
      self.hide_all_elements
      if try_element_by_id("loading-text")
        @elements.by_id("loading-text").show()
      else
        loading_text = Hydra::Text.new("loading-text", {
          :value => "Loading...",
          :position => "center"
        })
        self.add_element(loading_text)
      end
      self.update_screen
    end

    def show_feed_entries_view(entries : Array(LibMiniflux::FeedEntry)? = nil)
      self.current_view = Views::FeedEntries
      self.hide_all_elements
      if !(entries.nil?)
        @feed_entries = entries
      end
      feed_entry_list = try_element_by_id("feed-entries-list").as(Hydra::List?)
      if feed_entry_list.nil?
        feed_entry_list = Hydra::List.new("feed-entries-list", {
          :label => "unread",
          :position => "center",
          :width => [@screen.width, @feed_entries.map(&.title.size).max + 3].min.to_s
        })
        self.add_element(feed_entry_list)
        self.bind("feed-entries-list", "keypress.j") do |event_hub|
          feed_entry_list.select_down
          true
        end
        self.bind("feed-entries-list", "keypress.k") do |event_hub|
          feed_entry_list.select_up
          true
        end
        self.bind("feed-entries-list", "keypress.enter") do |event_hub|
          entry = @feed_entries[feed_entry_list.selected]
          self.show_read_entry_view(entry)
          true
        end
      end
      feed_entry_list.clear()
      @feed_entries.each {|entry| feed_entry_list.add_item(entry.title)}
      feed_entry_list.show()
      @event_hub.focus(feed_entry_list.id)
      self.update_screen
    end

    def show_read_entry_view(entry : LibMiniflux::FeedEntry)
      self.current_view = Views::ReadEntry
      self.hide_all_elements
      read_entry_text = try_element_by_id("read-entry-text")
      if read_entry_text.nil?
        read_entry_text = Hydra::Text.new("read-entry-text", {
          :label => entry.title,
          :value => self.format_entry_text(entry),
          :width => "80",
          :height => @screen.height.to_s,
          :position => "center"
        })
        self.add_element(read_entry_text)
        self.bind("read-entry-text", "keypress.b") do |event_hub|
          self.show_feed_entries_view()
          true
        end
      else
        read_entry_text.value = self.format_entry_text(entry)
      end
      read_entry_text.show()
      @event_hub.focus(read_entry_text.id)
    end

    private def try_element_by_id(id : String) : Hydra::Element?
      @elements.to_a.find {|el| el.id == id}
    end

    private def format_entry_text(entry : LibMiniflux::FeedEntry)
      wrap_width = [@screen.width, 80].min
      return <<-EOF
        #{entry.title.wrap(wrap_width)}
        #{"-" * [entry.title.size, 80].min}

        #{entry.content.wrap(wrap_width)}
      EOF
    end

    private def current_view : Views
      Views.parse?(@state["current_view"]) || Views::Loading
    end
    private def current_view=(view : Views)
      @state["current_view"] = view.to_s
    end

    private def hide_all_elements
      @elements.each {|e| e.hide()}
    end
  end
end
