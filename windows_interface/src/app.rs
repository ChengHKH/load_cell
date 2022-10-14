use serialport;
use tui;

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(vec!["Scale", "Logger", "Options", "Quit"]),
            should_quit: false,
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_enter(&mut self) {
        self.tabs.select();
    }

    pub fn on_tick(&mut self) {
        
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub current_index: usize,
    pub new_index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState {titles, current_index: 0, new_index: 0}
    }

    pub fn next(&mut self) {
        self.new_index = (self.new_index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.new_index > 0 {
            self.new_index -= 1;
        } else {
            self.new_index = self.titles.len() -1;
        }
    }

    pub fn select(&mut self) {
        self.current_index = self.new_index;
    }
}