require "ncurses"
require "./libminiflux.cr"

module Rendering
  class MainWindow
    property entries : Array(LibMiniflux::FeedEntry)
    @window_handle : NCurses::Window?
    @current_screen : Screen

    def initialize
      @window_handle = nil
      @entries = [] of LibMiniflux::FeedEntry
      @current_screen = LoadingScreen.new
    end

    def start
      NCurses.start
      NCurses.cbreak
      NCurses.no_echo
      return NCurses::Window.new(NCurses.height, NCurses.width)
    end

    private def window_handle : NCurses::Window
      if @window_handle.nil?
          @window_handle = self.start
      end
      return @window_handle.not_nil!
    end

    def draw_loading
      window_handle.clear()
      window_handle.print("Loading...")
      window_handle.refresh()
    end

    def draw_entries
      window_handle.clear()
      window_handle.print(
        @entries.map {|entry| entry.title}.join("\n")
      )
      # TODO: figure out how to center text
      window_handle.refresh()
    end

    def main_loop
      @current_screen.draw_content(window_handle)
      window_handle.get_char do |c|
        maybe_next_screen = @current_screen.handle_char(c, window_handle)
        if maybe_next_screen
            @current_screen = maybe_next_screen
        end
      end
      close
    end

    def close
      if @window_handle
        NCurses.end
        @window_handle = nil
      end
    end
  end

  class QuitProgram < Exception
  end

  abstract class Screen
    abstract def draw_content(window_handle : NCurses::Window)
    abstract def handle_char(c : Char|LibNCurses::Key|Nil, window_handle : NCurses::Window) : Screen?

    def quit
      NCurses.end
      raise QuitProgram.new
    end
  end

  class LoadingScreen < Screen
    def initialize
    end

    def draw_content(window_handle : NCurses::Window)
      window_handle.clear()
      window_handle.print("Loading...")
      window_handle.refresh()
    end

    def handle_char(c : Char|LibNCurses::Key|Nil, window_handle : NCurses::Window) : Screen?
      case c
      when 'q', 'Q' then quit()
      end
      nil
    end
  end

  class EntryListScreen < Screen
    def initialize(@entries : Array(LibMiniflux::FeedEntry))
      @selected_index = 0
    end

    def draw_content(window_handle : NCurses::Window)
      window_handle.clear()
      window_handle.print(
        @entries.map {|entry| entry.title}.join("\n")
      )
      # TODO: figure out how to center text
      # TODO: highlight current selection
      window_handle.refresh()
    end

    def handle_char(c : Char|LibNCurses::Key|Nil, window_handle : NCurses::Window) : Screen?
      case c
      when 'q', 'Q' then quit()
      when 'j', 'J' then
        if @selected_index > 0
          @selected_index -= 1
          self.draw_content(window_handle)
        end
      when 'k', 'K' then
        if @selected_index < (@entries.size - 1)
          @selected_index += 1
          self.draw_content(window_handle)
        end
      when '\n', '\r', LibNCurses::Key::Enter then
        return ReadEntryScreen.new(@entries[@selected_index], self)
      end
      return nil
    end
  end

  class ReadEntryScreen < Screen
    def initialize(@entry : LibMiniflux::FeedEntry, @parent : EntryListScreen)
    end

    def draw_content(window_handle : NCurses::Window)
      window_handle.clear()
      window_handle.print(formatted_entry_text)
      window_handle.refresh()
    end

    def handle_char(c : Char|LibNCurses::Key|Nil, window_handle : NCurses::Window) : Screen?
      case c
      when 'q', 'Q' then quit()
      when LibNCurses::Key::Left, 'b', 'B', LibNCurses::Key::Esc then
        return @parent
      end
      return nil
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
