use crate::language::Language;
use crate::car_physics::CarPhysicsByTrack;
use crate::car_type::CarType;
use crate::data_service::DataService;
use crate::car_type::TeamPlayer;
use crate::default_data_service::DefaultDataService;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;

use crate::template;
use crate::track::Track;
use crate::image::Image;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use web_view::*;
use strum::*;

const MMV3_BIN_SIZE: u64 = 674461872;

// messages received from the GUI
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GuiRequest {
    SetLanguage {
        language: Language,
    },
    LoadTrackList,
    LoadCarTypeList,
    LoadCarDataForTrack {
        track: Track
    },
    ResetCarDataForTrack {
        track: Track
    },
    LoadCarPhysicsForCarType {
        car_type: CarType
    },
    WriteCarDataForTrack {
        track: Track,
        primary: CarType,
        secondary: CarType,
        physics: CarPhysicsByTrack
    },
}

// messages to send to the GUI
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum GuiResponse {
    TrackList {
        tracks: Vec<TrackResponseData>
    },
    CarTypeList {
        car_types: Vec<CarTypeResponseData>
    },
    CarDataForTrack {
        primary: CarType,
        default_primary: CarType,
        secondary: CarType,
        default_secondary: CarType,
        physics: CarPhysicsByTrack,
        default_physics: CarPhysicsByTrack,
    },
    CarPhysicsForCarType {
        physics: CarPhysicsByTrack
    },
    WrittenCarDataForTrack,
    ResetCarDataForTrack,
    LanguageSet {
        language: Language
    },
}

#[derive(Serialize)]
pub struct TrackResponseData {
    key: Track,
    name: String
}

