use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};
fn main() -> io::Result<()> {
	let mut terminal = ratatui::init();
	let node = NodeExt::new("test node", vec![])
		.add_socket(SocketExt::new(
			"named",
			SocketDirection::Input,
			SocketType::Named,
			"",
			false,
		))
		.add_socket(SocketExt::new(
			"select",
			SocketDirection::Input,
			SocketType::Select,
			"selected value",
			false,
		))
		.add_socket(SocketExt::new(
			"switch",
			SocketDirection::Input,
			SocketType::Switch,
			"no repetition",
			false,
		))
		.add_socket(SocketExt::new(
			"number",
			SocketDirection::Input,
			SocketType::Number,
			"10",
			false,
		))
		.add_socket(SocketExt::new(
			"text",
			SocketDirection::Input,
			SocketType::Text,
			"hello world!",
			false,
		))
		.add_socket(SocketExt::new(
			"output",
			SocketDirection::Output,
			SocketType::Named,
			"",
			false,
		));
	let app_result = App::new(node).run(&mut terminal);
	ratatui::restore();
	app_result
}

#[derive(Debug)]
pub struct App {
	node_ext: NodeExt,
	exit: bool,
}

impl App {
	fn new(node_ext: NodeExt) -> Self {
		App {
			node_ext,
			exit: false,
		}
	}
}

#[derive(Debug, Clone)]
pub enum SocketDirection {
	Input,
	Output,
	None,
}

#[derive(Debug, Clone)]
pub enum SocketType {
	Named,
	Text,
	Number,
	Select,
	Switch,
}

#[derive(Debug, Clone)]
pub struct SocketExt {
	pub name: String,
	pub direction: SocketDirection,
	pub socket_type: SocketType,
	pub value: String,
	pub is_repetitive: bool,
}

pub fn format_text_left(mut string: String, max_len: usize) -> String {
	if string.len() > max_len {
		string = string[0..max_len - 1].to_owned()
	}
	string.insert(0, ' ');
	string
}

pub fn format_text_right(mut string: String, max_len: usize) -> String {
	if string.len() > max_len {
		string = string[0..max_len - 1].to_owned()
	}
	for _ in 0..max_len - string.len() - 1 {
		string.insert(0, ' ');
	}
	string.push(' ');
	string
}

impl SocketExt {
	pub fn new<T>(
		name: T,
		direction: SocketDirection,
		socket_type: SocketType,
		value: T,
		is_repetitive: bool,
	) -> Self
	where
		T: Into<String>,
	{
		SocketExt {
			name: name.into(),
			direction,
			socket_type,
			value: value.into(),
			is_repetitive,
		}
	}

	pub fn render(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		match self.direction {
			SocketDirection::Input => {
				if let Some(mut cell) = buf.cell_mut((0, y)) {
					cell.set_fg(ratatui::style::Color::White).set_char('⬤');
				}
			}
			SocketDirection::Output => {
				if let Some(mut cell) = buf.cell_mut((31, y)) {
					cell.set_fg(ratatui::style::Color::White).set_char('⬤');
				}
			}
			SocketDirection::None => (),
		}

		match self.socket_type {
			SocketType::Named => self.render_named(buf, port_index),
			SocketType::Text => self.render_text(buf, port_index),
			SocketType::Select => self.render_select(buf, port_index),
			SocketType::Switch => self.render_switch(buf, port_index),
			SocketType::Number => self.render_number(buf, port_index),
		}
	}

