use std::fs;

/**
 * The general format of a recording is
 * {{ gamemode name }}_{{ index of the recording local to the gamemode }}_{{ index of the record relative to all other recordings}}
 */

const REPLAY_DIRECTORY: &str = "./replays";

pub struct RecordingName {
    pub gamemode_name: String,
    pub gamemode_index: usize,
    pub total_recording_index: usize
}

impl RecordingName {
    fn new(gamemode_name: &str) -> Self {
        let (local, global) = get_recordings()
            .fold((0, 0), |(local, global), entry| (
                    local + if entry.gamemode_name == gamemode_name { 1 } else { 0 },
                    global + 1));
        Self {
            gamemode_name: gamemode_name.to_string(),
            gamemode_index: local,
            total_recording_index: global
        }
    }

    fn from_string(s: &str) -> Self {
        let comp = s.split("_").collect::<Vec<_>>();

        Self {
            gamemode_name: comp[0].to_string(),
            gamemode_index: comp[1].parse().unwrap(),
            total_recording_index: comp[2].parse().unwrap()
        }
    }

    fn to_string(&self) -> String {
        format!("{}_{}_{}", self.gamemode_name, self.gamemode_index, self.total_recording_index)
    }

    pub fn to_filename(&self) -> String {
        format!("{}/{}.json", REPLAY_DIRECTORY, self.to_string())
    }
}

pub fn get_recording_names() -> impl Iterator<Item = String> {
    fs::read_dir(REPLAY_DIRECTORY).unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| !path.is_dir())
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
}

pub fn get_recordings() -> impl Iterator<Item = RecordingName> {
    get_recording_names()
        .map(|name| RecordingName::from_string(name.split(".").next().unwrap()))
}

pub fn get_recording_filename_for_gamemode(gamemode_name: &str) -> String {
    RecordingName::new(gamemode_name).to_filename()
}

pub fn get_sorted_recordings() -> Vec<RecordingName> {
    let mut recordings = get_recordings().collect::<Vec<_>>();
    recordings.sort_by_key(|entry| std::cmp::Reverse((entry.total_recording_index, entry.gamemode_name.clone(), entry.gamemode_index)));

    return recordings;
}