#[derive(Serialize)]
pub struct CarTypeResponseData {
    key: CarType,
    name: String,
    image: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PersistentData {
    pub file_path: PathBuf,
    pub language: Language
}

pub fn spawn_gui() {
    let running = Arc::new(AtomicBool::new(true));

    set_dpi_aware();

    let html = template::build_html_template();

    let mut webview = web_view::builder()
        .title("Micro Machines V3 Car Physics Editor")
        .content(Content::Html(html))
        .size(1000, 800)
        .resizable(true)
        .debug(true)
        .user_data(PersistentData::default())
        .invoke_handler(move |mut webview, arg| {
            match serde_json::from_str::<GuiRequest>(arg) {
                Ok(GuiRequest::SetLanguage {
                    language
                }) => {
                    let mut user_data = webview.user_data_mut();

                    user_data.language = language;

                    message_dispatch(
                        &mut webview,
                        &GuiResponse::LanguageSet {
                            language
                        }
                    )
                },
                Ok(GuiRequest::LoadTrackList) => {
                    let path: PathBuf;

                    loop {
                        let path_opt = webview.dialog().open_file(
                            "Select MMv3 Binary",
                            "",
                        )?;

                        if let Some(path_val) = path_opt {
                            if path_val.extension().unwrap_or(OsStr::new("")) == "bin" {
                                let metadata = fs::metadata(&path_val);

                                if metadata.is_ok() && metadata.unwrap().len() == MMV3_BIN_SIZE {
                                    path = path_val;
                                    break;
                                }
                            }

                            webview
                            .dialog()
                            .warning("Warning", "Please choose a valid Micro Machines V3 binary file")?;
                        } else {
                            webview.exit(); // User has cancelled
                        }
                    }

                    let mut user_data = webview.user_data_mut();

                    user_data.file_path = path;
                    
                    message_dispatch(
                        &mut webview,
                        &GuiResponse::TrackList {
                            tracks: Track::iter()
                                .map(|track| TrackResponseData {
                                    key: track,
                                    name: track.get_message().unwrap().to_string()
                                }).collect()
                            }
                    )
                },
                Ok(GuiRequest::LoadCarTypeList) => message_dispatch(
                    &mut webview,
                    &GuiResponse::CarTypeList {
                        car_types: CarType::iter()
                            .map(|car_type| CarTypeResponseData {
                                key: car_type,
                                name: car_type.get_message().unwrap().to_string(),
                                image: Image::Car(car_type).get_encoded_image().get_html_src_string()
                            }).collect()
                    }
                ),
                Ok(GuiRequest::LoadCarDataForTrack {
                    track
                }) => {
                    let data_service = DataService::new(
                        &webview.user_data().file_path,
                        webview.user_data().language
                    );
                    
                    let primary = data_service
                        .read_car_type(track, TeamPlayer::First)
                        .expect("Failed to read primary car type!");
                    let secondary = data_service
                        .read_car_type(track, TeamPlayer::Second)
                        .expect("Failed to read secondary car type!");
                    let physics = data_service
                        .read_car_physics_by_track(track)
                        .expect("Failed to read car physics");
                    
                    let default_data_service = DefaultDataService::new();

                    let default_primary = default_data_service
                        .read_car_type(track, TeamPlayer::First)
                        .expect("Failed to read primary car type!");
                    let default_secondary = default_data_service
                        .read_car_type(track, TeamPlayer::Second)
                        .expect("Failed to read secondary car type!");
                    let default_physics = default_data_service
                        .read_car_physics_by_track(track)
                        .expect("Failed to read car physics");

                    message_dispatch(
                        &mut webview,
                        &GuiResponse::CarDataForTrack {
                            primary,
                            default_primary,
                            secondary,
                            default_secondary,
                            physics,
                            default_physics
                        }
                    );
                },
                Ok(GuiRequest::ResetCarDataForTrack {
                    track
                }) => {
                    let default_data_service = DefaultDataService::new();
                    
                    let default_primary = default_data_service
                        .read_car_type(track, TeamPlayer::First)
                        .expect("Failed to read primary car type!");
                    let default_secondary = default_data_service
                        .read_car_type(track, TeamPlayer::Second)
                        .expect("Failed to read secondary car type!");
                    let default_physics = default_data_service
                        .read_car_physics_by_track(track)
                        .expect("Failed to read car physics");

                    let data_service = DataService::new(
                        &webview.user_data().file_path,
                        webview.user_data().language
                    );

                    data_service
                        .write_car_type(track, TeamPlayer::First, default_primary)
                        .expect("Failed to write primary car type!");
                    data_service
                        .write_car_type(track, TeamPlayer::Second, default_secondary)
                        .expect("Failed to write secondary car type!");
                    data_service
                        .write_car_physics_by_track(track, default_physics)
                        .expect("Failed to write car physics for track!");

                    message_dispatch(
                        &mut webview,
                        &GuiResponse::ResetCarDataForTrack
                    );
                },
                Ok(GuiRequest::LoadCarPhysicsForCarType {
                    car_type
                }) => {
                    let data_service = DataService::new(
                        &webview.user_data().file_path,
                        webview.user_data().language
                    );

                    let physics: CarPhysicsByTrack = data_service
                        .read_car_physics_by_car_type(car_type)
                        .expect("Failed to read car physics")
                        .into();

                    message_dispatch(
                        &mut webview,
                        &GuiResponse::CarPhysicsForCarType {
                            physics,
                        }
                    );
                },
                Ok(GuiRequest::WriteCarDataForTrack {
                    track,
                    primary,
                    secondary,
                    physics
                }) => {
                    let data_service = DataService::new(
                        &webview.user_data().file_path,
                        webview.user_data().language
                    );

                    data_service
                        .write_car_type(track, TeamPlayer::First, primary)
                        .expect("Failed to write primary car type!");
                    data_service
                        .write_car_type(track, TeamPlayer::Second, secondary)
                        .expect("Failed to write secondary car type!");
                    data_service
                        .write_car_physics_by_track(track, physics)
                        .expect("Failed to write car physics for track!");

                    message_dispatch(
                        &mut webview,
                        &GuiResponse::WrittenCarDataForTrack
                    );
                }
                ,
                _ => ()            
            }

            Ok(())
        })
        .build()
        .expect("WebView");

    while running.load(Ordering::SeqCst) {
        match webview.step() {
            Some(Ok(_)) => (),
            Some(e) => {
                eprintln!("Error: {:?}", e);
            }
            None => {
                break;
            }
        }
    }

    webview.into_inner();
}

fn message_dispatch<T>(wv: &mut web_view::WebView<'_, T>, msg: &GuiResponse) {
    let js = format!(
        "Response.dispatch({})",
        serde_json::to_string(msg).expect("serialize")
    );

    wv.eval(&js).ok();
}

fn set_dpi_aware() {
    use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_SYSTEM_DPI_AWARE};

    unsafe { SetProcessDpiAwareness(PROCESS_SYSTEM_DPI_AWARE) };
}