	pub fn render_named(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		for x in 1..31 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::Black)
					.set_fg(ratatui::style::Color::White);
			}
		}
		let mut formated_name = match self.direction {
			SocketDirection::Input => format_text_left(self.name.clone(), 30),
			SocketDirection::Output => format_text_right(self.name.clone(), 30),
			SocketDirection::None => format_text_left(self.name.clone(), 30),
		};
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}
	}

	pub fn render_switch(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		for x in 1..12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::Black)
					.set_fg(ratatui::style::Color::White);
			}
		}
		for x in 12..31 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::White)
					.set_fg(ratatui::style::Color::Black);
			}
		}
		let mut formated_name = format_text_left(self.name.clone(), 12);
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}

		let mut formated_value = format_text_left(self.value.clone(), 19);
		formated_value.insert(0, '☼');

		for x in 12u16..formated_value.len() as u16 + 12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_value.chars().nth((x - 12) as usize).unwrap_or(' '));
			}
		}
	}

	pub fn render_select(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		for x in 1..12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::Black)
					.set_fg(ratatui::style::Color::White);
			}
		}
		for x in 12..31 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::White)
					.set_fg(ratatui::style::Color::Black);
			}
		}
		let mut formated_name = format_text_left(self.name.clone(), 12);
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}

		let mut formated_value = format_text_left(self.value.clone(), 19);
		formated_value.insert(0, '☰');

		for x in 12u16..formated_value.len() as u16 + 12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_value.chars().nth((x - 12) as usize).unwrap_or(' '));
			}
		}
	}

	pub fn render_text(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		for x in 1..12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::Black)
					.set_fg(ratatui::style::Color::White);
			}
		}
		for x in 12..31 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::White)
					.set_fg(ratatui::style::Color::Black);
			}
		}
		let mut formated_name = format_text_left(self.name.clone(), 12);
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}

		let mut formated_value = format_text_left(self.value.clone(), 20);

		for x in 12u16..formated_value.len() as u16 + 12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_value.chars().nth((x - 12) as usize).unwrap_or(' '));
			}
		}
	}

	pub fn render_number(&self, buf: &mut Buffer, port_index: u8) {
		let y = (4 + port_index * 2) as u16;
		for x in 1..12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::Black)
					.set_fg(ratatui::style::Color::White);
			}
		}
		for x in 12..31 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_bg(ratatui::style::Color::White)
					.set_fg(ratatui::style::Color::Black);
			}
		}
		let mut formated_name = format_text_left(self.name.clone(), 12);
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}

		let mut formated_value = format_text_left(self.value.clone(), 20);

		for x in 12u16..formated_value.len() as u16 + 12 {
			if let Some(mut cell) = buf.cell_mut((x, y)) {
				cell.set_char(formated_value.chars().nth((x - 12) as usize).unwrap_or(' '));
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct NodeExt {
	name: String,
	sockets: Vec<SocketExt>,
}

impl NodeExt {
	pub fn new<T>(name: T, sockets: Vec<SocketExt>) -> Self
	where
		T: Into<String>,
	{
		NodeExt {
			name: name.into(),
			sockets,
		}
	}

	pub fn add_socket(&mut self, socket: SocketExt) -> Self {
		self.sockets.push(socket);
		self.clone()
	}

	pub fn render(&self, buf: &mut Buffer) {
		for y in 0..(4 + (self.sockets.len() * 2) as u16) {
			for x in 0..32 {
				if let Some(mut cell) = buf.cell_mut((x, y)) {
					cell.set_bg(ratatui::style::Color::DarkGray);
				}
			}
		}
		for x in 1..31 {
			if let Some(mut cell) = buf.cell_mut((x, 1)) {
				cell.set_bg(ratatui::style::Color::White)
					.set_fg(ratatui::style::Color::Black);
			}
		}
		let mut formated_name = self.name.clone();
		if formated_name.len() > 30 {
			formated_name = formated_name[0..30].to_owned()
		}
		for _ in 0..((30.0 - formated_name.len() as f32) / 2.0).floor() as u16 {
			formated_name.insert(0, ' ');
		}
		for x in 1u16..formated_name.len() as u16 + 1 {
			if let Some(mut cell) = buf.cell_mut((x, 1)) {
				cell.set_char(formated_name.chars().nth((x - 1) as usize).unwrap_or(' '));
			}
		}

		for (i, socket) in self.sockets.iter().enumerate() {
			socket.render(buf, i as u8);
		}
	}
}

impl App {
	/// runs the application's main loop until the user quits
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	fn draw(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.area());
	}

	fn handle_events(&mut self) -> io::Result<()> {
		match event::read()? {
			// it's important to check that the event is a key press event as
			// crossterm also emits key release and repeat events on Windows.
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			}
			_ => {}
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('q') => self.exit(),
			_ => {}
		}
	}

	fn exit(&mut self) {
		self.exit = true;
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		self.node_ext.render(buf);
	}
}

