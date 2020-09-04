#[macro_use]
extern crate vst;
extern crate vst_gui;

use std::sync::{Arc, Mutex};

use vst::api;
use vst::api::Supported;
use vst::buffer::AudioBuffer;
use vst::event::{Event, MidiEvent};
use vst::editor::Editor;
use vst::plugin::{CanDo, Category, Plugin, Info, HostCallback};
use vst::host::Host;


fn inline_script(s: &str) -> String {
	format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn inline_style(s: &str) -> String {
	format!(r#"<style type="text/css">{}</style>"#, s)
}


fn get_html() -> String {
	format!(r#"
		<!doctype html>
		<html>
		<head>
		<meta charset="utf-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">
		{style}
		</head>
		<body>
		<div id="app"></div>
		<!--[if lt IE 9]>
		<div class="ie-upgrade-container">
		<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
		<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
		</div>
		<![endif]-->
		<!--[if gte IE 9 | !IE ]> <!-->
		{scripts}
		<![endif]-->
		</body>
		</html>
		"#,
		style = inline_style(include_str!("style.css")),
		scripts = inline_script(include_str!("bundle.js"))
	)
}

#[derive(Default)]
struct FLChen {
	pub mode: u8,
	pub time: f64
}

fn create_javascript_callback(fl_chen: Arc<Mutex<FLChen>>) -> vst_gui::JavascriptCallback
{
	Box::new(move |message: String| {
		let mut tokens = message.split_whitespace();

		let command = tokens.next().unwrap_or("");

		let locked_fl_chen = fl_chen.lock().unwrap();

		match command {
			"getMode" => {
				let mode = &locked_fl_chen.mode;
				return mode.to_string();
			},
			"getTime" => {
				let time = &locked_fl_chen.time;
				return time.to_string();
			},
			_ => {}
		}

		String::new()
	})
}

#[derive(Default)]
struct VstFLChen {
	sample_rate: f32,
	fl_chen: Arc<Mutex<FLChen>>,
	host: HostCallback
}

impl VstFLChen {
	fn process_midi_event(&mut self, data: [u8; 3]) {
		let mut locked_fl_chen = self.fl_chen.lock().unwrap();
		match data[1] {
			69 => locked_fl_chen.mode = 0,
			71 => locked_fl_chen.mode = 1,
			72 => locked_fl_chen.mode = 2,
			74 => locked_fl_chen.mode = 3,
			76 => locked_fl_chen.mode = 4,
			77 => locked_fl_chen.mode = 5,
			79 => locked_fl_chen.mode = 6,
			81 => locked_fl_chen.mode = 7,
			83 => locked_fl_chen.mode = 8,
			_ => (),
		}
	}
}

impl Plugin for VstFLChen {

	fn new(host: HostCallback) -> Self {
		let fl_chen = Arc::new(Mutex::new(
			FLChen {
				mode: 0,
				time: 0.
			}
		));

		VstFLChen {
			sample_rate: 44100.,
			fl_chen: fl_chen.clone(),
			host: host
		}
	}

	fn get_info(&self) -> Info {
		Info {
			name: "FL-Chen".to_string(),
			vendor: "Wehiroi".to_string(),
			unique_id: 9614,
			category: Category::Synth,
			inputs: 2,
			outputs: 2,
			parameters: 0,
			..Info::default()
		}
	}

	fn process_events(&mut self, events: &api::Events) {
		for e in events.events() {
			match e {
				Event::Midi(MidiEvent { data, .. }) => self.process_midi_event(data),
				_ => ()
			}
		}
	}

	fn process(&mut self, _buffer: &mut AudioBuffer<f32>) {
		let mut locked_fl_chen = self.fl_chen.lock().unwrap();
		let time_info = self.host.get_time_info(2147483647).unwrap();
		locked_fl_chen.time = time_info.ppq_pos;
	}

	fn set_sample_rate(&mut self, sample_rate: f32) {
		self.sample_rate = sample_rate as f32;
	}

	fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
		let gui = vst_gui::new_plugin_gui(
			String::from(get_html()),
			create_javascript_callback(self.fl_chen.clone()),
			Some((460, 600)));
		Some(Box::new(gui))
	}

	fn can_do(&self, can_do: CanDo) -> Supported {
		match can_do {
			CanDo::ReceiveMidiEvent => Supported::Yes,
			_ => Supported::Maybe,
		}
	}
}

plugin_main!(VstFLChen);
