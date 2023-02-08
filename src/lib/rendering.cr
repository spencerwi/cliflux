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
      if @window_handle == nil
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
        @current_screen.handle_char(c, window_handle)
        # TODO: how do I swap screens?
      end
      close
    end

    def close
      if @window_handle != nil
        NCurses.end
        @window_handle = nil
      end
    end
  end

  class QuitProgram < Exception
  end

  abstract class Screen
    abstract def draw_content(window_handle : NCurses::Window)
    abstract def handle_char(c : Char, window_handle : NCurses::Window)

    def quit
      NCurses.end
      raise QuitProgram
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

    def handle_char(c : Char, window_handle : NCurses::Window)
      case c
      when 'q', 'Q' then quit()
      end
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

    def handle_char(c : Char, window_handle : NCurses::Window)
      case c
      when 'q', 'Q' then quit()
      when 'j', 'J' then
        if @selected_index > 0
          @selected_index -= 1
          self.draw_content
        end
      when 'k', 'K' then
        if @selected_index < (@entries.count - 1)
          @selected_index += 1
        end
      end
    end
  end
end
