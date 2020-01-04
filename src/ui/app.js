/* jshint strict: true, esversion: 5, browser: true */

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
		load_car_types_for_selected_track: function() {
			external.invoke(JSON.stringify({ type: 'LoadCarTypesForTrack', track: $('#track-select').val() }));
		},
		set_car_types_for_selected_track: function() {
			external.invoke(JSON.stringify({ 
				type: 'WriteCarTypesForTrack', 
				track: $('#track-select').val(),
				primary: $('#primary-car-type').val(),
				secondary: $('#secondary-car-type').val(),
			}));
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
				case "CarTypesForTrack":
					Gui.set_active_car_types(msg.primary, msg.secondary);
					Gui.disable_car_type_actions();
					break;
				case "WrittenCarTypesForTrack":
					Gui.disable_car_type_actions();
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
		set_car_type_list: function(car_types) {
			var primary_select = document.getElementById('primary-car-type');

			car_types.forEach(function(element, key) {
				primary_select[key] = new Option(element.name, element.key);
			});

			var secondary_select = document.getElementById('secondary-car-type');

			car_types.forEach(function(element, key) {
				secondary_select[key] = new Option(element.name, element.key);
			});

			Action.load_car_types_for_selected_track();
		},
		set_active_car_types: function(primary, secondary) {
			$('#primary-car-type').val(primary);
			$('#secondary-car-type').val(secondary);
		},
		enable_car_type_actions: function() {
			$('#track-save-button').prop('disabled', false);
			$('#track-cancel-button').prop('disabled', false);
		},
		disable_car_type_actions: function() {
			$('#track-save-button').prop('disabled', true);
			$('#track-cancel-button').prop('disabled', true);
		},
	};
})();

$(document).ready(Gui.boot);