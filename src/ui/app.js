/* jshint strict: true, esversion: 5, browser: true */

var car_types = {};

var Util = (function() {
	"use strict";

	return {};
});

// Actions call back into Rust
var Action = (function() {
	"use strict";

	return {
		load_track_list: function() {
			external.invoke(JSON.stringify({ type: 'LoadTrackList' }));
		},
		load_car_type_list: function() {
			external.invoke(JSON.stringify({ type: 'LoadCarTypeList' }));
		},
		load_car_data_for_selected_track: function() {
			external.invoke(JSON.stringify({ type: 'LoadCarDataForTrack', track: $('#track-select').val() }));
		},
		load_car_physics_for_car_type: function() {
			external.invoke(JSON.stringify({ type: 'LoadCarPhysicsForCarType', car_type: $('#primary-car-type').val() }));
		},
		set_car_data_for_selected_track: function() {
			var mainForm = $('#main-form');

			if(! mainForm[0].checkValidity()) {
				return false;
			}

			var physics = {
				acceleration: parseInt($('#acceleration').val()),
				top_speed: parseInt($('#top-speed').val()),
				grip: parseInt($('#grip').val()),
				collision_impact: parseInt($('#collision-impact').val()),
				turning: parseInt($('#turning').val()),
				sliding_friction: parseInt($('#sliding-friction').val()),
			}

			external.invoke(JSON.stringify({ 
				type: 'WriteCarDataForTrack', 
				track: $('#track-select').val(),
				primary: $('#primary-car-type').val(),
				secondary: $('#secondary-car-type').val(),
				physics: physics
			}));

			return false;
		}
	};
})();

// Responses come from Rust
var Response = (function() {
	"use strict";

	return {
		dispatch: function(msg) {
			switch(msg.type) {
				case "TrackList":
					Gui.set_track_list(msg.tracks);
					break;
				case "CarTypeList":
					Gui.set_car_type_list(msg.car_types);
					break;
				case "CarDataForTrack":
					Gui.set_car_data(msg.primary, msg.secondary, msg.physics);
					Action.load_car_physics_for_car_type();
					Gui.disable_car_data_actions();
					break;
				case "CarPhysicsForCarType":
					Gui.set_car_physics_by_car_type(msg.physics);
					break;
				case "WrittenCarDataForTrack":
					Gui.disable_car_data_actions();
					break;
			}
		}
	};
})();

// Anything poking the GUI lives here
var Gui = (function() {
	"use strict";

	return {
		boot: function() {
			Action.load_track_list();
		},
		set_track_list: function(tracks) {
			var select = document.getElementById('track-select');

			tracks.forEach(function(element, key) {
				select[key] = new Option(element.name, element.key);
			});

			Action.load_car_type_list();
		},
		set_car_type_list: function(car_type_list) {
			var primary_select = document.getElementById('primary-car-type');

			car_type_list.forEach(function(element, key) {
				primary_select[key] = new Option(element.name, element.key);
			});

			var secondary_select = document.getElementById('secondary-car-type');

			car_type_list.forEach(function(element, key) {
				secondary_select[key] = new Option(element.name, element.key);
			});

			car_types = car_type_list;

			Action.load_car_data_for_selected_track();
		},
		set_car_data: function(primary, secondary, physics, physics_car_type) {
			$('#primary-car-type').val(primary);
			$('#secondary-car-type').val(secondary);
			$('#acceleration').val(physics.acceleration);
			$('#top-speed').val(physics.top_speed);
			$('#grip').val(physics.grip);
			$('#collision-impact').val(physics.collision_impact);
			$('#turning').val(physics.turning);
			$('#sliding-friction').val(physics.sliding_friction);

			Gui.set_active_car_type_images();
		},
		set_car_physics_by_car_type: function(physics) {
			$('#acceleration-car-type').text(physics.acceleration);
			$('#top-speed-car-type').text(physics.top_speed);
			$('#grip-car-type').text(physics.grip);
			$('#collision-impact-car-type').text(physics.collision_impact);
			$('#turning-car-type').text(physics.turning);
			$('#sliding-friction-car-type').text(physics.sliding_friction);
		},
		set_active_car_type_images: function() {
			let primary = $('#primary-car-type').val();
			let secondary = $('#secondary-car-type').val();
			$('#primary-car-image').prop('src', car_types.find((car_type) => car_type.key === primary).image);
			$('#secondary-car-image').prop('src', car_types.find((car_type) => car_type.key === secondary).image);
		},
		change_of_car_type: function() {
			Gui.set_active_car_type_images();
			Gui.enable_car_data_actions();
			Action.load_car_physics_for_car_type($('#primary-car-type').val());
		},
		change_of_car_physics: function() {
			Gui.enable_car_data_actions();
		},
		enable_car_data_actions: function() {
			$('#track-save-button').prop('disabled', false);
			$('#track-cancel-button').prop('disabled', false);
		},
		disable_car_data_actions: function() {
			$('#track-save-button').prop('disabled', true);
			$('#track-cancel-button').prop('disabled', true);
		},
	};
})();

$(document).ready(Gui.boot);