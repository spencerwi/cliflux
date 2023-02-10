require "event_handler"
require "hydra"
require "./libminiflux.cr"


## TODO: rewrite to match this example: https://github.com/Ghrind/hydra/blob/master/examples/file_manager.cr

module Rendering
  class MainWindow < Hydra::Application
    include EventHandler
    event QuitProgram

    property entries : Array(LibMiniflux::FeedEntry)
    @current_view : View

    def initialize
      @entries = [] of LibMiniflux::FeedEntry
      @current_view = LoadingView.new
    end

    def start
      # Set up event listeners
      @current_view.on(View::ChangeView) do |evt|
        spawn do
          self.change_view(evt.new_view)
        end
      end
      self.bind("keypress.q") do |event_hub|
        event_hub.trigger("application", "stop")
        emit QuitProgram
        true
      end
      @current_view.show(self, @event_hub)

      # And init the display
      self.run
      self.teardown
    end

    def render_feed_entries(entries : Array(LibMiniflux::FeedEntry))
      self.change_view(EntryListView.new(entries))
    end

    private def change_view(new_view)
      @current_view.cleanup(self)
      @current_view = new_view
      new_view.show(self, @event_hub)
    end
  end

  abstract class View
    include EventHandler
    event ChangeView, new_view : View
    @widgets : Array(Hydra::Element) = Array(Hydra::Element).new

    abstract def setup(app : Hydra::Application, event_hub : Hydra::EventHub)
    abstract def focus_target : Hydra::Element

    def has_been_setup?
      !(@widgets.empty?)
    end

    def show(app : Hydra::Application, event_hub : Hydra::EventHub)
      if has_been_setup?
        @widgets.each {|w| w.show() }
      else
        setup(app, event_hub)
      end
      event_hub.focus(focus_target.id)
    end

    def cleanup(app : Hydra::Application)
      @widgets.each {|w| w.hide() }
    end
  end

  class LoadingView < View
    def initialize
    end

    def setup(app : Hydra::Application, event_hub : Hydra::EventHub)
      loader = Hydra::Text.new("loader_text", {
        :value => "Loading...",
        :position => "center"
      })
      app.add_element(loader)
      @widgets << loader
    end

    def focus_target : Hydra::Element
      @widgets[0]
    end
  end

  class EntryListView < View
    def initialize(@entries : Array(LibMiniflux::FeedEntry))
    end

    def setup(app : Hydra::Application, event_hub : Hydra::EventHub)
      list = Hydra::List.new("feed_entry_list", {
        :label => "Unread",
        :position => "center"
      })
      @entries.each {|entry| list.add_item(entry.title)}
      app.add_element(list)
      app.bind("keypress.j") do |event_hub|
        list.trigger("select_down")
        true
      end
      app.bind("keypress.k") do |event_hub|
        list.trigger("select_up")
        true
      end
      app.bind("keypress.enter") do |event_hub|
        spawn do
          entry = @entries[list.selected]
          entry_view = ReadEntryView.new(entry, self)
          emit ChangeView, entry_view
        end
        true
      end
      @widgets << list
    end

    def focus_target : Hydra::Element
      @widgets[0]
    end
  end

  class ReadEntryView < View
    def initialize(@entry : LibMiniflux::FeedEntry, @parent : EntryListView)
    end

    def setup(app : Hydra::Application, event_hub : Hydra::EventHub)
      text = Hydra::Text.new("text-#{@entry.feed_id}-#{@entry.id}", {
        :label => @entry.title,
        :value => formatted_entry_text,
        :position => "center"
      })
      app.add_element(text)
      @widgets << text
    end

    private def formatted_entry_text()
      return <<-EOF
      #{@entry.title}
      #{"-" * @entry.title.size}

      #{@entry.content}
      EOF
    end

    def focus_target : Hydra::Element
      @widgets[0]
    end
  end
end
