use std::collections::HashMap;
use std::ops::Deref;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use kronark_node_parser::kronarknode::{
	Node,
	instance::Instance,
	nodes::NodeEntry,
	roots::Roots,
	types::TypeEntry,
};

// Take ownership of a `Node` and parse out its contents
// The data will be taken out of the node and restructured to make rendering easier
// Create a HashMap allowing access of instances by ID, types and node types by string
// Input Root always goes in the far left, Output Root always in the far right
//
// Instances should be separated into columns based on their connection depth
// Connection depth is defined to be the max of the connection depths of all instances connecting
// TO the instance in question, plus one. If an instance has no connections on its input side, it
// has a connection depth of zero. This means we'll be ignoring the stored x values of the instance
//
// The vertical placement of an instance is compressed with all instances in its column, with a
// padding of one. The order of vertical placement will respect the y values stored in the
// instances, but exact positioning will not
//
// Padding between columns is based on how many incoming connections the right column has plus how
// many outgoing connections the previous column has that do NOT link to the right column
// Columns will have a single column of space for each outgoing connection of the previous column,
// with one column of padding on both sides
//
// The intention with this is to draw connections horizontally outwards until they have their own
// unique column to bend, then it will bend up or down as needed to get to the row of its
// connection if the connection exists in the next column of instances, or to clear the bottom of
// the next column of instances if it does not connect immediately to the next depth.
// This requires at minimum one column of spacing per connected input of the right instance column,
// plus an additional column of spacing for each output that needs to leave the region rather than
// connecting to the right column
//
// When a connection bends back horizontally, it's possible that two lines may overlap if we
// compress the vertical space as much as possible, as shown below:
// ********************************
// ───┐ ┌────
//    │ │
// ─────┘────
// ********************************
// In this situation, the line starting at the top left was drawn first, extended out to its target
// column, then went back horizontal at the target row. The bottom line did the same and damaged
// the drawing of the top line. I haven't thought up a good way to avoid this, so the simple method
// which will get us close to a functional renderer as quickly as possible is to simply alternate
// the socket positions from column to column, so that inputs and outputs never lie on the same
// row. Additionally, we will have to detect intersections to replace them with the character '┼'
// (or we ignore that because it's not that important and we can still make sense of it)
//
// I am *very* open to ideas for this. Remember, we're not trying to make it pretty, just good
// enough so we can document the nodes. Pretty comes later.
//
// Additionally, out of necessity I believe it's a good idea to allow scrolling of the view window
// with arrow keys, to browse larger node graphs. `ratatui` does not inherently support having its
// widgets overdraw, but we can implement our own widgets and draw to the buffer provided,
// performing our own overdraw culling. See the video and main.rs file sent in the Kronark Discord
// under the forum thread for this project. I apologize in advance for the shitty code in that
// file, it was put together as hastily as I could to get a demonstration.
// Alternatively, an idea I had while writing this, we could instead only scroll by column and not
// worry about culling overdraw. We generate a simple widget for each instance, do some simple
// calculations to determine the column widths, then render only as many columns would 100% fit on
// screen. Pressing right arrow would shift the leftmost visible column over once. Lines connecting
// to offscreen instances will draw as much of their route as they can, then terminate in an angle
// bracket indicating they go offscreen. This might be simpler. Same logic can be applied to
// vertical scrolling, instead you go by instance within a column.
//
// I've tried to outline what the structure of this renderer could look like below, but this is
// certainly not final. If someone begins to implement this or components of this, do let me know
// so we can coordinate our work and discuss the structure of this.

// Thin wrapper for type-safety
#[derive(PartialEq, Eq, Hash)]
struct InstanceID(usize);
impl Deref for InstanceID {
	type Target = usize;
	fn deref(&self) -> &Self::Target { &self.0 }
}

// TODO: We need a way to store the padding. Should it be here or elsewhere?
struct Column {
	instances: Vec<InstanceID>,
	width: i32, // Max width of any instance in the column, signed to play nicely with shifts
}

struct NodeDefRenderer {
	roots: Roots,
	// We aren't guaranteed to have consecutive instance IDs, so a `HashMap` it is
	instance_table: HashMap<InstanceID, Instance>,
	node_table: Vec<NodeEntry>,
	type_table: Vec<TypeEntry>,

	instance_layout: Vec<Column>,
	x_shift: i32,
	y_shift: i32,
}
impl NodeDefRenderer {
	fn from_node(node: Node) -> Self {
		match node {
			Node::V1(node_def) => {
				let roots = node_def.roots;
				let node_table = node_def.nodes;
				let type_table = node_def.types;

				let mut instance_table = HashMap::new();
				for instance in node_def.instances {
					instance_table.insert(InstanceID(instance.key), instance);
				}

				// TODO: Here we would parse the data and generate the columns
				let mut instance_layout = vec!();

				NodeDefRenderer {
					roots,
					instance_table,
					node_table,
					type_table,
					instance_layout,
					x_shift: 0,
					y_shift: 0,
				}
			},
			#[allow(unreachable_patterns)]
			_ => panic!("unsupported version"),
		}
	}
}
impl Widget for &NodeDefRenderer {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		todo!()
	}
}

fn run(renderer: NodeDefRenderer) {
	todo!();
}

// This will setup the terminal and the renderer struct, then enter another function to loop
// drawing and event processing
pub fn enter_node_view(node: Node) -> std::io::Result<()> {
	todo!()
}
