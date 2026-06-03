CREATE TABLE tracks 
(
  id INTEGER PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  location TEXT NOT NULL,
  duration_seconds FLOAT NOT NULL
);

CREATE TABLE playlists
(
  id INTEGER PRIMARY KEY,
  name VARCHAR(100) NOT NULL
);

CREATE TABLE track_playlists
(
  track_id INTEGER REFERENCES track(id),
  playlist_id INTEGER REFERENCES playlists(id),
  position INTEGER NOT NULL,
  PRIMARY KEY(track_id, playlist_id)
);
