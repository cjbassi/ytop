use num_rational::Ratio;

use crate::app::Widgets;

pub trait UpdatableWidget {
	fn update(&mut self);
	fn get_update_interval(&self) -> Ratio<u64>;
}

pub fn update_widgets(widgets: &mut Widgets, seconds: Ratio<u64>) {
	let mut widgets_to_update: Vec<&mut (dyn UpdatableWidget)> =
		vec![&mut widgets.cpu, &mut widgets.mem, &mut widgets.proc];

	if let (Some(disk), Some(net), Some(temp)) = (
		widgets.disk.as_mut(),
		widgets.net.as_mut(),
		widgets.temp.as_mut(),
	) {
		widgets_to_update.push(disk);
		widgets_to_update.push(net);
		widgets_to_update.push(temp);
	}

	if let Some(battery) = widgets.battery.as_mut() {
		widgets_to_update.push(battery);
	}

	for widget in widgets_to_update {
		if seconds % widget.get_update_interval() == Ratio::from_integer(0) {
			widget.update();
		}
	}
}
