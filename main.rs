use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Widget;

#[derive(Default)]
struct App {
	x_shift: i32,
	y_shift: i32,
	nodes: HashMap<usize, Node>,
}
impl App {
	fn run(mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
		loop {
			// Render
			terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
			// Wait for events
			if let Event::Key(k) = event::read()? {
				if k.kind == KeyEventKind::Press && k.code == KeyCode::Char('q') {
					return Ok(())
				}

				match k.code {
					KeyCode::Left => self.x_shift += 1,
					KeyCode::Right => self.x_shift -= 1,
					KeyCode::Up => self.y_shift += 1,
					KeyCode::Down => self.y_shift -= 1,
					_ => (),
				}
			}
		}
	}

	fn create_node(&mut self, name: &str) -> usize {
		let id = self.nodes.len();
		let new_node = Node {
			name: name.to_owned(),
			x_pos: 0,
			y_pos: 0,
			id,
			sockets: vec!(),
			connections: vec!(),
		};
		self.nodes.insert(id, new_node);
		id
	}
	fn get_node(&self, id: usize) -> Option<&Node> {
		self.nodes.get(&id)
	}
	fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
		self.nodes.get_mut(&id)
	}
}
impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {

		// Clear buffer
		for x in 0..area.x {
			for y in 0..area.y {
				if let Some(cell) = buf.cell_mut((x, y)) {
					cell.set_char(' ');
					cell.set_fg(Color::default());
					cell.set_bg(Color::default());
				}
			}
		}

		// Draw nodes with offsets
		for (_, node) in &self.nodes {
			node.draw(self.x_shift, self.y_shift, area, buf);
		}
	}
}

struct Connection {
	from_node: usize,
	from_socket: usize,
	to_socket: usize,
}

struct Socket {
	name: String,
	input: bool,
}

struct Node {
	name: String,
	x_pos: u16,
	y_pos: u16,
	id: usize,
	sockets: Vec<Socket>,
	connections: Vec<Connection>,
}
impl Node {
	fn set_pos(&mut self, x: u16, y: u16) {
		self.x_pos = x;
		self.y_pos = y;
	}
	fn get_pos(&self) -> (u16, u16) {
		(self.x_pos, self.y_pos)
	}

	fn add_socket(&mut self, socket: Socket) {
		self.sockets.push(socket);
	}
	fn get_sockets(&self) -> &Vec<Socket> {
		&self.sockets
	}

	fn add_connection(&mut self, from_node: usize, from_socket: usize, to_socket: usize) {
		self.connections.push(Connection { from_node, from_socket, to_socket });
	}
	fn get_connections(&self) -> &Vec<Connection> {
		&self.connections
	}

	fn draw(&self, x_shift: i32, y_shift: i32, bounds: Rect, buf: &mut Buffer) {
		// The name and each socket's name is a line
		let mut lines = vec!();
		lines.push(Line::from(self.name.clone()).white().bold());
		for socket in &self.sockets {
			lines.push(Line::from(socket.name.clone()).red());
		}

		let text = Text::from(lines).centered().bg(Color::Rgb(40,40,40));
		let x_pos = self.x_pos as i32 + x_shift;
		let y_pos = self.y_pos as i32 + y_shift;
		let width = text.width() as i32;
		let height = text.height() as i32;

		// We'll have to render the text ourselves, computing padding
		// Fill with background color
		buf.set_style(signed_rect_intersection((x_pos, y_pos, width, height), *buf.area()), text.style);

		for (cur_y, line) in (y_pos..).zip(&text) {
			if cur_y < 0 || cur_y >= bounds.y as i32 + bounds.height as i32 { continue; }
			let alignment = text.alignment.or(line.alignment).unwrap_or_default();
			let padding: i32 = match alignment {
				Alignment::Left => 0,
				Alignment::Right => width - line.width() as i32,
				Alignment::Center => (width - line.width() as i32) / 2,
			};

			let base_style = line.style.patch(text.style);

			// this is hilariously hard to read
			for (cur_x, cur_grapheme) in ((x_pos+padding)..(x_pos+padding+line.width() as i32))
				.zip(line.iter().map(|span| span.styled_graphemes(base_style)).flatten()) {
				if cur_x < 0 || cur_x >= bounds.x as i32 + bounds.width as i32 { continue; }
				let cell = buf.cell_mut((cur_x.try_into().unwrap(), cur_y.try_into().unwrap())).unwrap();
				cell.set_style(cur_grapheme.style);
				cell.set_symbol(cur_grapheme.symbol);
			}
		}
	}
}

fn signed_rect_intersection(signed_rect: (i32, i32, i32, i32), other: Rect) -> Rect {
	let (x, y, width, height);

	x = signed_rect.0.max(other.x as i32).min((other.x + other.width) as i32) as u16;
	y = signed_rect.1.max(other.y as i32).min((other.y + other.height) as i32) as u16;
	width = signed_rect.2
		.min(signed_rect.0 + signed_rect.2 - other.x as i32)
		.min((other.x + other.width) as i32 - signed_rect.0)
		.max(0).min(other.width as i32) as u16;
	height = signed_rect.3
		.min(signed_rect.1 + signed_rect.3 - other.x as i32)
		.min((other.x + other.width) as i32 - signed_rect.1)
		.max(0).min(other.width as i32) as u16;

	Rect {
		x,
		y,
		width,
		height,
	}
}

fn main() -> std::io::Result<()> {
	let mut terminal = ratatui::init();
	let mut app = App::default();

	let input_id = app.create_node("input");
	let input_node = app.get_node_mut(input_id).unwrap();
	input_node.add_socket(Socket { name: "In Socket 1".to_owned(), input: false });
	input_node.add_socket(Socket { name: "In Socket 2".to_owned(), input: false });
	input_node.add_socket(Socket { name: "Longer In Socket 3".to_owned(), input: false });

	let output_id = app.create_node("output");
	let output_node = app.get_node_mut(output_id).unwrap();
	output_node.add_socket(Socket { name: "Out Socket 1".to_owned(), input: false });
	output_node.add_connection(input_id, 0, 0);
	output_node.set_pos(40, 0);

	let result = app.run(&mut terminal);
	ratatui::restore();

	result
}
