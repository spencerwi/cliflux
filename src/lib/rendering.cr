require "event_handler"
require "crysterm"
require "./libminiflux.cr"

module Rendering
  class MainWindow
    include EventHandler
    event QuitProgram

    property entries : Array(LibMiniflux::FeedEntry)
    @display : Crysterm::Display
    @screen : Crysterm::Screen
    @current_view : View

    def initialize
      @entries = [] of LibMiniflux::FeedEntry
      @display = Crysterm::Display.new
      @screen = Crysterm::Screen.new(display: @display, show_fps: false)
      @current_view = LoadingView.new
    end

    def run
      # Set up event listeners
      @current_view.on(View::ChangeView) do |evt|
        self.change_view(evt.new_view)
      end
      @display.on(Crysterm::Event::KeyPress) do |evt|
        if evt.char == 'q'
          @display.destroy
          emit QuitProgram
        end
      end
      @current_view.draw_content(@screen)

      # Spawn the main loop over here:
      spawn do
        loop do
          @screen.render
          Fiber.yield
        end
      end

      # And init the display
      @display.exec(@screen)
    end

    def render_feed_entries(entries : Array(LibMiniflux::FeedEntry))
      self.change_view(EntryListView.new(entries))
    end

    private def change_view(new_view)
      @current_view.cleanup(@screen)
      @current_view = new_view
      new_view.draw_content(@screen)
    end
  end

  abstract class View
    include EventHandler
    event ChangeView, new_view : View
    @widgets : Array(Crysterm::Widget) = [] of Crysterm::Widget

    abstract def draw_content(screen : Crysterm::Screen)

    def cleanup(screen : Crysterm::Screen)
      @widgets.each {|w| w.destroy() }
    end
  end

  class LoadingView < View
    def initialize
    end

    def draw_content(screen : Crysterm::Screen)
      loading_spinner = Crysterm::Widget::Loading.new(
        align: Tput::AlignFlag::Center,
        compact: true,
        interval: 0.2.seconds,
        border: Crysterm::Border.new(type: Crysterm::BorderType::Line),
        content: "Loading..."
      )
      screen.append loading_spinner
      loading_spinner.start
      @widgets << loading_spinner
    end
  end

  class EntryListView < View
    def initialize(@entries : Array(LibMiniflux::FeedEntry))
    end

    def draw_content(screen : Crysterm::Screen)
      menu = Crysterm::Widget::List.new(
        name: "Feed items",
        width: "60%",
        top: "center",
        left: "20%",
        track: true,
        border: true,
        input: true
      )
      menu.set_items(@entries.map {|entry| entry.title})
      menu.on(Crysterm::Event::SelectItem) do |evt|
        selected_entry = @entries[evt.index]
        if selected_entry
          read_entry_view = ReadEntryView.new(selected_entry, self)
          emit View::ChangeView, read_entry_view
        end
      end
      screen.append menu
      menu.focus
      @widgets << menu
    end
  end

  class ReadEntryView < View
    def initialize(@entry : LibMiniflux::FeedEntry, @parent : EntryListView)
    end

    def draw_content(screen : Crysterm::Screen)
      text = Crysterm::Widget::ScrollableText.new(
        name: @entry.title,
        width: "half",
        top: "center",
        left: "center",
        content: formatted_entry_text
      )
      screen.append text
      @widgets << text
    end

    private def formatted_entry_text()
      return <<-EOF
      #{@entry.title}
      #{"-" * @entry.title.size}

      #{@entry.content}
      EOF
    end
  end
end
