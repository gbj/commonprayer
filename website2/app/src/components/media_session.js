export function set_media_metadata(title, artist, album) {
	if ('mediaSession' in navigator) {
		navigator.mediaSession.metadata = new MediaMetadata({
			title,
			artist,
			album
		});
	}
}

export function set_media_session_action_handler(action, handler) {
	if ('mediaSession' in navigator) {
		navigator.mediaSession.setActionHandler(action, handler);
	}
}

export function clear_position_state() {
	if ('mediaSession' in navigator) {
		navigator.mediaSession.setPositionState();
	}
}

export function set_position_state(duration, playbackRate, position) {
	if ('mediaSession' in navigator) {
		navigator.mediaSession.setPositionState({duration, playbackRate, position});
	}
}

export function set_playback_state(state) {
	if ('mediaSession' in navigator) {
		navigator.mediaSession.playbackState = state;
	}
}
